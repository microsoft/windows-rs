#[cfg(feature = "implement_exclusive")]
pub trait ICompositionConditionalValueImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetValue(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionConditionalValue";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionConditionalValueVtbl {
    pub const fn new<Impl: ICompositionConditionalValueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionConditionalValueVtbl {
        unsafe extern "system" fn Condition<Impl: ICompositionConditionalValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: ICompositionConditionalValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ICompositionConditionalValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ICompositionConditionalValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionConditionalValue>, base.5, Condition::<Impl, OFFSET>, SetCondition::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionConditionalValueStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<CompositionConditionalValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionConditionalValueStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionConditionalValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionConditionalValueStaticsVtbl {
    pub const fn new<Impl: ICompositionConditionalValueStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionConditionalValueStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ICompositionConditionalValueStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionConditionalValueStatics>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait ICompositionInteractionSourceImpl: Sized {}
impl ::windows::core::RuntimeName for ICompositionInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSource";
}
impl ICompositionInteractionSourceVtbl {
    pub const fn new<Impl: ICompositionInteractionSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionInteractionSourceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionInteractionSource>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionInteractionSourceCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, value: &::core::option::Option<ICompositionInteractionSource>) -> ::windows::core::Result<()>;
    fn Remove(&self, value: &::core::option::Option<ICompositionInteractionSource>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionInteractionSourceCollection {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSourceCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionInteractionSourceCollectionVtbl {
    pub const fn new<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionInteractionSourceCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Add(&*(&value as *const <ICompositionInteractionSource as ::windows::core::Abi>::Abi as *const <ICompositionInteractionSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Remove(&*(&value as *const <ICompositionInteractionSource as ::windows::core::Abi>::Abi as *const <ICompositionInteractionSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionInteractionSourceCollection>, base.5, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, RemoveAll::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionSourceConfigurationImpl: Sized {
    fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode>;
    fn SetPositionXSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
    fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode>;
    fn SetPositionYSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
    fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode>;
    fn SetScaleSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionSourceConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionSourceConfigurationVtbl {
    pub const fn new<Impl: IInteractionSourceConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionSourceConfigurationVtbl {
        unsafe extern "system" fn PositionXSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionXSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionXSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionXSourceMode(value).into()
        }
        unsafe extern "system" fn PositionYSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionYSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionYSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionYSourceMode(value).into()
        }
        unsafe extern "system" fn ScaleSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleSourceMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionSourceConfiguration>, base.5, PositionXSourceMode::<Impl, OFFSET>, SetPositionXSourceMode::<Impl, OFFSET>, PositionYSourceMode::<Impl, OFFSET>, SetPositionYSourceMode::<Impl, OFFSET>, ScaleSourceMode::<Impl, OFFSET>, SetScaleSourceMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerImpl: Sized {
    fn InteractionSources(&self) -> ::windows::core::Result<CompositionInteractionSourceCollection>;
    fn IsPositionRoundingSuggested(&self) -> ::windows::core::Result<bool>;
    fn MaxPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetMaxPosition(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn MaxScale(&self) -> ::windows::core::Result<f32>;
    fn SetMaxScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn MinPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetMinPosition(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn MinScale(&self) -> ::windows::core::Result<f32>;
    fn SetMinScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn NaturalRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn NaturalRestingScale(&self) -> ::windows::core::Result<f32>;
    fn Owner(&self) -> ::windows::core::Result<IInteractionTrackerOwner>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn PositionInertiaDecayRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SetPositionInertiaDecayRate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Scale(&self) -> ::windows::core::Result<f32>;
    fn ScaleInertiaDecayRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn SetScaleInertiaDecayRate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32>;
    fn AdjustPositionXIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()>;
    fn AdjustPositionYIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()>;
    fn ConfigurePositionXInertiaModifiers(&self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>) -> ::windows::core::Result<()>;
    fn ConfigurePositionYInertiaModifiers(&self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>) -> ::windows::core::Result<()>;
    fn ConfigureScaleInertiaModifiers(&self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>) -> ::windows::core::Result<()>;
    fn TryUpdatePosition(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionBy(&self, amount: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionWithAnimation(&self, animation: &::core::option::Option<super::CompositionAnimation>) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionWithAdditionalVelocity(&self, velocityinpixelspersecond: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdateScale(&self, value: f32, centerpoint: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdateScaleWithAnimation(&self, animation: &::core::option::Option<super::CompositionAnimation>, centerpoint: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdateScaleWithAdditionalVelocity(&self, velocityinpercentpersecond: f32, centerpoint: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVtbl {
    pub const fn new<Impl: IInteractionTrackerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerVtbl {
        unsafe extern "system" fn InteractionSources<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InteractionSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPositionRoundingSuggested<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPositionRoundingSuggested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPosition<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPosition<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxPosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxScale<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxScale<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxScale(value).into()
        }
        unsafe extern "system" fn MinPosition<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPosition<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinPosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinScale<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinScale<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinScale(value).into()
        }
        unsafe extern "system" fn NaturalRestingPosition<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalRestingPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalRestingScale<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalRestingScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Owner<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionInertiaDecayRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionInertiaDecayRate(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionVelocityInPixelsPerSecond<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionVelocityInPixelsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scale<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleInertiaDecayRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleInertiaDecayRate(&*(&value as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScaleVelocityInPercentPerSecond<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleVelocityInPercentPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustPositionXIfGreaterThanThreshold<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AdjustPositionXIfGreaterThanThreshold(adjustment, positionthreshold).into()
        }
        unsafe extern "system" fn AdjustPositionYIfGreaterThanThreshold<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AdjustPositionYIfGreaterThanThreshold(adjustment, positionthreshold).into()
        }
        unsafe extern "system" fn ConfigurePositionXInertiaModifiers<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigurePositionXInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigurePositionYInertiaModifiers<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigurePositionYInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureScaleInertiaModifiers<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureScaleInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryUpdatePosition<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdatePosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionBy<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionBy(&*(&amount as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionWithAnimation<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithAnimation(&*(&animation as *const <super::CompositionAnimation as ::windows::core::Abi>::Abi as *const <super::CompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionWithAdditionalVelocity<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithAdditionalVelocity(&*(&velocityinpixelspersecond as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateScale<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdateScale(value, &*(&centerpoint as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateScaleWithAnimation<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdateScaleWithAnimation(&*(&animation as *const <super::CompositionAnimation as ::windows::core::Abi>::Abi as *const <super::CompositionAnimation as ::windows::core::DefaultType>::DefaultType), &*(&centerpoint as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateScaleWithAdditionalVelocity<Impl: IInteractionTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdateScaleWithAdditionalVelocity(velocityinpercentpersecond, &*(&centerpoint as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IInteractionTracker>,
            base.5,
            InteractionSources::<Impl, OFFSET>,
            IsPositionRoundingSuggested::<Impl, OFFSET>,
            MaxPosition::<Impl, OFFSET>,
            SetMaxPosition::<Impl, OFFSET>,
            MaxScale::<Impl, OFFSET>,
            SetMaxScale::<Impl, OFFSET>,
            MinPosition::<Impl, OFFSET>,
            SetMinPosition::<Impl, OFFSET>,
            MinScale::<Impl, OFFSET>,
            SetMinScale::<Impl, OFFSET>,
            NaturalRestingPosition::<Impl, OFFSET>,
            NaturalRestingScale::<Impl, OFFSET>,
            Owner::<Impl, OFFSET>,
            Position::<Impl, OFFSET>,
            PositionInertiaDecayRate::<Impl, OFFSET>,
            SetPositionInertiaDecayRate::<Impl, OFFSET>,
            PositionVelocityInPixelsPerSecond::<Impl, OFFSET>,
            Scale::<Impl, OFFSET>,
            ScaleInertiaDecayRate::<Impl, OFFSET>,
            SetScaleInertiaDecayRate::<Impl, OFFSET>,
            ScaleVelocityInPercentPerSecond::<Impl, OFFSET>,
            AdjustPositionXIfGreaterThanThreshold::<Impl, OFFSET>,
            AdjustPositionYIfGreaterThanThreshold::<Impl, OFFSET>,
            ConfigurePositionXInertiaModifiers::<Impl, OFFSET>,
            ConfigurePositionYInertiaModifiers::<Impl, OFFSET>,
            ConfigureScaleInertiaModifiers::<Impl, OFFSET>,
            TryUpdatePosition::<Impl, OFFSET>,
            TryUpdatePositionBy::<Impl, OFFSET>,
            TryUpdatePositionWithAnimation::<Impl, OFFSET>,
            TryUpdatePositionWithAdditionalVelocity::<Impl, OFFSET>,
            TryUpdateScale::<Impl, OFFSET>,
            TryUpdateScaleWithAnimation::<Impl, OFFSET>,
            TryUpdateScaleWithAdditionalVelocity::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTracker2Impl: Sized {
    fn ConfigureCenterPointXInertiaModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureCenterPointYInertiaModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTracker2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTracker2Vtbl {
    pub const fn new<Impl: IInteractionTracker2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTracker2Vtbl {
        unsafe extern "system" fn ConfigureCenterPointXInertiaModifiers<Impl: IInteractionTracker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointXInertiaModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureCenterPointYInertiaModifiers<Impl: IInteractionTracker2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointYInertiaModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTracker2>, base.5, ConfigureCenterPointXInertiaModifiers::<Impl, OFFSET>, ConfigureCenterPointYInertiaModifiers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTracker3Impl: Sized {
    fn ConfigureVector2PositionInertiaModifiers(&self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTracker3 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker3";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTracker3Vtbl {
    pub const fn new<Impl: IInteractionTracker3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTracker3Vtbl {
        unsafe extern "system" fn ConfigureVector2PositionInertiaModifiers<Impl: IInteractionTracker3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureVector2PositionInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTracker3>, base.5, ConfigureVector2PositionInertiaModifiers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTracker4Impl: Sized {
    fn TryUpdatePositionWithOption(&self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionByWithOption(&self, amount: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTracker4 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker4";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTracker4Vtbl {
    pub const fn new<Impl: IInteractionTracker4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTracker4Vtbl {
        unsafe extern "system" fn TryUpdatePositionWithOption<Impl: IInteractionTracker4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithOption(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionByWithOption<Impl: IInteractionTracker4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionByWithOption(&*(&amount as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInertiaFromImpulse<Impl: IInteractionTracker4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInertiaFromImpulse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTracker4>, base.5, TryUpdatePositionWithOption::<Impl, OFFSET>, TryUpdatePositionByWithOption::<Impl, OFFSET>, IsInertiaFromImpulse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTracker5Impl: Sized {
    fn TryUpdatePositionWithOption(&self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTracker5 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker5";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTracker5Vtbl {
    pub const fn new<Impl: IInteractionTracker5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTracker5Vtbl {
        unsafe extern "system" fn TryUpdatePositionWithOption<Impl: IInteractionTracker5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithOption(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), option, posupdateoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTracker5>, base.5, TryUpdatePositionWithOption::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerCustomAnimationStateEnteredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerCustomAnimationStateEnteredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerCustomAnimationStateEnteredArgsVtbl {
    pub const fn new<Impl: IInteractionTrackerCustomAnimationStateEnteredArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerCustomAnimationStateEnteredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerCustomAnimationStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerCustomAnimationStateEnteredArgs>, base.5, RequestId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerCustomAnimationStateEnteredArgs2Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerCustomAnimationStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerCustomAnimationStateEnteredArgs2Vtbl {
    pub const fn new<Impl: IInteractionTrackerCustomAnimationStateEnteredArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerCustomAnimationStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerCustomAnimationStateEnteredArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerCustomAnimationStateEnteredArgs2>, base.5, IsFromBinding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerIdleStateEnteredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerIdleStateEnteredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerIdleStateEnteredArgsVtbl {
    pub const fn new<Impl: IInteractionTrackerIdleStateEnteredArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerIdleStateEnteredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerIdleStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerIdleStateEnteredArgs>, base.5, RequestId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerIdleStateEnteredArgs2Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerIdleStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerIdleStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerIdleStateEnteredArgs2Vtbl {
    pub const fn new<Impl: IInteractionTrackerIdleStateEnteredArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerIdleStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerIdleStateEnteredArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerIdleStateEnteredArgs2>, base.5, IsFromBinding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaModifierImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaModifier";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaModifierVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaModifierImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaModifierVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaModifier>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaModifierFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaModifierFactory {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaModifierFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaModifierFactoryVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaModifierFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaModifierFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaModifierFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaMotionImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn Motion(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetMotion(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaMotion";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaMotionVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaMotionVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Motion<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Motion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMotion<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMotion(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaMotion>, base.5, Condition::<Impl, OFFSET>, SetCondition::<Impl, OFFSET>, Motion::<Impl, OFFSET>, SetMotion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaMotionStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaMotion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaMotionStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaMotionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaMotionStaticsVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaMotionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaMotionStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaMotionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaMotionStatics>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaNaturalMotionImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn NaturalMotion(&self) -> ::windows::core::Result<super::ScalarNaturalMotionAnimation>;
    fn SetNaturalMotion(&self, value: &::core::option::Option<super::ScalarNaturalMotionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaNaturalMotion";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaNaturalMotionVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaNaturalMotionVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalMotion<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalMotion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNaturalMotion<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNaturalMotion(&*(&value as *const <super::ScalarNaturalMotionAnimation as ::windows::core::Abi>::Abi as *const <super::ScalarNaturalMotionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaNaturalMotion>, base.5, Condition::<Impl, OFFSET>, SetCondition::<Impl, OFFSET>, NaturalMotion::<Impl, OFFSET>, SetNaturalMotion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaNaturalMotionStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaNaturalMotion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaNaturalMotionStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaNaturalMotionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaNaturalMotionStaticsVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaNaturalMotionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaNaturalMotionStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaNaturalMotionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaNaturalMotionStatics>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaRestingValueImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn RestingValue(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetRestingValue(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaRestingValue";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaRestingValueVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaRestingValueVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestingValue<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestingValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestingValue<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRestingValue(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaRestingValue>, base.5, Condition::<Impl, OFFSET>, SetCondition::<Impl, OFFSET>, RestingValue::<Impl, OFFSET>, SetRestingValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaRestingValueStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaRestingValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaRestingValueStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaRestingValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaRestingValueStaticsVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaRestingValueStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaRestingValueStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaRestingValueStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaRestingValueStatics>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaStateEnteredArgsImpl: Sized {
    fn ModifiedRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn ModifiedRestingScale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn NaturalRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn NaturalRestingScale(&self) -> ::windows::core::Result<f32>;
    fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaStateEnteredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaStateEnteredArgsVtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaStateEnteredArgsVtbl {
        unsafe extern "system" fn ModifiedRestingPosition<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifiedRestingPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedRestingScale<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifiedRestingScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalRestingPosition<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalRestingPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalRestingScale<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalRestingScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionVelocityInPixelsPerSecond<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionVelocityInPixelsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleVelocityInPercentPerSecond<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleVelocityInPercentPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaStateEnteredArgs>, base.5, ModifiedRestingPosition::<Impl, OFFSET>, ModifiedRestingScale::<Impl, OFFSET>, NaturalRestingPosition::<Impl, OFFSET>, NaturalRestingScale::<Impl, OFFSET>, PositionVelocityInPixelsPerSecond::<Impl, OFFSET>, RequestId::<Impl, OFFSET>, ScaleVelocityInPercentPerSecond::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaStateEnteredArgs2Impl: Sized {
    fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaStateEnteredArgs2Vtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaStateEnteredArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsInertiaFromImpulse<Impl: IInteractionTrackerInertiaStateEnteredArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInertiaFromImpulse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaStateEnteredArgs2>, base.5, IsInertiaFromImpulse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaStateEnteredArgs3Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaStateEnteredArgs3 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaStateEnteredArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaStateEnteredArgs3Vtbl {
    pub const fn new<Impl: IInteractionTrackerInertiaStateEnteredArgs3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInertiaStateEnteredArgs3Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerInertiaStateEnteredArgs3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInertiaStateEnteredArgs3>, base.5, IsFromBinding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInteractingStateEnteredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInteractingStateEnteredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInteractingStateEnteredArgsVtbl {
    pub const fn new<Impl: IInteractionTrackerInteractingStateEnteredArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInteractingStateEnteredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerInteractingStateEnteredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInteractingStateEnteredArgs>, base.5, RequestId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInteractingStateEnteredArgs2Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInteractingStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInteractingStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInteractingStateEnteredArgs2Vtbl {
    pub const fn new<Impl: IInteractionTrackerInteractingStateEnteredArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerInteractingStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerInteractingStateEnteredArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerInteractingStateEnteredArgs2>, base.5, IsFromBinding::<Impl, OFFSET>)
    }
}
pub trait IInteractionTrackerOwnerImpl: Sized {
    fn CustomAnimationStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerCustomAnimationStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn IdleStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerIdleStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InertiaStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInertiaStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InteractingStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInteractingStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn RequestIgnored(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerRequestIgnoredArgs>) -> ::windows::core::Result<()>;
    fn ValuesChanged(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerValuesChangedArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInteractionTrackerOwner {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerOwner";
}
impl IInteractionTrackerOwnerVtbl {
    pub const fn new<Impl: IInteractionTrackerOwnerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerOwnerVtbl {
        unsafe extern "system" fn CustomAnimationStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CustomAnimationStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IdleStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).IdleStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerIdleStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerIdleStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InertiaStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InertiaStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerInertiaStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerInertiaStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InteractingStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InteractingStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerInteractingStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerInteractingStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestIgnored<Impl: IInteractionTrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RequestIgnored(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerRequestIgnoredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerRequestIgnoredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ValuesChanged<Impl: IInteractionTrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ValuesChanged(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerValuesChangedArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerValuesChangedArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerOwner>, base.5, CustomAnimationStateEntered::<Impl, OFFSET>, IdleStateEntered::<Impl, OFFSET>, InertiaStateEntered::<Impl, OFFSET>, InteractingStateEntered::<Impl, OFFSET>, RequestIgnored::<Impl, OFFSET>, ValuesChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerRequestIgnoredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerRequestIgnoredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerRequestIgnoredArgsVtbl {
    pub const fn new<Impl: IInteractionTrackerRequestIgnoredArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerRequestIgnoredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerRequestIgnoredArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerRequestIgnoredArgs>, base.5, RequestId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTracker>;
    fn CreateWithOwner(&self, compositor: &::core::option::Option<super::Compositor>, owner: &::core::option::Option<IInteractionTrackerOwner>) -> ::windows::core::Result<InteractionTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerStaticsVtbl {
    pub const fn new<Impl: IInteractionTrackerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithOwner<Impl: IInteractionTrackerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithOwner(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType), &*(&owner as *const <IInteractionTrackerOwner as ::windows::core::Abi>::Abi as *const <IInteractionTrackerOwner as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerStatics>, base.5, Create::<Impl, OFFSET>, CreateWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerStatics2Impl: Sized {
    fn SetBindingMode(&self, boundtracker1: &::core::option::Option<InteractionTracker>, boundtracker2: &::core::option::Option<InteractionTracker>, axismode: InteractionBindingAxisModes) -> ::windows::core::Result<()>;
    fn GetBindingMode(&self, boundtracker1: &::core::option::Option<InteractionTracker>, boundtracker2: &::core::option::Option<InteractionTracker>) -> ::windows::core::Result<InteractionBindingAxisModes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerStatics2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerStatics2Vtbl {
    pub const fn new<Impl: IInteractionTrackerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerStatics2Vtbl {
        unsafe extern "system" fn SetBindingMode<Impl: IInteractionTrackerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, axismode: InteractionBindingAxisModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBindingMode(&*(&boundtracker1 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&boundtracker2 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), axismode).into()
        }
        unsafe extern "system" fn GetBindingMode<Impl: IInteractionTrackerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, result__: *mut InteractionBindingAxisModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBindingMode(&*(&boundtracker1 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&boundtracker2 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerStatics2>, base.5, SetBindingMode::<Impl, OFFSET>, GetBindingMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerValuesChangedArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn Scale(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerValuesChangedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerValuesChangedArgsVtbl {
    pub const fn new<Impl: IInteractionTrackerValuesChangedArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerValuesChangedArgsVtbl {
        unsafe extern "system" fn Position<Impl: IInteractionTrackerValuesChangedArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerValuesChangedArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scale<Impl: IInteractionTrackerValuesChangedArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerValuesChangedArgs>, base.5, Position::<Impl, OFFSET>, RequestId::<Impl, OFFSET>, Scale::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaModifierImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaModifier";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaModifierVtbl {
    pub const fn new<Impl: IInteractionTrackerVector2InertiaModifierImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerVector2InertiaModifierVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerVector2InertiaModifier>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaModifierFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaModifierFactory {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaModifierFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaModifierFactoryVtbl {
    pub const fn new<Impl: IInteractionTrackerVector2InertiaModifierFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerVector2InertiaModifierFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerVector2InertiaModifierFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaNaturalMotionImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn NaturalMotion(&self) -> ::windows::core::Result<super::Vector2NaturalMotionAnimation>;
    fn SetNaturalMotion(&self, value: &::core::option::Option<super::Vector2NaturalMotionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaNaturalMotion";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaNaturalMotionVtbl {
    pub const fn new<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerVector2InertiaNaturalMotionVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalMotion<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalMotion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNaturalMotion<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNaturalMotion(&*(&value as *const <super::Vector2NaturalMotionAnimation as ::windows::core::Abi>::Abi as *const <super::Vector2NaturalMotionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerVector2InertiaNaturalMotion>, base.5, Condition::<Impl, OFFSET>, SetCondition::<Impl, OFFSET>, NaturalMotion::<Impl, OFFSET>, SetNaturalMotion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaNaturalMotionStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerVector2InertiaNaturalMotion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaNaturalMotionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaNaturalMotionStaticsVtbl {
    pub const fn new<Impl: IInteractionTrackerVector2InertiaNaturalMotionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractionTrackerVector2InertiaNaturalMotionStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerVector2InertiaNaturalMotionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractionTrackerVector2InertiaNaturalMotionStatics>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceImpl: Sized {
    fn IsPositionXRailsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPositionYRailsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ManipulationRedirectionMode(&self) -> ::windows::core::Result<VisualInteractionSourceRedirectionMode>;
    fn SetManipulationRedirectionMode(&self, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
    fn PositionXChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode>;
    fn SetPositionXChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()>;
    fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode>;
    fn SetPositionXSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()>;
    fn PositionYChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode>;
    fn SetPositionYChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()>;
    fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode>;
    fn SetPositionYSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()>;
    fn ScaleChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode>;
    fn SetScaleChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()>;
    fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode>;
    fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<super::Visual>;
    fn TryRedirectForManipulation(&self, pointerpoint: &::core::option::Option<super::super::Input::PointerPoint>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSourceVtbl {
    pub const fn new<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualInteractionSourceVtbl {
        unsafe extern "system" fn IsPositionXRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPositionXRailsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPositionXRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPositionXRailsEnabled(value).into()
        }
        unsafe extern "system" fn IsPositionYRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPositionYRailsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPositionYRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPositionYRailsEnabled(value).into()
        }
        unsafe extern "system" fn ManipulationRedirectionMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationRedirectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManipulationRedirectionMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetManipulationRedirectionMode(value).into()
        }
        unsafe extern "system" fn PositionXChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionXChainingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionXChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionXChainingMode(value).into()
        }
        unsafe extern "system" fn PositionXSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionXSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionXSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionXSourceMode(value).into()
        }
        unsafe extern "system" fn PositionYChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionYChainingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionYChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionYChainingMode(value).into()
        }
        unsafe extern "system" fn PositionYSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionYSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionYSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionYSourceMode(value).into()
        }
        unsafe extern "system" fn ScaleChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleChainingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleChainingMode(value).into()
        }
        unsafe extern "system" fn ScaleSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleSourceMode(value).into()
        }
        unsafe extern "system" fn Source<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRedirectForManipulation<Impl: IVisualInteractionSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).TryRedirectForManipulation(&*(&pointerpoint as *const <super::super::Input::PointerPoint as ::windows::core::Abi>::Abi as *const <super::super::Input::PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVisualInteractionSource>,
            base.5,
            IsPositionXRailsEnabled::<Impl, OFFSET>,
            SetIsPositionXRailsEnabled::<Impl, OFFSET>,
            IsPositionYRailsEnabled::<Impl, OFFSET>,
            SetIsPositionYRailsEnabled::<Impl, OFFSET>,
            ManipulationRedirectionMode::<Impl, OFFSET>,
            SetManipulationRedirectionMode::<Impl, OFFSET>,
            PositionXChainingMode::<Impl, OFFSET>,
            SetPositionXChainingMode::<Impl, OFFSET>,
            PositionXSourceMode::<Impl, OFFSET>,
            SetPositionXSourceMode::<Impl, OFFSET>,
            PositionYChainingMode::<Impl, OFFSET>,
            SetPositionYChainingMode::<Impl, OFFSET>,
            PositionYSourceMode::<Impl, OFFSET>,
            SetPositionYSourceMode::<Impl, OFFSET>,
            ScaleChainingMode::<Impl, OFFSET>,
            SetScaleChainingMode::<Impl, OFFSET>,
            ScaleSourceMode::<Impl, OFFSET>,
            SetScaleSourceMode::<Impl, OFFSET>,
            Source::<Impl, OFFSET>,
            TryRedirectForManipulation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSource2Impl: Sized {
    fn DeltaPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn DeltaScale(&self) -> ::windows::core::Result<f32>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn PositionVelocity(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Scale(&self) -> ::windows::core::Result<f32>;
    fn ScaleVelocity(&self) -> ::windows::core::Result<f32>;
    fn ConfigureCenterPointXModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureCenterPointYModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureDeltaPositionXModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureDeltaPositionYModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureDeltaScaleModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSource2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSource2Vtbl {
    pub const fn new<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualInteractionSource2Vtbl {
        unsafe extern "system" fn DeltaPosition<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeltaPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaScale<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeltaScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionVelocity<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scale<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleVelocity<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureCenterPointXModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointXModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureCenterPointYModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointYModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaPositionXModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaPositionXModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaPositionYModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaPositionYModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaScaleModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaScaleModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVisualInteractionSource2>,
            base.5,
            DeltaPosition::<Impl, OFFSET>,
            DeltaScale::<Impl, OFFSET>,
            Position::<Impl, OFFSET>,
            PositionVelocity::<Impl, OFFSET>,
            Scale::<Impl, OFFSET>,
            ScaleVelocity::<Impl, OFFSET>,
            ConfigureCenterPointXModifiers::<Impl, OFFSET>,
            ConfigureCenterPointYModifiers::<Impl, OFFSET>,
            ConfigureDeltaPositionXModifiers::<Impl, OFFSET>,
            ConfigureDeltaPositionYModifiers::<Impl, OFFSET>,
            ConfigureDeltaScaleModifiers::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSource3Impl: Sized {
    fn PointerWheelConfig(&self) -> ::windows::core::Result<InteractionSourceConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSource3 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource3";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSource3Vtbl {
    pub const fn new<Impl: IVisualInteractionSource3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualInteractionSource3Vtbl {
        unsafe extern "system" fn PointerWheelConfig<Impl: IVisualInteractionSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerWheelConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualInteractionSource3>, base.5, PointerWheelConfig::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSourceObjectFactory {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSourceObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSourceObjectFactoryVtbl {
    pub const fn new<Impl: IVisualInteractionSourceObjectFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualInteractionSourceObjectFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualInteractionSourceObjectFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceStaticsImpl: Sized {
    fn Create(&self, source: &::core::option::Option<super::Visual>) -> ::windows::core::Result<VisualInteractionSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSourceStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSourceStaticsVtbl {
    pub const fn new<Impl: IVisualInteractionSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualInteractionSourceStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IVisualInteractionSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&source as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualInteractionSourceStatics>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceStatics2Impl: Sized {
    fn CreateFromIVisualElement(&self, source: &::core::option::Option<super::IVisualElement>) -> ::windows::core::Result<VisualInteractionSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSourceStatics2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSourceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSourceStatics2Vtbl {
    pub const fn new<Impl: IVisualInteractionSourceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualInteractionSourceStatics2Vtbl {
        unsafe extern "system" fn CreateFromIVisualElement<Impl: IVisualInteractionSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromIVisualElement(&*(&source as *const <super::IVisualElement as ::windows::core::Abi>::Abi as *const <super::IVisualElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualInteractionSourceStatics2>, base.5, CreateFromIVisualElement::<Impl, OFFSET>)
    }
}
