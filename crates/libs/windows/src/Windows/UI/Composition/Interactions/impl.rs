#[cfg(feature = "implement_exclusive")]
pub trait ICompositionConditionalValueImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetValue(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionConditionalValueStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<CompositionConditionalValue>;
}
pub trait ICompositionInteractionSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionInteractionSourceCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, value: &::core::option::Option<ICompositionInteractionSource>) -> ::windows::core::Result<()>;
    fn Remove(&self, value: &::core::option::Option<ICompositionInteractionSource>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
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
pub trait IInteractionTracker2Impl: Sized {
    fn ConfigureCenterPointXInertiaModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
    fn ConfigureCenterPointYInertiaModifiers(&self, conditionalvalues: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTracker3Impl: Sized {
    fn ConfigureVector2PositionInertiaModifiers(&self, modifiers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTracker4Impl: Sized {
    fn TryUpdatePositionWithOption(&self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn TryUpdatePositionByWithOption(&self, amount: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32>;
    fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTracker5Impl: Sized {
    fn TryUpdatePositionWithOption(&self, value: &super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerCustomAnimationStateEnteredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerCustomAnimationStateEnteredArgs2Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerIdleStateEnteredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerIdleStateEnteredArgs2Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaModifierImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaModifierFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaMotionImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn Motion(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetMotion(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaMotionStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaMotion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaNaturalMotionImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn NaturalMotion(&self) -> ::windows::core::Result<super::ScalarNaturalMotionAnimation>;
    fn SetNaturalMotion(&self, value: &::core::option::Option<super::ScalarNaturalMotionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaNaturalMotionStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaNaturalMotion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaRestingValueImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn RestingValue(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetRestingValue(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaRestingValueStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerInertiaRestingValue>;
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
pub trait IInteractionTrackerInertiaStateEnteredArgs2Impl: Sized {
    fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInertiaStateEnteredArgs3Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInteractingStateEnteredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerInteractingStateEnteredArgs2Impl: Sized {
    fn IsFromBinding(&self) -> ::windows::core::Result<bool>;
}
pub trait IInteractionTrackerOwnerImpl: Sized {
    fn CustomAnimationStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerCustomAnimationStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn IdleStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerIdleStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InertiaStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInertiaStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InteractingStateEntered(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInteractingStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn RequestIgnored(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerRequestIgnoredArgs>) -> ::windows::core::Result<()>;
    fn ValuesChanged(&self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerValuesChangedArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerRequestIgnoredArgsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTracker>;
    fn CreateWithOwner(&self, compositor: &::core::option::Option<super::Compositor>, owner: &::core::option::Option<IInteractionTrackerOwner>) -> ::windows::core::Result<InteractionTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerStatics2Impl: Sized {
    fn SetBindingMode(&self, boundtracker1: &::core::option::Option<InteractionTracker>, boundtracker2: &::core::option::Option<InteractionTracker>, axismode: InteractionBindingAxisModes) -> ::windows::core::Result<()>;
    fn GetBindingMode(&self, boundtracker1: &::core::option::Option<InteractionTracker>, boundtracker2: &::core::option::Option<InteractionTracker>) -> ::windows::core::Result<InteractionBindingAxisModes>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerValuesChangedArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn Scale(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaModifierImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaModifierFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaNaturalMotionImpl: Sized {
    fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation>;
    fn SetCondition(&self, value: &::core::option::Option<super::ExpressionAnimation>) -> ::windows::core::Result<()>;
    fn NaturalMotion(&self) -> ::windows::core::Result<super::Vector2NaturalMotionAnimation>;
    fn SetNaturalMotion(&self, value: &::core::option::Option<super::Vector2NaturalMotionAnimation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInteractionTrackerVector2InertiaNaturalMotionStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<InteractionTrackerVector2InertiaNaturalMotion>;
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
pub trait IVisualInteractionSource3Impl: Sized {
    fn PointerWheelConfig(&self) -> ::windows::core::Result<InteractionSourceConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceStaticsImpl: Sized {
    fn Create(&self, source: &::core::option::Option<super::Visual>) -> ::windows::core::Result<VisualInteractionSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualInteractionSourceStatics2Impl: Sized {
    fn CreateFromIVisualElement(&self, source: &::core::option::Option<super::IVisualElement>) -> ::windows::core::Result<VisualInteractionSource>;
}
