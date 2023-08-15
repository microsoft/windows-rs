#[doc(hidden)]
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicy_Vtbl;
}
impl ::core::clone::Clone for IContentRestrictionsBrowsePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContentRestrictionsBrowsePolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c0133a4_442e_461a_8757_fad2f5bd37e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentRestrictionsBrowsePolicy_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxBrowsableAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBrowsableAgeRating: usize,
    #[cfg(feature = "Foundation")]
    pub PreferredAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredAgeRating: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentDescription {
    type Vtable = IRatedContentDescription_Vtbl;
}
impl ::core::clone::Clone for IRatedContentDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRatedContentDescription {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x694866df_66b2_4dc3_96b1_f090eedee255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescription_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RatedContentCategory) -> ::windows_core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RatedContentCategory) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Ratings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ratings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRatings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRatings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentDescriptionFactory {
    type Vtable = IRatedContentDescriptionFactory_Vtbl;
}
impl ::core::clone::Clone for IRatedContentDescriptionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRatedContentDescriptionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e38df62_9b90_4fa6_89c1_4b8d2ffb3573);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescriptionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, title: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: RatedContentCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentRestrictions {
    type Vtable = IRatedContentRestrictions_Vtbl;
}
impl ::core::clone::Clone for IRatedContentRestrictions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRatedContentRestrictions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f7f23cb_ba07_4401_a49d_8b9222205723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetBrowsePolicyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBrowsePolicyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRestrictionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRestrictionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestContentAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestContentAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RestrictionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RestrictionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRestrictionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRestrictionsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentRestrictionsFactory {
    type Vtable = IRatedContentRestrictionsFactory_Vtbl;
}
impl ::core::clone::Clone for IRatedContentRestrictionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRatedContentRestrictionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb4b2996_c3bd_4910_9619_97cfd0694d56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithMaxAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxagerating: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(::windows_core::IUnknown);
impl ContentRestrictionsBrowsePolicy {
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBrowsableAgeRating(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrowsableAgeRating)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredAgeRating(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredAgeRating)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContentRestrictionsBrowsePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentRestrictionsBrowsePolicy {}
impl ::core::fmt::Debug for ContentRestrictionsBrowsePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentRestrictionsBrowsePolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContentRestrictionsBrowsePolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy;{8c0133a4-442e-461a-8757-fad2f5bd37e4})");
}
impl ::core::clone::Clone for ContentRestrictionsBrowsePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentRestrictionsBrowsePolicy {
    const IID: ::windows_core::GUID = <IContentRestrictionsBrowsePolicy as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentRestrictionsBrowsePolicy {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy";
}
::windows_core::imp::interface_hierarchy!(ContentRestrictionsBrowsePolicy, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContentRestrictionsBrowsePolicy {}
unsafe impl ::core::marker::Sync for ContentRestrictionsBrowsePolicy {}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct RatedContentDescription(::windows_core::IUnknown);
impl RatedContentDescription {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Image)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImage<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImage)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Category(&self) -> ::windows_core::Result<RatedContentCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCategory(&self, value: RatedContentCategory) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ratings(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ratings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRatings<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRatings)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Create(id: &::windows_core::HSTRING, title: &::windows_core::HSTRING, category: RatedContentCategory) -> ::windows_core::Result<RatedContentDescription> {
        Self::IRatedContentDescriptionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(title), category, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRatedContentDescriptionFactory<R, F: FnOnce(&IRatedContentDescriptionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RatedContentDescription, IRatedContentDescriptionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for RatedContentDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RatedContentDescription {}
impl ::core::fmt::Debug for RatedContentDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RatedContentDescription").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RatedContentDescription {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentDescription;{694866df-66b2-4dc3-96b1-f090eedee255})");
}
impl ::core::clone::Clone for RatedContentDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RatedContentDescription {
    type Vtable = IRatedContentDescription_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RatedContentDescription {
    const IID: ::windows_core::GUID = <IRatedContentDescription as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RatedContentDescription {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentDescription";
}
::windows_core::imp::interface_hierarchy!(RatedContentDescription, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RatedContentDescription {}
unsafe impl ::core::marker::Sync for RatedContentDescription {}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct RatedContentRestrictions(::windows_core::IUnknown);
impl RatedContentRestrictions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RatedContentRestrictions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetBrowsePolicyAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBrowsePolicyAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRestrictionLevelAsync<P0>(&self, ratedcontentdescription: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>
    where
        P0: ::windows_core::IntoParam<RatedContentDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRestrictionLevelAsync)(::windows_core::Interface::as_raw(this), ratedcontentdescription.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestContentAccessAsync<P0>(&self, ratedcontentdescription: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<RatedContentDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestContentAccessAsync)(::windows_core::Interface::as_raw(this), ratedcontentdescription.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RestrictionsChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RestrictionsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRestrictionsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRestrictionsChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateWithMaxAgeRating(maxagerating: u32) -> ::windows_core::Result<RatedContentRestrictions> {
        Self::IRatedContentRestrictionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMaxAgeRating)(::windows_core::Interface::as_raw(this), maxagerating, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRatedContentRestrictionsFactory<R, F: FnOnce(&IRatedContentRestrictionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RatedContentRestrictions, IRatedContentRestrictionsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for RatedContentRestrictions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RatedContentRestrictions {}
impl ::core::fmt::Debug for RatedContentRestrictions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RatedContentRestrictions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RatedContentRestrictions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentRestrictions;{3f7f23cb-ba07-4401-a49d-8b9222205723})");
}
impl ::core::clone::Clone for RatedContentRestrictions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RatedContentRestrictions {
    type Vtable = IRatedContentRestrictions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RatedContentRestrictions {
    const IID: ::windows_core::GUID = <IRatedContentRestrictions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RatedContentRestrictions {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentRestrictions";
}
::windows_core::imp::interface_hierarchy!(RatedContentRestrictions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RatedContentRestrictions {}
unsafe impl ::core::marker::Sync for RatedContentRestrictions {}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContentAccessRestrictionLevel(pub i32);
impl ContentAccessRestrictionLevel {
    pub const Allow: Self = Self(0i32);
    pub const Warn: Self = Self(1i32);
    pub const Block: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
}
impl ::core::marker::Copy for ContentAccessRestrictionLevel {}
impl ::core::clone::Clone for ContentAccessRestrictionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContentAccessRestrictionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ContentAccessRestrictionLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContentAccessRestrictionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentAccessRestrictionLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContentAccessRestrictionLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel;i4)");
}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RatedContentCategory(pub i32);
impl RatedContentCategory {
    pub const General: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Game: Self = Self(2i32);
    pub const Movie: Self = Self(3i32);
    pub const Television: Self = Self(4i32);
    pub const Music: Self = Self(5i32);
}
impl ::core::marker::Copy for RatedContentCategory {}
impl ::core::clone::Clone for RatedContentCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RatedContentCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RatedContentCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RatedContentCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RatedContentCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RatedContentCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.RatedContentCategory;i4)");
}
