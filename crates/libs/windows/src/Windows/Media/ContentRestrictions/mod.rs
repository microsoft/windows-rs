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
unsafe impl ::windows::core::Abi for ContentAccessRestrictionLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContentAccessRestrictionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentAccessRestrictionLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentAccessRestrictionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(::windows::core::IUnknown);
impl ContentRestrictionsBrowsePolicy {
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GeographicRegion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBrowsableAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxBrowsableAgeRating)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreferredAgeRating)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for ContentRestrictionsBrowsePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ContentRestrictionsBrowsePolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy;{8c0133a4-442e-461a-8757-fad2f5bd37e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicy_Vtbl;
    const IID: ::windows::core::GUID = <IContentRestrictionsBrowsePolicy as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentRestrictionsBrowsePolicy {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy";
}
impl ::core::convert::From<ContentRestrictionsBrowsePolicy> for ::windows::core::IUnknown {
    fn from(value: ContentRestrictionsBrowsePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentRestrictionsBrowsePolicy> for ::windows::core::IUnknown {
    fn from(value: &ContentRestrictionsBrowsePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContentRestrictionsBrowsePolicy> for &::windows::core::IUnknown {
    fn from(value: &ContentRestrictionsBrowsePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContentRestrictionsBrowsePolicy> for ::windows::core::IInspectable {
    fn from(value: ContentRestrictionsBrowsePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentRestrictionsBrowsePolicy> for ::windows::core::IInspectable {
    fn from(value: &ContentRestrictionsBrowsePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContentRestrictionsBrowsePolicy> for &::windows::core::IInspectable {
    fn from(value: &ContentRestrictionsBrowsePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ContentRestrictionsBrowsePolicy {}
unsafe impl ::core::marker::Sync for ContentRestrictionsBrowsePolicy {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c0133a4_442e_461a_8757_fad2f5bd37e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentRestrictionsBrowsePolicy_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxBrowsableAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBrowsableAgeRating: usize,
    #[cfg(feature = "Foundation")]
    pub PreferredAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredAgeRating: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentDescription {
    type Vtable = IRatedContentDescription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x694866df_66b2_4dc3_96b1_f090eedee255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescription_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RatedContentCategory) -> ::windows::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RatedContentCategory) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Ratings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ratings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRatings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRatings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentDescriptionFactory {
    type Vtable = IRatedContentDescriptionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e38df62_9b90_4fa6_89c1_4b8d2ffb3573);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescriptionFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: RatedContentCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentRestrictions {
    type Vtable = IRatedContentRestrictions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f7f23cb_ba07_4401_a49d_8b9222205723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictions_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetBrowsePolicyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBrowsePolicyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRestrictionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRestrictionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestContentAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestContentAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RestrictionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RestrictionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRestrictionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRestrictionsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentRestrictionsFactory {
    type Vtable = IRatedContentRestrictionsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb4b2996_c3bd_4910_9619_97cfd0694d56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateWithMaxAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxagerating: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for RatedContentCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for RatedContentCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RatedContentCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RatedContentCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.RatedContentCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct RatedContentDescription(::windows::core::IUnknown);
impl RatedContentDescription {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Image)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImage<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetImage)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Category(&self) -> ::windows::core::Result<RatedContentCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Category)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RatedContentCategory>(result__)
        }
    }
    pub fn SetCategory(&self, value: RatedContentCategory) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCategory)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ratings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ratings)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRatings<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRatings)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Create(id: &::windows::core::HSTRING, title: &::windows::core::HSTRING, category: RatedContentCategory) -> ::windows::core::Result<RatedContentDescription> {
        Self::IRatedContentDescriptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(title), category, result__.as_mut_ptr()).from_abi::<RatedContentDescription>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRatedContentDescriptionFactory<R, F: FnOnce(&IRatedContentDescriptionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RatedContentDescription, IRatedContentDescriptionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RatedContentDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for RatedContentDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentDescription;{694866df-66b2-4dc3-96b1-f090eedee255})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RatedContentDescription {
    type Vtable = IRatedContentDescription_Vtbl;
    const IID: ::windows::core::GUID = <IRatedContentDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RatedContentDescription {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentDescription";
}
impl ::core::convert::From<RatedContentDescription> for ::windows::core::IUnknown {
    fn from(value: RatedContentDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentDescription> for ::windows::core::IUnknown {
    fn from(value: &RatedContentDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RatedContentDescription> for &::windows::core::IUnknown {
    fn from(value: &RatedContentDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RatedContentDescription> for ::windows::core::IInspectable {
    fn from(value: RatedContentDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentDescription> for ::windows::core::IInspectable {
    fn from(value: &RatedContentDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RatedContentDescription> for &::windows::core::IInspectable {
    fn from(value: &RatedContentDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for RatedContentDescription {}
unsafe impl ::core::marker::Sync for RatedContentDescription {}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct RatedContentRestrictions(::windows::core::IUnknown);
impl RatedContentRestrictions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RatedContentRestrictions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetBrowsePolicyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetBrowsePolicyAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRestrictionLevelAsync<'a, P0>(&self, ratedcontentdescription: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, RatedContentDescription>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetRestrictionLevelAsync)(::windows::core::Interface::as_raw(this), ratedcontentdescription.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestContentAccessAsync<'a, P0>(&self, ratedcontentdescription: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, RatedContentDescription>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestContentAccessAsync)(::windows::core::Interface::as_raw(this), ratedcontentdescription.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RestrictionsChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RestrictionsChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRestrictionsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRestrictionsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateWithMaxAgeRating(maxagerating: u32) -> ::windows::core::Result<RatedContentRestrictions> {
        Self::IRatedContentRestrictionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithMaxAgeRating)(::windows::core::Interface::as_raw(this), maxagerating, result__.as_mut_ptr()).from_abi::<RatedContentRestrictions>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRatedContentRestrictionsFactory<R, F: FnOnce(&IRatedContentRestrictionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RatedContentRestrictions, IRatedContentRestrictionsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RatedContentRestrictions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for RatedContentRestrictions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentRestrictions;{3f7f23cb-ba07-4401-a49d-8b9222205723})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RatedContentRestrictions {
    type Vtable = IRatedContentRestrictions_Vtbl;
    const IID: ::windows::core::GUID = <IRatedContentRestrictions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RatedContentRestrictions {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentRestrictions";
}
impl ::core::convert::From<RatedContentRestrictions> for ::windows::core::IUnknown {
    fn from(value: RatedContentRestrictions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentRestrictions> for ::windows::core::IUnknown {
    fn from(value: &RatedContentRestrictions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RatedContentRestrictions> for &::windows::core::IUnknown {
    fn from(value: &RatedContentRestrictions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RatedContentRestrictions> for ::windows::core::IInspectable {
    fn from(value: RatedContentRestrictions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentRestrictions> for ::windows::core::IInspectable {
    fn from(value: &RatedContentRestrictions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RatedContentRestrictions> for &::windows::core::IInspectable {
    fn from(value: &RatedContentRestrictions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for RatedContentRestrictions {}
unsafe impl ::core::marker::Sync for RatedContentRestrictions {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
