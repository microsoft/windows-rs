pub trait IUIAnimationInterpolatorImpl: Sized {
    fn SetInitialValueAndVelocity();
    fn SetDuration();
    fn GetDuration();
    fn GetFinalValue();
    fn InterpolateValue();
    fn InterpolateVelocity();
    fn GetDependencies();
}
pub trait IUIAnimationInterpolator2Impl: Sized {
    fn GetDimension();
    fn SetInitialValueAndVelocity();
    fn SetDuration();
    fn GetDuration();
    fn GetFinalValue();
    fn InterpolateValue();
    fn InterpolateVelocity();
    fn GetPrimitiveInterpolation();
    fn GetDependencies();
}
pub trait IUIAnimationLoopIterationChangeHandler2Impl: Sized {
    fn OnLoopIterationChanged();
}
pub trait IUIAnimationManagerImpl: Sized {
    fn CreateAnimationVariable();
    fn ScheduleTransition();
    fn CreateStoryboard();
    fn FinishAllStoryboards();
    fn AbandonAllStoryboards();
    fn Update();
    fn GetVariableFromTag();
    fn GetStoryboardFromTag();
    fn GetStatus();
    fn SetAnimationMode();
    fn Pause();
    fn Resume();
    fn SetManagerEventHandler();
    fn SetCancelPriorityComparison();
    fn SetTrimPriorityComparison();
    fn SetCompressPriorityComparison();
    fn SetConcludePriorityComparison();
    fn SetDefaultLongestAcceptableDelay();
    fn Shutdown();
}
pub trait IUIAnimationManager2Impl: Sized {
    fn CreateAnimationVectorVariable();
    fn CreateAnimationVariable();
    fn ScheduleTransition();
    fn CreateStoryboard();
    fn FinishAllStoryboards();
    fn AbandonAllStoryboards();
    fn Update();
    fn GetVariableFromTag();
    fn GetStoryboardFromTag();
    fn EstimateNextEventTime();
    fn GetStatus();
    fn SetAnimationMode();
    fn Pause();
    fn Resume();
    fn SetManagerEventHandler();
    fn SetCancelPriorityComparison();
    fn SetTrimPriorityComparison();
    fn SetCompressPriorityComparison();
    fn SetConcludePriorityComparison();
    fn SetDefaultLongestAcceptableDelay();
    fn Shutdown();
}
pub trait IUIAnimationManagerEventHandlerImpl: Sized {
    fn OnManagerStatusChanged();
}
pub trait IUIAnimationManagerEventHandler2Impl: Sized {
    fn OnManagerStatusChanged();
}
pub trait IUIAnimationPrimitiveInterpolationImpl: Sized {
    fn AddCubic();
    fn AddSinusoidal();
}
pub trait IUIAnimationPriorityComparisonImpl: Sized {
    fn HasPriority();
}
pub trait IUIAnimationPriorityComparison2Impl: Sized {
    fn HasPriority();
}
pub trait IUIAnimationStoryboardImpl: Sized {
    fn AddTransition();
    fn AddKeyframeAtOffset();
    fn AddKeyframeAfterTransition();
    fn AddTransitionAtKeyframe();
    fn AddTransitionBetweenKeyframes();
    fn RepeatBetweenKeyframes();
    fn HoldVariable();
    fn SetLongestAcceptableDelay();
    fn Schedule();
    fn Conclude();
    fn Finish();
    fn Abandon();
    fn SetTag();
    fn GetTag();
    fn GetStatus();
    fn GetElapsedTime();
    fn SetStoryboardEventHandler();
}
pub trait IUIAnimationStoryboard2Impl: Sized {
    fn AddTransition();
    fn AddKeyframeAtOffset();
    fn AddKeyframeAfterTransition();
    fn AddTransitionAtKeyframe();
    fn AddTransitionBetweenKeyframes();
    fn RepeatBetweenKeyframes();
    fn HoldVariable();
    fn SetLongestAcceptableDelay();
    fn SetSkipDuration();
    fn Schedule();
    fn Conclude();
    fn Finish();
    fn Abandon();
    fn SetTag();
    fn GetTag();
    fn GetStatus();
    fn GetElapsedTime();
    fn SetStoryboardEventHandler();
}
pub trait IUIAnimationStoryboardEventHandlerImpl: Sized {
    fn OnStoryboardStatusChanged();
    fn OnStoryboardUpdated();
}
pub trait IUIAnimationStoryboardEventHandler2Impl: Sized {
    fn OnStoryboardStatusChanged();
    fn OnStoryboardUpdated();
}
pub trait IUIAnimationTimerImpl: Sized {
    fn SetTimerUpdateHandler();
    fn SetTimerEventHandler();
    fn Enable();
    fn Disable();
    fn IsEnabled();
    fn GetTime();
    fn SetFrameRateThreshold();
}
pub trait IUIAnimationTimerClientEventHandlerImpl: Sized {
    fn OnTimerClientStatusChanged();
}
pub trait IUIAnimationTimerEventHandlerImpl: Sized {
    fn OnPreUpdate();
    fn OnPostUpdate();
    fn OnRenderingTooSlow();
}
pub trait IUIAnimationTimerUpdateHandlerImpl: Sized {
    fn OnUpdate();
    fn SetTimerClientEventHandler();
    fn ClearTimerClientEventHandler();
}
pub trait IUIAnimationTransitionImpl: Sized {
    fn SetInitialValue();
    fn SetInitialVelocity();
    fn IsDurationKnown();
    fn GetDuration();
}
pub trait IUIAnimationTransition2Impl: Sized {
    fn GetDimension();
    fn SetInitialValue();
    fn SetInitialVectorValue();
    fn SetInitialVelocity();
    fn SetInitialVectorVelocity();
    fn IsDurationKnown();
    fn GetDuration();
}
pub trait IUIAnimationTransitionFactoryImpl: Sized {
    fn CreateTransition();
}
pub trait IUIAnimationTransitionFactory2Impl: Sized {
    fn CreateTransition();
}
pub trait IUIAnimationTransitionLibraryImpl: Sized {
    fn CreateInstantaneousTransition();
    fn CreateConstantTransition();
    fn CreateDiscreteTransition();
    fn CreateLinearTransition();
    fn CreateLinearTransitionFromSpeed();
    fn CreateSinusoidalTransitionFromVelocity();
    fn CreateSinusoidalTransitionFromRange();
    fn CreateAccelerateDecelerateTransition();
    fn CreateReversalTransition();
    fn CreateCubicTransition();
    fn CreateSmoothStopTransition();
    fn CreateParabolicTransitionFromAcceleration();
}
pub trait IUIAnimationTransitionLibrary2Impl: Sized {
    fn CreateInstantaneousTransition();
    fn CreateInstantaneousVectorTransition();
    fn CreateConstantTransition();
    fn CreateDiscreteTransition();
    fn CreateDiscreteVectorTransition();
    fn CreateLinearTransition();
    fn CreateLinearVectorTransition();
    fn CreateLinearTransitionFromSpeed();
    fn CreateLinearVectorTransitionFromSpeed();
    fn CreateSinusoidalTransitionFromVelocity();
    fn CreateSinusoidalTransitionFromRange();
    fn CreateAccelerateDecelerateTransition();
    fn CreateReversalTransition();
    fn CreateCubicTransition();
    fn CreateCubicVectorTransition();
    fn CreateSmoothStopTransition();
    fn CreateParabolicTransitionFromAcceleration();
    fn CreateCubicBezierLinearTransition();
    fn CreateCubicBezierLinearVectorTransition();
}
pub trait IUIAnimationVariableImpl: Sized {
    fn GetValue();
    fn GetFinalValue();
    fn GetPreviousValue();
    fn GetIntegerValue();
    fn GetFinalIntegerValue();
    fn GetPreviousIntegerValue();
    fn GetCurrentStoryboard();
    fn SetLowerBound();
    fn SetUpperBound();
    fn SetRoundingMode();
    fn SetTag();
    fn GetTag();
    fn SetVariableChangeHandler();
    fn SetVariableIntegerChangeHandler();
}
pub trait IUIAnimationVariable2Impl: Sized {
    fn GetDimension();
    fn GetValue();
    fn GetVectorValue();
    fn GetCurve();
    fn GetVectorCurve();
    fn GetFinalValue();
    fn GetFinalVectorValue();
    fn GetPreviousValue();
    fn GetPreviousVectorValue();
    fn GetIntegerValue();
    fn GetIntegerVectorValue();
    fn GetFinalIntegerValue();
    fn GetFinalIntegerVectorValue();
    fn GetPreviousIntegerValue();
    fn GetPreviousIntegerVectorValue();
    fn GetCurrentStoryboard();
    fn SetLowerBound();
    fn SetLowerBoundVector();
    fn SetUpperBound();
    fn SetUpperBoundVector();
    fn SetRoundingMode();
    fn SetTag();
    fn GetTag();
    fn SetVariableChangeHandler();
    fn SetVariableIntegerChangeHandler();
    fn SetVariableCurveChangeHandler();
}
pub trait IUIAnimationVariableChangeHandlerImpl: Sized {
    fn OnValueChanged();
}
pub trait IUIAnimationVariableChangeHandler2Impl: Sized {
    fn OnValueChanged();
}
pub trait IUIAnimationVariableCurveChangeHandler2Impl: Sized {
    fn OnCurveChanged();
}
pub trait IUIAnimationVariableIntegerChangeHandlerImpl: Sized {
    fn OnIntegerValueChanged();
}
pub trait IUIAnimationVariableIntegerChangeHandler2Impl: Sized {
    fn OnIntegerValueChanged();
}
