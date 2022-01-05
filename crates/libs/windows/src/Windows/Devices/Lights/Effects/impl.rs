#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBitmapEffectImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetUpdateInterval(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SuggestedBitmapSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn BitmapRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBitmapRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBitmapEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBitmapEffect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBitmapRequestedEventArgsImpl: Sized {
    fn SinceStarted(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn UpdateBitmap(&self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBlinkEffectImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AttackDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetAttackDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SustainDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetSustainDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DecayDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDecayDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn RepetitionDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRepetitionDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Occurrences(&self) -> ::windows::core::Result<i32>;
    fn SetOccurrences(&self, value: i32) -> ::windows::core::Result<()>;
    fn RepetitionMode(&self) -> ::windows::core::Result<LampArrayRepetitionMode>;
    fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBlinkEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBlinkEffect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayColorRampEffectImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn RampDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRampDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CompletionBehavior(&self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior>;
    fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayColorRampEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayColorRampEffect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayCustomEffectImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetUpdateInterval(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayCustomEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayCustomEffect>;
}
pub trait ILampArrayEffectImpl: Sized {
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayEffectPlaylistImpl: Sized {
    fn Append(&self, effect: &::core::option::Option<ILampArrayEffect>) -> ::windows::core::Result<()>;
    fn OverrideZIndex(&self, zindex: i32) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn EffectStartMode(&self) -> ::windows::core::Result<LampArrayEffectStartMode>;
    fn SetEffectStartMode(&self, value: LampArrayEffectStartMode) -> ::windows::core::Result<()>;
    fn Occurrences(&self) -> ::windows::core::Result<i32>;
    fn SetOccurrences(&self, value: i32) -> ::windows::core::Result<()>;
    fn RepetitionMode(&self) -> ::windows::core::Result<LampArrayRepetitionMode>;
    fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayEffectPlaylistStaticsImpl: Sized {
    fn StartAll(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
    fn StopAll(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
    fn PauseAll(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArraySolidEffectImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CompletionBehavior(&self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior>;
    fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArraySolidEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArraySolidEffect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayUpdateRequestedEventArgsImpl: Sized {
    fn SinceStarted(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetColor(&self, desiredcolor: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetColorForIndex(&self, lampindex: i32, desiredcolor: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetSingleColorForIndices(&self, desiredcolor: &super::super::super::UI::Color, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForIndices(&self, desiredcolors: &[<super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
