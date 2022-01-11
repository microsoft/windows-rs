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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionConditionalValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionConditionalValueVtbl {
        unsafe extern "system" fn Condition<Impl: ICompositionConditionalValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: ICompositionConditionalValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ICompositionConditionalValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ICompositionConditionalValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionConditionalValue, BASE_OFFSET>(),
            Condition: Condition::<Impl, IMPL_OFFSET>,
            SetCondition: SetCondition::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionConditionalValue as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionConditionalValueStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionConditionalValueStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ICompositionConditionalValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionConditionalValueStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionConditionalValueStatics as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionInteractionSourceImpl: Sized {}
impl ::windows::core::RuntimeName for ICompositionInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSource";
}
impl ICompositionInteractionSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionInteractionSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionInteractionSourceVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionInteractionSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionInteractionSource as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionInteractionSourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionInteractionSourceCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&value as *const <ICompositionInteractionSource as ::windows::core::Abi>::Abi as *const <ICompositionInteractionSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&value as *const <ICompositionInteractionSource as ::windows::core::Abi>::Abi as *const <ICompositionInteractionSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionInteractionSourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionInteractionSourceCollection, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionInteractionSourceCollection as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionSourceConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionSourceConfigurationVtbl {
        unsafe extern "system" fn PositionXSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionXSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionXSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionXSourceMode(value).into()
        }
        unsafe extern "system" fn PositionYSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionYSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionYSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionYSourceMode(value).into()
        }
        unsafe extern "system" fn ScaleSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleSourceMode<Impl: IInteractionSourceConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleSourceMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionSourceConfiguration, BASE_OFFSET>(),
            PositionXSourceMode: PositionXSourceMode::<Impl, IMPL_OFFSET>,
            SetPositionXSourceMode: SetPositionXSourceMode::<Impl, IMPL_OFFSET>,
            PositionYSourceMode: PositionYSourceMode::<Impl, IMPL_OFFSET>,
            SetPositionYSourceMode: SetPositionYSourceMode::<Impl, IMPL_OFFSET>,
            ScaleSourceMode: ScaleSourceMode::<Impl, IMPL_OFFSET>,
            SetScaleSourceMode: SetScaleSourceMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionSourceConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVtbl {
        unsafe extern "system" fn InteractionSources<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InteractionSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPositionRoundingSuggested<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPositionRoundingSuggested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPosition<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPosition<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxScale<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxScale<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxScale(value).into()
        }
        unsafe extern "system" fn MinPosition<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPosition<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinScale<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinScale<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinScale(value).into()
        }
        unsafe extern "system" fn NaturalRestingPosition<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalRestingPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalRestingScale<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalRestingScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Owner<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionInertiaDecayRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionInertiaDecayRate(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionVelocityInPixelsPerSecond<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionVelocityInPixelsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scale<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleInertiaDecayRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleInertiaDecayRate<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleInertiaDecayRate(&*(&value as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScaleVelocityInPercentPerSecond<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleVelocityInPercentPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustPositionXIfGreaterThanThreshold<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustPositionXIfGreaterThanThreshold(adjustment, positionthreshold).into()
        }
        unsafe extern "system" fn AdjustPositionYIfGreaterThanThreshold<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustPositionYIfGreaterThanThreshold(adjustment, positionthreshold).into()
        }
        unsafe extern "system" fn ConfigurePositionXInertiaModifiers<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigurePositionXInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigurePositionYInertiaModifiers<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigurePositionYInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureScaleInertiaModifiers<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureScaleInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryUpdatePosition<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdatePosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionBy<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionBy(&*(&amount as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionWithAnimation<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithAnimation(&*(&animation as *const <super::CompositionAnimation as ::windows::core::Abi>::Abi as *const <super::CompositionAnimation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionWithAdditionalVelocity<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithAdditionalVelocity(&*(&velocityinpixelspersecond as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateScale<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdateScale(value, &*(&centerpoint as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateScaleWithAnimation<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdateScaleWithAnimation(&*(&animation as *const <super::CompositionAnimation as ::windows::core::Abi>::Abi as *const <super::CompositionAnimation as ::windows::core::DefaultType>::DefaultType), &*(&centerpoint as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdateScaleWithAdditionalVelocity<Impl: IInteractionTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdateScaleWithAdditionalVelocity(velocityinpercentpersecond, &*(&centerpoint as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTracker, BASE_OFFSET>(),
            InteractionSources: InteractionSources::<Impl, IMPL_OFFSET>,
            IsPositionRoundingSuggested: IsPositionRoundingSuggested::<Impl, IMPL_OFFSET>,
            MaxPosition: MaxPosition::<Impl, IMPL_OFFSET>,
            SetMaxPosition: SetMaxPosition::<Impl, IMPL_OFFSET>,
            MaxScale: MaxScale::<Impl, IMPL_OFFSET>,
            SetMaxScale: SetMaxScale::<Impl, IMPL_OFFSET>,
            MinPosition: MinPosition::<Impl, IMPL_OFFSET>,
            SetMinPosition: SetMinPosition::<Impl, IMPL_OFFSET>,
            MinScale: MinScale::<Impl, IMPL_OFFSET>,
            SetMinScale: SetMinScale::<Impl, IMPL_OFFSET>,
            NaturalRestingPosition: NaturalRestingPosition::<Impl, IMPL_OFFSET>,
            NaturalRestingScale: NaturalRestingScale::<Impl, IMPL_OFFSET>,
            Owner: Owner::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            PositionInertiaDecayRate: PositionInertiaDecayRate::<Impl, IMPL_OFFSET>,
            SetPositionInertiaDecayRate: SetPositionInertiaDecayRate::<Impl, IMPL_OFFSET>,
            PositionVelocityInPixelsPerSecond: PositionVelocityInPixelsPerSecond::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            ScaleInertiaDecayRate: ScaleInertiaDecayRate::<Impl, IMPL_OFFSET>,
            SetScaleInertiaDecayRate: SetScaleInertiaDecayRate::<Impl, IMPL_OFFSET>,
            ScaleVelocityInPercentPerSecond: ScaleVelocityInPercentPerSecond::<Impl, IMPL_OFFSET>,
            AdjustPositionXIfGreaterThanThreshold: AdjustPositionXIfGreaterThanThreshold::<Impl, IMPL_OFFSET>,
            AdjustPositionYIfGreaterThanThreshold: AdjustPositionYIfGreaterThanThreshold::<Impl, IMPL_OFFSET>,
            ConfigurePositionXInertiaModifiers: ConfigurePositionXInertiaModifiers::<Impl, IMPL_OFFSET>,
            ConfigurePositionYInertiaModifiers: ConfigurePositionYInertiaModifiers::<Impl, IMPL_OFFSET>,
            ConfigureScaleInertiaModifiers: ConfigureScaleInertiaModifiers::<Impl, IMPL_OFFSET>,
            TryUpdatePosition: TryUpdatePosition::<Impl, IMPL_OFFSET>,
            TryUpdatePositionBy: TryUpdatePositionBy::<Impl, IMPL_OFFSET>,
            TryUpdatePositionWithAnimation: TryUpdatePositionWithAnimation::<Impl, IMPL_OFFSET>,
            TryUpdatePositionWithAdditionalVelocity: TryUpdatePositionWithAdditionalVelocity::<Impl, IMPL_OFFSET>,
            TryUpdateScale: TryUpdateScale::<Impl, IMPL_OFFSET>,
            TryUpdateScaleWithAnimation: TryUpdateScaleWithAnimation::<Impl, IMPL_OFFSET>,
            TryUpdateScaleWithAdditionalVelocity: TryUpdateScaleWithAdditionalVelocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTracker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInteractionTracker2Impl: Sized {
    fn ConfigureCenterPointXInertiaModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureCenterPointYInertiaModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInteractionTracker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker2Vtbl {
        unsafe extern "system" fn ConfigureCenterPointXInertiaModifiers<Impl: IInteractionTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointXInertiaModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureCenterPointYInertiaModifiers<Impl: IInteractionTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointYInertiaModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTracker2, BASE_OFFSET>(),
            ConfigureCenterPointXInertiaModifiers: ConfigureCenterPointXInertiaModifiers::<Impl, IMPL_OFFSET>,
            ConfigureCenterPointYInertiaModifiers: ConfigureCenterPointYInertiaModifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTracker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInteractionTracker3Impl: Sized {
    fn ConfigureVector2PositionInertiaModifiers(&self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker3 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInteractionTracker3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker3Vtbl {
        unsafe extern "system" fn ConfigureVector2PositionInertiaModifiers<Impl: IInteractionTracker3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureVector2PositionInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTracker3, BASE_OFFSET>(),
            ConfigureVector2PositionInertiaModifiers: ConfigureVector2PositionInertiaModifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTracker3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInteractionTracker4Impl: Sized {
    fn TryUpdatePositionWithOption(&self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionByWithOption(&self, amount: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker4 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker4";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTracker4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker4Vtbl {
        unsafe extern "system" fn TryUpdatePositionWithOption<Impl: IInteractionTracker4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithOption(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdatePositionByWithOption<Impl: IInteractionTracker4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionByWithOption(&*(&amount as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInertiaFromImpulse<Impl: IInteractionTracker4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInertiaFromImpulse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTracker4, BASE_OFFSET>(),
            TryUpdatePositionWithOption: TryUpdatePositionWithOption::<Impl, IMPL_OFFSET>,
            TryUpdatePositionByWithOption: TryUpdatePositionByWithOption::<Impl, IMPL_OFFSET>,
            IsInertiaFromImpulse: IsInertiaFromImpulse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTracker4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInteractionTracker5Impl: Sized {
    fn TryUpdatePositionWithOption(&self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker5 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker5";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTracker5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker5Vtbl {
        unsafe extern "system" fn TryUpdatePositionWithOption<Impl: IInteractionTracker5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUpdatePositionWithOption(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), option, posupdateoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTracker5, BASE_OFFSET>(),
            TryUpdatePositionWithOption: TryUpdatePositionWithOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTracker5 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerCustomAnimationStateEnteredArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerCustomAnimationStateEnteredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerCustomAnimationStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerCustomAnimationStateEnteredArgs, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerCustomAnimationStateEnteredArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerCustomAnimationStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerCustomAnimationStateEnteredArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerCustomAnimationStateEnteredArgs2, BASE_OFFSET>(),
            IsFromBinding: IsFromBinding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerCustomAnimationStateEnteredArgs2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerIdleStateEnteredArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerIdleStateEnteredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerIdleStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerIdleStateEnteredArgs, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerIdleStateEnteredArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerIdleStateEnteredArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerIdleStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerIdleStateEnteredArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerIdleStateEnteredArgs2, BASE_OFFSET>(),
            IsFromBinding: IsFromBinding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerIdleStateEnteredArgs2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaModifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaModifierVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaModifier, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaModifier as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaModifierFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaModifierFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaModifierFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaModifierFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaMotionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaMotionVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Motion<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Motion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMotion<Impl: IInteractionTrackerInertiaMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMotion(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaMotion, BASE_OFFSET>(),
            Condition: Condition::<Impl, IMPL_OFFSET>,
            SetCondition: SetCondition::<Impl, IMPL_OFFSET>,
            Motion: Motion::<Impl, IMPL_OFFSET>,
            SetMotion: SetMotion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaMotion as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaMotionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaMotionStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaMotionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaMotionStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaMotionStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaNaturalMotionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaNaturalMotionVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalMotion<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalMotion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNaturalMotion<Impl: IInteractionTrackerInertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNaturalMotion(&*(&value as *const <super::ScalarNaturalMotionAnimation as ::windows::core::Abi>::Abi as *const <super::ScalarNaturalMotionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaNaturalMotion, BASE_OFFSET>(),
            Condition: Condition::<Impl, IMPL_OFFSET>,
            SetCondition: SetCondition::<Impl, IMPL_OFFSET>,
            NaturalMotion: NaturalMotion::<Impl, IMPL_OFFSET>,
            SetNaturalMotion: SetNaturalMotion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaNaturalMotion as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaNaturalMotionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaNaturalMotionStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaNaturalMotionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaNaturalMotionStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaNaturalMotionStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaRestingValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaRestingValueVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestingValue<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestingValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRestingValue<Impl: IInteractionTrackerInertiaRestingValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestingValue(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaRestingValue, BASE_OFFSET>(),
            Condition: Condition::<Impl, IMPL_OFFSET>,
            SetCondition: SetCondition::<Impl, IMPL_OFFSET>,
            RestingValue: RestingValue::<Impl, IMPL_OFFSET>,
            SetRestingValue: SetRestingValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaRestingValue as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaRestingValueStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaRestingValueStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaRestingValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaRestingValueStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaRestingValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInteractionTrackerInertiaStateEnteredArgsImpl: Sized {
    fn ModifiedRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn ModifiedRestingScale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn NaturalRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn NaturalRestingScale(&self) -> ::windows::core::Result<f32>;
    fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaStateEnteredArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTrackerInertiaStateEnteredArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaStateEnteredArgsVtbl {
        unsafe extern "system" fn ModifiedRestingPosition<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifiedRestingPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedRestingScale<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifiedRestingScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalRestingPosition<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalRestingPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalRestingScale<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalRestingScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionVelocityInPixelsPerSecond<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionVelocityInPixelsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleVelocityInPercentPerSecond<Impl: IInteractionTrackerInertiaStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleVelocityInPercentPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaStateEnteredArgs, BASE_OFFSET>(),
            ModifiedRestingPosition: ModifiedRestingPosition::<Impl, IMPL_OFFSET>,
            ModifiedRestingScale: ModifiedRestingScale::<Impl, IMPL_OFFSET>,
            NaturalRestingPosition: NaturalRestingPosition::<Impl, IMPL_OFFSET>,
            NaturalRestingScale: NaturalRestingScale::<Impl, IMPL_OFFSET>,
            PositionVelocityInPixelsPerSecond: PositionVelocityInPixelsPerSecond::<Impl, IMPL_OFFSET>,
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            ScaleVelocityInPercentPerSecond: ScaleVelocityInPercentPerSecond::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaStateEnteredArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaStateEnteredArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsInertiaFromImpulse<Impl: IInteractionTrackerInertiaStateEnteredArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInertiaFromImpulse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaStateEnteredArgs2, BASE_OFFSET>(),
            IsInertiaFromImpulse: IsInertiaFromImpulse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaStateEnteredArgs2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaStateEnteredArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaStateEnteredArgs3Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerInertiaStateEnteredArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaStateEnteredArgs3, BASE_OFFSET>(),
            IsFromBinding: IsFromBinding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaStateEnteredArgs3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInteractingStateEnteredArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInteractingStateEnteredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerInteractingStateEnteredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInteractingStateEnteredArgs, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInteractingStateEnteredArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInteractingStateEnteredArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInteractingStateEnteredArgs2Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerInteractingStateEnteredArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFromBinding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInteractingStateEnteredArgs2, BASE_OFFSET>(),
            IsFromBinding: IsFromBinding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInteractingStateEnteredArgs2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwnerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerOwnerVtbl {
        unsafe extern "system" fn CustomAnimationStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CustomAnimationStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IdleStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IdleStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerIdleStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerIdleStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InertiaStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InertiaStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerInertiaStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerInertiaStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InteractingStateEntered<Impl: IInteractionTrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InteractingStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerInteractingStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerInteractingStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestIgnored<Impl: IInteractionTrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestIgnored(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerRequestIgnoredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerRequestIgnoredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ValuesChanged<Impl: IInteractionTrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValuesChanged(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerValuesChangedArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerValuesChangedArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerOwner, BASE_OFFSET>(),
            CustomAnimationStateEntered: CustomAnimationStateEntered::<Impl, IMPL_OFFSET>,
            IdleStateEntered: IdleStateEntered::<Impl, IMPL_OFFSET>,
            InertiaStateEntered: InertiaStateEntered::<Impl, IMPL_OFFSET>,
            InteractingStateEntered: InteractingStateEntered::<Impl, IMPL_OFFSET>,
            RequestIgnored: RequestIgnored::<Impl, IMPL_OFFSET>,
            ValuesChanged: ValuesChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerOwner as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerRequestIgnoredArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerRequestIgnoredArgsVtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerRequestIgnoredArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerRequestIgnoredArgs, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerRequestIgnoredArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithOwner<Impl: IInteractionTrackerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithOwner(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType), &*(&owner as *const <IInteractionTrackerOwner as ::windows::core::Abi>::Abi as *const <IInteractionTrackerOwner as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithOwner: CreateWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerStatics2Vtbl {
        unsafe extern "system" fn SetBindingMode<Impl: IInteractionTrackerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, axismode: InteractionBindingAxisModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBindingMode(&*(&boundtracker1 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&boundtracker2 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), axismode).into()
        }
        unsafe extern "system" fn GetBindingMode<Impl: IInteractionTrackerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, result__: *mut InteractionBindingAxisModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindingMode(&*(&boundtracker1 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&boundtracker2 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerStatics2, BASE_OFFSET>(),
            SetBindingMode: SetBindingMode::<Impl, IMPL_OFFSET>,
            GetBindingMode: GetBindingMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IInteractionTrackerValuesChangedArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn Scale(&self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerValuesChangedArgs";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTrackerValuesChangedArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerValuesChangedArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerValuesChangedArgsVtbl {
        unsafe extern "system" fn Position<Impl: IInteractionTrackerValuesChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerValuesChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scale<Impl: IInteractionTrackerValuesChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerValuesChangedArgs, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerValuesChangedArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaModifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaModifierVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerVector2InertiaModifier, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerVector2InertiaModifier as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaModifierFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaModifierFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerVector2InertiaModifierFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerVector2InertiaModifierFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaNaturalMotionVtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalMotion<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalMotion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNaturalMotion<Impl: IInteractionTrackerVector2InertiaNaturalMotionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNaturalMotion(&*(&value as *const <super::Vector2NaturalMotionAnimation as ::windows::core::Abi>::Abi as *const <super::Vector2NaturalMotionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerVector2InertiaNaturalMotion, BASE_OFFSET>(),
            Condition: Condition::<Impl, IMPL_OFFSET>,
            SetCondition: SetCondition::<Impl, IMPL_OFFSET>,
            NaturalMotion: NaturalMotion::<Impl, IMPL_OFFSET>,
            SetNaturalMotion: SetNaturalMotion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerVector2InertiaNaturalMotion as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaNaturalMotionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaNaturalMotionStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerVector2InertiaNaturalMotionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerVector2InertiaNaturalMotionStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerVector2InertiaNaturalMotionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Input", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource";
}
#[cfg(all(feature = "UI_Input", feature = "implement_exclusive"))]
impl IVisualInteractionSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceVtbl {
        unsafe extern "system" fn IsPositionXRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPositionXRailsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPositionXRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPositionXRailsEnabled(value).into()
        }
        unsafe extern "system" fn IsPositionYRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPositionYRailsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPositionYRailsEnabled<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPositionYRailsEnabled(value).into()
        }
        unsafe extern "system" fn ManipulationRedirectionMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationRedirectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManipulationRedirectionMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManipulationRedirectionMode(value).into()
        }
        unsafe extern "system" fn PositionXChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionXChainingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionXChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionXChainingMode(value).into()
        }
        unsafe extern "system" fn PositionXSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionXSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionXSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionXSourceMode(value).into()
        }
        unsafe extern "system" fn PositionYChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionYChainingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionYChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionYChainingMode(value).into()
        }
        unsafe extern "system" fn PositionYSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionYSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionYSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionYSourceMode(value).into()
        }
        unsafe extern "system" fn ScaleChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleChainingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleChainingMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleChainingMode(value).into()
        }
        unsafe extern "system" fn ScaleSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleSourceMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleSourceMode<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleSourceMode(value).into()
        }
        unsafe extern "system" fn Source<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRedirectForManipulation<Impl: IVisualInteractionSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryRedirectForManipulation(&*(&pointerpoint as *const <super::super::Input::PointerPoint as ::windows::core::Abi>::Abi as *const <super::super::Input::PointerPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualInteractionSource, BASE_OFFSET>(),
            IsPositionXRailsEnabled: IsPositionXRailsEnabled::<Impl, IMPL_OFFSET>,
            SetIsPositionXRailsEnabled: SetIsPositionXRailsEnabled::<Impl, IMPL_OFFSET>,
            IsPositionYRailsEnabled: IsPositionYRailsEnabled::<Impl, IMPL_OFFSET>,
            SetIsPositionYRailsEnabled: SetIsPositionYRailsEnabled::<Impl, IMPL_OFFSET>,
            ManipulationRedirectionMode: ManipulationRedirectionMode::<Impl, IMPL_OFFSET>,
            SetManipulationRedirectionMode: SetManipulationRedirectionMode::<Impl, IMPL_OFFSET>,
            PositionXChainingMode: PositionXChainingMode::<Impl, IMPL_OFFSET>,
            SetPositionXChainingMode: SetPositionXChainingMode::<Impl, IMPL_OFFSET>,
            PositionXSourceMode: PositionXSourceMode::<Impl, IMPL_OFFSET>,
            SetPositionXSourceMode: SetPositionXSourceMode::<Impl, IMPL_OFFSET>,
            PositionYChainingMode: PositionYChainingMode::<Impl, IMPL_OFFSET>,
            SetPositionYChainingMode: SetPositionYChainingMode::<Impl, IMPL_OFFSET>,
            PositionYSourceMode: PositionYSourceMode::<Impl, IMPL_OFFSET>,
            SetPositionYSourceMode: SetPositionYSourceMode::<Impl, IMPL_OFFSET>,
            ScaleChainingMode: ScaleChainingMode::<Impl, IMPL_OFFSET>,
            SetScaleChainingMode: SetScaleChainingMode::<Impl, IMPL_OFFSET>,
            ScaleSourceMode: ScaleSourceMode::<Impl, IMPL_OFFSET>,
            SetScaleSourceMode: SetScaleSourceMode::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            TryRedirectForManipulation: TryRedirectForManipulation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualInteractionSource2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVisualInteractionSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSource2Vtbl {
        unsafe extern "system" fn DeltaPosition<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaScale<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionVelocity<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scale<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleVelocity<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureCenterPointXModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointXModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureCenterPointYModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointYModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaPositionXModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaPositionXModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaPositionYModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaPositionYModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaScaleModifiers<Impl: IVisualInteractionSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaScaleModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualInteractionSource2, BASE_OFFSET>(),
            DeltaPosition: DeltaPosition::<Impl, IMPL_OFFSET>,
            DeltaScale: DeltaScale::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            PositionVelocity: PositionVelocity::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            ScaleVelocity: ScaleVelocity::<Impl, IMPL_OFFSET>,
            ConfigureCenterPointXModifiers: ConfigureCenterPointXModifiers::<Impl, IMPL_OFFSET>,
            ConfigureCenterPointYModifiers: ConfigureCenterPointYModifiers::<Impl, IMPL_OFFSET>,
            ConfigureDeltaPositionXModifiers: ConfigureDeltaPositionXModifiers::<Impl, IMPL_OFFSET>,
            ConfigureDeltaPositionYModifiers: ConfigureDeltaPositionYModifiers::<Impl, IMPL_OFFSET>,
            ConfigureDeltaScaleModifiers: ConfigureDeltaScaleModifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSource2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSource3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSource3Vtbl {
        unsafe extern "system" fn PointerWheelConfig<Impl: IVisualInteractionSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerWheelConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualInteractionSource3, BASE_OFFSET>(),
            PointerWheelConfig: PointerWheelConfig::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSource3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceObjectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceObjectFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualInteractionSourceObjectFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSourceObjectFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IVisualInteractionSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&source as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualInteractionSourceStatics, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSourceStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceStatics2Vtbl {
        unsafe extern "system" fn CreateFromIVisualElement<Impl: IVisualInteractionSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIVisualElement(&*(&source as *const <super::IVisualElement as ::windows::core::Abi>::Abi as *const <super::IVisualElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualInteractionSourceStatics2, BASE_OFFSET>(),
            CreateFromIVisualElement: CreateFromIVisualElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSourceStatics2 as ::windows::core::Interface>::IID
    }
}
