#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ILampArrayBitmapEffect();
    fn ILampArrayBitmapEffectFactory();
    fn ILampArrayBitmapRequestedEventArgs();
    fn ILampArrayBlinkEffect();
    fn ILampArrayBlinkEffectFactory();
    fn ILampArrayColorRampEffect();
    fn ILampArrayColorRampEffectFactory();
    fn ILampArrayCustomEffect();
    fn ILampArrayCustomEffectFactory();
    fn ILampArrayEffect();
    fn ILampArrayEffectPlaylist();
    fn ILampArrayEffectPlaylistStatics();
    fn ILampArraySolidEffect();
    fn ILampArraySolidEffectFactory();
    fn ILampArrayUpdateRequestedEventArgs();
    fn LampArrayBitmapEffect();
    fn LampArrayBitmapRequestedEventArgs();
    fn LampArrayBlinkEffect();
    fn LampArrayColorRampEffect();
    fn LampArrayCustomEffect();
    fn LampArrayEffectCompletionBehavior();
    fn LampArrayEffectPlaylist();
    fn LampArrayEffectStartMode();
    fn LampArrayRepetitionMode();
    fn LampArraySolidEffect();
    fn LampArrayUpdateRequestedEventArgs();
}
