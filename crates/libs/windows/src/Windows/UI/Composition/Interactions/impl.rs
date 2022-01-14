#[cfg(feature = "implement_exclusive")]
pub trait ICompositionConditionalValue_Impl: Sized {
    fn Condition(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetValue(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionConditionalValue";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionConditionalValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionConditionalValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionConditionalValue_Vtbl {
        unsafe extern "system" fn Condition<Impl: ICompositionConditionalValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCondition<Impl: ICompositionConditionalValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ICompositionConditionalValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: ICompositionConditionalValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionConditionalValueStatics_Impl: Sized {
    fn Create(&mut self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<CompositionConditionalValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionConditionalValueStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionConditionalValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionConditionalValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionConditionalValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionConditionalValueStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: ICompositionConditionalValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompositionInteractionSource_Impl: Sized {}
impl ::windows::core::RuntimeName for ICompositionInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSource";
}
impl ICompositionInteractionSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionInteractionSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionInteractionSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionInteractionSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionInteractionSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionInteractionSourceCollection_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, value: &::core::option::Option<ICompositionInteractionSource>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, value: &::core::option::Option<ICompositionInteractionSource>) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionInteractionSourceCollection {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSourceCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionInteractionSourceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionInteractionSourceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionInteractionSourceCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: ICompositionInteractionSourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Add<Impl: ICompositionInteractionSourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&value as *const <ICompositionInteractionSource as ::windows::core::Abi>::Abi as *const <ICompositionInteractionSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICompositionInteractionSourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&value as *const <ICompositionInteractionSource as ::windows::core::Abi>::Abi as *const <ICompositionInteractionSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ICompositionInteractionSourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IInteractionSourceConfiguration_Impl: Sized {
    fn PositionXSourceMode(&mut self) -> ::windows::core::Result<InteractionSourceRedirectionMode>;
    fn SetPositionXSourceMode(&mut self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
    fn PositionYSourceMode(&mut self) -> ::windows::core::Result<InteractionSourceRedirectionMode>;
    fn SetPositionYSourceMode(&mut self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
    fn ScaleSourceMode(&mut self) -> ::windows::core::Result<InteractionSourceRedirectionMode>;
    fn SetScaleSourceMode(&mut self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionSourceConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionSourceConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionSourceConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionSourceConfiguration_Vtbl {
        unsafe extern "system" fn PositionXSourceMode<Impl: IInteractionSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionXSourceMode<Impl: IInteractionSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionXSourceMode(value).into()
        }
        unsafe extern "system" fn PositionYSourceMode<Impl: IInteractionSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionYSourceMode<Impl: IInteractionSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionYSourceMode(value).into()
        }
        unsafe extern "system" fn ScaleSourceMode<Impl: IInteractionSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScaleSourceMode<Impl: IInteractionSourceConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
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
pub trait IInteractionTracker_Impl: Sized {
    fn InteractionSources(&mut self) -> ::windows::core::Result<CompositionInteractionSourceCollection>;
    fn IsPositionRoundingSuggested(&mut self) -> ::windows::core::Result<bool>;
    fn MaxPosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetMaxPosition(&mut self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn MaxScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetMaxScale(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn MinPosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetMinPosition(&mut self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn MinScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetMinScale(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn NaturalRestingPosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn NaturalRestingScale(&mut self) -> ::windows::core::Result<f32>;
    fn Owner(&mut self) -> ::windows::core::Result<IInteractionTrackerOwner>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn PositionInertiaDecayRate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SetPositionInertiaDecayRate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn PositionVelocityInPixelsPerSecond(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Scale(&mut self) -> ::windows::core::Result<f32>;
    fn ScaleInertiaDecayRate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn SetScaleInertiaDecayRate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn ScaleVelocityInPercentPerSecond(&mut self) -> ::windows::core::Result<f32>;
    fn AdjustPositionXIfGreaterThanThreshold(&mut self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()>;
    fn AdjustPositionYIfGreaterThanThreshold(&mut self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()>;
    fn ConfigurePositionXInertiaModifiers(&mut self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>) -> ::windows::core::Result<()>;
    fn ConfigurePositionYInertiaModifiers(&mut self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>) -> ::windows::core::Result<()>;
    fn ConfigureScaleInertiaModifiers(&mut self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>) -> ::windows::core::Result<()>;
    fn TryUpdatePosition(&mut self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionBy(&mut self, amount: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionWithAnimation(&mut self, animation: &::core::option::Option<super::CompositionAnimation>) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionWithAdditionalVelocity(&mut self, velocityinpixelspersecond: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdateScale(&mut self, value: f32, centerpoint: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdateScaleWithAnimation(&mut self, animation: &::core::option::Option<super::CompositionAnimation>, centerpoint: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
    fn TryUpdateScaleWithAdditionalVelocity(&mut self, velocityinpercentpersecond: f32, centerpoint: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTracker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker_Vtbl {
        unsafe extern "system" fn InteractionSources<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPositionRoundingSuggested<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxPosition<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxPosition<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxScale<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxScale<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxScale(value).into()
        }
        unsafe extern "system" fn MinPosition<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinPosition<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPosition(&*(&value as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinScale<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinScale<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinScale(value).into()
        }
        unsafe extern "system" fn NaturalRestingPosition<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NaturalRestingScale<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Owner<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionInertiaDecayRate<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionInertiaDecayRate<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionInertiaDecayRate(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionVelocityInPixelsPerSecond<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scale<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScaleInertiaDecayRate<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScaleInertiaDecayRate<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleInertiaDecayRate(&*(&value as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScaleVelocityInPercentPerSecond<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdjustPositionXIfGreaterThanThreshold<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustPositionXIfGreaterThanThreshold(adjustment, positionthreshold).into()
        }
        unsafe extern "system" fn AdjustPositionYIfGreaterThanThreshold<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustPositionYIfGreaterThanThreshold(adjustment, positionthreshold).into()
        }
        unsafe extern "system" fn ConfigurePositionXInertiaModifiers<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigurePositionXInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigurePositionYInertiaModifiers<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigurePositionYInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureScaleInertiaModifiers<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureScaleInertiaModifiers(&*(&modifiers as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryUpdatePosition<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUpdatePositionBy<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUpdatePositionWithAnimation<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUpdatePositionWithAdditionalVelocity<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUpdateScale<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUpdateScaleWithAnimation<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUpdateScaleWithAdditionalVelocity<Impl: IInteractionTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTracker2_Impl: Sized {
    fn ConfigureCenterPointXInertiaModifiers(&mut self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureCenterPointYInertiaModifiers(&mut self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInteractionTracker2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker2_Vtbl {
        unsafe extern "system" fn ConfigureCenterPointXInertiaModifiers<Impl: IInteractionTracker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointXInertiaModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureCenterPointYInertiaModifiers<Impl: IInteractionTracker2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTracker3_Impl: Sized {
    fn ConfigureVector2PositionInertiaModifiers(&mut self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker3 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInteractionTracker3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker3_Vtbl {
        unsafe extern "system" fn ConfigureVector2PositionInertiaModifiers<Impl: IInteractionTracker3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTracker4_Impl: Sized {
    fn TryUpdatePositionWithOption(&mut self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionByWithOption(&mut self, amount: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn IsInertiaFromImpulse(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker4 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker4";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTracker4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker4_Vtbl {
        unsafe extern "system" fn TryUpdatePositionWithOption<Impl: IInteractionTracker4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUpdatePositionByWithOption<Impl: IInteractionTracker4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInertiaFromImpulse<Impl: IInteractionTracker4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IInteractionTracker5_Impl: Sized {
    fn TryUpdatePositionWithOption(&mut self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTracker5 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTracker5";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTracker5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTracker5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTracker5_Vtbl {
        unsafe extern "system" fn TryUpdatePositionWithOption<Impl: IInteractionTracker5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerCustomAnimationStateEnteredArgs_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerCustomAnimationStateEnteredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerCustomAnimationStateEnteredArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerCustomAnimationStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerCustomAnimationStateEnteredArgs2_Impl: Sized {
    fn IsFromBinding(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerCustomAnimationStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerCustomAnimationStateEnteredArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerCustomAnimationStateEnteredArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerIdleStateEnteredArgs_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerIdleStateEnteredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerIdleStateEnteredArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerIdleStateEnteredArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerIdleStateEnteredArgs_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerIdleStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerIdleStateEnteredArgs2_Impl: Sized {
    fn IsFromBinding(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerIdleStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerIdleStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerIdleStateEnteredArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerIdleStateEnteredArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerIdleStateEnteredArgs2_Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerIdleStateEnteredArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaModifier_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaModifier";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaModifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaModifier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaModifier_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaModifier, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaModifier as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaModifierFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaModifierFactory {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaModifierFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaModifierFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaModifierFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaModifierFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerInertiaModifierFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerInertiaModifierFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaMotion_Impl: Sized {
    fn Condition(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn Motion(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetMotion(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaMotion";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaMotion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaMotion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaMotion_Vtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Motion<Impl: IInteractionTrackerInertiaMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMotion<Impl: IInteractionTrackerInertiaMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaMotionStatics_Impl: Sized {
    fn Create(&mut self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaMotion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaMotionStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaMotionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaMotionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaMotionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaMotionStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaMotionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaNaturalMotion_Impl: Sized {
    fn Condition(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn NaturalMotion(&mut self) -> ::windows::core::Result<super::ScalarNaturalMotionAnimation>;
    fn SetNaturalMotion(&mut self, value: &::core::option::Option<super::ScalarNaturalMotionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaNaturalMotion";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaNaturalMotion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaNaturalMotion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaNaturalMotion_Vtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalMotion<Impl: IInteractionTrackerInertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNaturalMotion<Impl: IInteractionTrackerInertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaNaturalMotionStatics_Impl: Sized {
    fn Create(&mut self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaNaturalMotion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaNaturalMotionStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaNaturalMotionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaNaturalMotionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaNaturalMotionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaNaturalMotionStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaNaturalMotionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaRestingValue_Impl: Sized {
    fn Condition(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn RestingValue(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetRestingValue(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaRestingValue";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaRestingValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaRestingValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaRestingValue_Vtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerInertiaRestingValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerInertiaRestingValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestingValue<Impl: IInteractionTrackerInertiaRestingValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRestingValue<Impl: IInteractionTrackerInertiaRestingValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaRestingValueStatics_Impl: Sized {
    fn Create(&mut self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaRestingValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaRestingValueStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaRestingValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaRestingValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaRestingValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaRestingValueStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerInertiaRestingValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaStateEnteredArgs_Impl: Sized {
    fn ModifiedRestingPosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn ModifiedRestingScale(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn NaturalRestingPosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn NaturalRestingScale(&mut self) -> ::windows::core::Result<f32>;
    fn PositionVelocityInPixelsPerSecond(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
    fn ScaleVelocityInPercentPerSecond(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaStateEnteredArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTrackerInertiaStateEnteredArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaStateEnteredArgs_Vtbl {
        unsafe extern "system" fn ModifiedRestingPosition<Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModifiedRestingScale<Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NaturalRestingPosition<Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NaturalRestingScale<Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionVelocityInPixelsPerSecond<Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScaleVelocityInPercentPerSecond<Impl: IInteractionTrackerInertiaStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaStateEnteredArgs2_Impl: Sized {
    fn IsInertiaFromImpulse(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaStateEnteredArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaStateEnteredArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaStateEnteredArgs2_Vtbl {
        unsafe extern "system" fn IsInertiaFromImpulse<Impl: IInteractionTrackerInertiaStateEnteredArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInertiaStateEnteredArgs3_Impl: Sized {
    fn IsFromBinding(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInertiaStateEnteredArgs3 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInertiaStateEnteredArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInertiaStateEnteredArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInertiaStateEnteredArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInertiaStateEnteredArgs3_Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerInertiaStateEnteredArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInteractingStateEnteredArgs_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInteractingStateEnteredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInteractingStateEnteredArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInteractingStateEnteredArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInteractingStateEnteredArgs_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerInteractingStateEnteredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerInteractingStateEnteredArgs2_Impl: Sized {
    fn IsFromBinding(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerInteractingStateEnteredArgs2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerInteractingStateEnteredArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerInteractingStateEnteredArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerInteractingStateEnteredArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerInteractingStateEnteredArgs2_Vtbl {
        unsafe extern "system" fn IsFromBinding<Impl: IInteractionTrackerInteractingStateEnteredArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerOwner_Impl: Sized {
    fn CustomAnimationStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerCustomAnimationStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn IdleStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerIdleStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InertiaStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInertiaStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InteractingStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInteractingStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn RequestIgnored(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerRequestIgnoredArgs>) -> ::windows::core::Result<()>;
    fn ValuesChanged(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerValuesChangedArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInteractionTrackerOwner {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerOwner";
}
impl IInteractionTrackerOwner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerOwner_Vtbl {
        unsafe extern "system" fn CustomAnimationStateEntered<Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CustomAnimationStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IdleStateEntered<Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IdleStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerIdleStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerIdleStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InertiaStateEntered<Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InertiaStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerInertiaStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerInertiaStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InteractingStateEntered<Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InteractingStateEntered(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerInteractingStateEnteredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerInteractingStateEnteredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestIgnored<Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestIgnored(&*(&sender as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <InteractionTrackerRequestIgnoredArgs as ::windows::core::Abi>::Abi as *const <InteractionTrackerRequestIgnoredArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ValuesChanged<Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerRequestIgnoredArgs_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerRequestIgnoredArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerRequestIgnoredArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerRequestIgnoredArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerRequestIgnoredArgs_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerRequestIgnoredArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerStatics_Impl: Sized {
    fn Create(&mut self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTracker>;
    fn CreateWithOwner(&mut self, compositor: &::core::option::Option<super::Compositor>, owner: &::core::option::Option<IInteractionTrackerOwner>) -> ::windows::core::Result<InteractionTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithOwner<Impl: IInteractionTrackerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerStatics2_Impl: Sized {
    fn SetBindingMode(&mut self, boundtracker1: &::core::option::Option<InteractionTracker>, boundtracker2: &::core::option::Option<InteractionTracker>, axismode: InteractionBindingAxisModes) -> ::windows::core::Result<()>;
    fn GetBindingMode(&mut self, boundtracker1: &::core::option::Option<InteractionTracker>, boundtracker2: &::core::option::Option<InteractionTracker>) -> ::windows::core::Result<InteractionBindingAxisModes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerStatics2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerStatics2_Vtbl {
        unsafe extern "system" fn SetBindingMode<Impl: IInteractionTrackerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, axismode: InteractionBindingAxisModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBindingMode(&*(&boundtracker1 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), &*(&boundtracker2 as *const <InteractionTracker as ::windows::core::Abi>::Abi as *const <InteractionTracker as ::windows::core::DefaultType>::DefaultType), axismode).into()
        }
        unsafe extern "system" fn GetBindingMode<Impl: IInteractionTrackerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, result__: *mut InteractionBindingAxisModes) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerValuesChangedArgs_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
    fn Scale(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerValuesChangedArgs";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IInteractionTrackerValuesChangedArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerValuesChangedArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerValuesChangedArgs_Vtbl {
        unsafe extern "system" fn Position<Impl: IInteractionTrackerValuesChangedArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestId<Impl: IInteractionTrackerValuesChangedArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scale<Impl: IInteractionTrackerValuesChangedArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerVector2InertiaModifier_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaModifier";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaModifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaModifier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaModifier_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerVector2InertiaModifier, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerVector2InertiaModifier as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaModifierFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaModifierFactory {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaModifierFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaModifierFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaModifierFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaModifierFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerVector2InertiaModifierFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerVector2InertiaModifierFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaNaturalMotion_Impl: Sized {
    fn Condition(&mut self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&mut self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn NaturalMotion(&mut self) -> ::windows::core::Result<super::Vector2NaturalMotionAnimation>;
    fn SetNaturalMotion(&mut self, value: &::core::option::Option<super::Vector2NaturalMotionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaNaturalMotion";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaNaturalMotion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaNaturalMotion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaNaturalMotion_Vtbl {
        unsafe extern "system" fn Condition<Impl: IInteractionTrackerVector2InertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCondition<Impl: IInteractionTrackerVector2InertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCondition(&*(&value as *const <super::ExpressionAnimation as ::windows::core::Abi>::Abi as *const <super::ExpressionAnimation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalMotion<Impl: IInteractionTrackerVector2InertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNaturalMotion<Impl: IInteractionTrackerVector2InertiaNaturalMotion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInteractionTrackerVector2InertiaNaturalMotionStatics_Impl: Sized {
    fn Create(&mut self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerVector2InertiaNaturalMotion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerVector2InertiaNaturalMotionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerVector2InertiaNaturalMotionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IInteractionTrackerVector2InertiaNaturalMotionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVisualInteractionSource_Impl: Sized {
    fn IsPositionXRailsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPositionXRailsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsPositionYRailsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPositionYRailsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ManipulationRedirectionMode(&mut self) -> ::windows::core::Result<VisualInteractionSourceRedirectionMode>;
    fn SetManipulationRedirectionMode(&mut self, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::Result<()>;
    fn PositionXChainingMode(&mut self) -> ::windows::core::Result<InteractionChainingMode>;
    fn SetPositionXChainingMode(&mut self, value: InteractionChainingMode) -> ::windows::core::Result<()>;
    fn PositionXSourceMode(&mut self) -> ::windows::core::Result<InteractionSourceMode>;
    fn SetPositionXSourceMode(&mut self, value: InteractionSourceMode) -> ::windows::core::Result<()>;
    fn PositionYChainingMode(&mut self) -> ::windows::core::Result<InteractionChainingMode>;
    fn SetPositionYChainingMode(&mut self, value: InteractionChainingMode) -> ::windows::core::Result<()>;
    fn PositionYSourceMode(&mut self) -> ::windows::core::Result<InteractionSourceMode>;
    fn SetPositionYSourceMode(&mut self, value: InteractionSourceMode) -> ::windows::core::Result<()>;
    fn ScaleChainingMode(&mut self) -> ::windows::core::Result<InteractionChainingMode>;
    fn SetScaleChainingMode(&mut self, value: InteractionChainingMode) -> ::windows::core::Result<()>;
    fn ScaleSourceMode(&mut self) -> ::windows::core::Result<InteractionSourceMode>;
    fn SetScaleSourceMode(&mut self, value: InteractionSourceMode) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<super::Visual>;
    fn TryRedirectForManipulation(&mut self, pointerpoint: &::core::option::Option<super::super::Input::PointerPoint>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource";
}
#[cfg(all(feature = "UI_Input", feature = "implement_exclusive"))]
impl IVisualInteractionSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSource_Vtbl {
        unsafe extern "system" fn IsPositionXRailsEnabled<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPositionXRailsEnabled<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPositionXRailsEnabled(value).into()
        }
        unsafe extern "system" fn IsPositionYRailsEnabled<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPositionYRailsEnabled<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPositionYRailsEnabled(value).into()
        }
        unsafe extern "system" fn ManipulationRedirectionMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetManipulationRedirectionMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManipulationRedirectionMode(value).into()
        }
        unsafe extern "system" fn PositionXChainingMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionXChainingMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionXChainingMode(value).into()
        }
        unsafe extern "system" fn PositionXSourceMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionXSourceMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionXSourceMode(value).into()
        }
        unsafe extern "system" fn PositionYChainingMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionYChainingMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionYChainingMode(value).into()
        }
        unsafe extern "system" fn PositionYSourceMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPositionYSourceMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionYSourceMode(value).into()
        }
        unsafe extern "system" fn ScaleChainingMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScaleChainingMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleChainingMode(value).into()
        }
        unsafe extern "system" fn ScaleSourceMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScaleSourceMode<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleSourceMode(value).into()
        }
        unsafe extern "system" fn Source<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryRedirectForManipulation<Impl: IVisualInteractionSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVisualInteractionSource2_Impl: Sized {
    fn DeltaPosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn DeltaScale(&mut self) -> ::windows::core::Result<f32>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn PositionVelocity(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Scale(&mut self) -> ::windows::core::Result<f32>;
    fn ScaleVelocity(&mut self) -> ::windows::core::Result<f32>;
    fn ConfigureCenterPointXModifiers(&mut self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureCenterPointYModifiers(&mut self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureDeltaPositionXModifiers(&mut self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureDeltaPositionYModifiers(&mut self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureDeltaScaleModifiers(&mut self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualInteractionSource2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IVisualInteractionSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSource2_Vtbl {
        unsafe extern "system" fn DeltaPosition<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeltaScale<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionVelocity<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scale<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScaleVelocity<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConfigureCenterPointXModifiers<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointXModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureCenterPointYModifiers<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureCenterPointYModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaPositionXModifiers<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaPositionXModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaPositionYModifiers<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureDeltaPositionYModifiers(&*(&conditionalvalues as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConfigureDeltaScaleModifiers<Impl: IVisualInteractionSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVisualInteractionSource3_Impl: Sized {
    fn PointerWheelConfig(&mut self) -> ::windows::core::Result<InteractionSourceConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSource3 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSource3";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSource3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSource3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSource3_Vtbl {
        unsafe extern "system" fn PointerWheelConfig<Impl: IVisualInteractionSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVisualInteractionSourceObjectFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSourceObjectFactory {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSourceObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSourceObjectFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceObjectFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceObjectFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualInteractionSourceObjectFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualInteractionSourceObjectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceStatics_Impl: Sized {
    fn Create(&mut self, source: &::core::option::Option<super::Visual>) -> ::windows::core::Result<VisualInteractionSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSourceStatics {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSourceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IVisualInteractionSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVisualInteractionSourceStatics2_Impl: Sized {
    fn CreateFromIVisualElement(&mut self, source: &::core::option::Option<super::IVisualElement>) -> ::windows::core::Result<VisualInteractionSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualInteractionSourceStatics2 {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IVisualInteractionSourceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualInteractionSourceStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualInteractionSourceStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualInteractionSourceStatics2_Vtbl {
        unsafe extern "system" fn CreateFromIVisualElement<Impl: IVisualInteractionSourceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
