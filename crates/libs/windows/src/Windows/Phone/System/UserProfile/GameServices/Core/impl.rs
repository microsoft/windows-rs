#[cfg(feature = "implement_exclusive")]
pub trait IGameServiceImpl: Sized {
    fn ServiceUri(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::Uri>;
    fn GetGamerProfileAsync(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>;
    fn GetInstalledGameItemsAsync(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>;
    fn GetPartnerTokenAsync(&self, audienceuri: &::core::option::Option<super::super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetPrivilegesAsync(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GrantAchievement(&self, achievementid: u32) -> ::windows::core::Result<()>;
    fn GrantAvatarAward(&self, avatarawardid: u32) -> ::windows::core::Result<()>;
    fn PostResult(&self, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: &::core::option::Option<super::super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameService2Impl: Sized {
    fn NotifyPartnerTokenExpired(&self, audienceuri: &::core::option::Option<super::super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn GetAuthenticationStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameServicePropertyCollectionImpl: Sized {
    fn GetPropertyAsync(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
}
