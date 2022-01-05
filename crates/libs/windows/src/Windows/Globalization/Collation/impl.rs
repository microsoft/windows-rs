#[cfg(feature = "implement_exclusive")]
pub trait ICharacterGroupingImpl: Sized {
    fn First(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICharacterGroupingsImpl: Sized + IIterableImpl<CharacterGrouping> + IVectorViewImpl<CharacterGrouping> {
    fn Lookup(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICharacterGroupingsFactoryImpl: Sized {
    fn Create(&self, language: &::windows::core::HSTRING) -> ::windows::core::Result<CharacterGroupings>;
}
