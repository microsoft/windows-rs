#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ContentAccessRestrictionLevel {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ContentAccessRestrictionLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentAccessRestrictionLevel {}
unsafe impl ::windows::core::RuntimeType for ContentAccessRestrictionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel;i4)");
}
impl ::windows::core::DefaultType for ContentAccessRestrictionLevel {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(::windows::core::IUnknown);
impl ContentRestrictionsBrowsePolicy {
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxBrowsableAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PreferredAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
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
unsafe impl ::windows::core::RuntimeType for ContentRestrictionsBrowsePolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy;{8c0133a4-442e-461a-8757-fad2f5bd37e4})");
}
unsafe impl ::windows::core::Interface for ContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c0133a4_442e_461a_8757_fad2f5bd37e4);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentRestrictionsBrowsePolicy {}
unsafe impl ::core::marker::Sync for ContentRestrictionsBrowsePolicy {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c0133a4_442e_461a_8757_fad2f5bd37e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentRestrictionsBrowsePolicyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentDescription {
    type Vtable = IRatedContentDescriptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x694866df_66b2_4dc3_96b1_f090eedee255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescriptionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RatedContentCategory) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RatedContentCategory) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentDescriptionFactory {
    type Vtable = IRatedContentDescriptionFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e38df62_9b90_4fa6_89c1_4b8d2ffb3573);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescriptionFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: RatedContentCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentRestrictions {
    type Vtable = IRatedContentRestrictionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f7f23cb_ba07_4401_a49d_8b9222205723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRatedContentRestrictionsFactory {
    type Vtable = IRatedContentRestrictionsFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb4b2996_c3bd_4910_9619_97cfd0694d56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictionsFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxagerating: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for RatedContentCategory {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RatedContentCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RatedContentCategory {}
unsafe impl ::windows::core::RuntimeType for RatedContentCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.RatedContentCategory;i4)");
}
impl ::windows::core::DefaultType for RatedContentCategory {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct RatedContentDescription(::windows::core::IUnknown);
impl RatedContentDescription {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Category(&self) -> ::windows::core::Result<RatedContentCategory> {
        let this = self;
        unsafe {
            let mut result__: RatedContentCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RatedContentCategory>(result__)
        }
    }
    pub fn SetCategory(&self, value: RatedContentCategory) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ratings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRatings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0, title: Param1, category: RatedContentCategory) -> ::windows::core::Result<RatedContentDescription> {
        Self::IRatedContentDescriptionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), title.into_param().abi(), category, &mut result__).from_abi::<RatedContentDescription>(result__)
        })
    }
    pub fn IRatedContentDescriptionFactory<R, F: FnOnce(&IRatedContentDescriptionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RatedContentDescription, IRatedContentDescriptionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::RuntimeType for RatedContentDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentDescription;{694866df-66b2-4dc3-96b1-f090eedee255})");
}
unsafe impl ::windows::core::Interface for RatedContentDescription {
    type Vtable = IRatedContentDescriptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x694866df_66b2_4dc3_96b1_f090eedee255);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RatedContentDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RatedContentDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RatedContentDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RatedContentDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RatedContentDescription {}
unsafe impl ::core::marker::Sync for RatedContentDescription {}
#[repr(transparent)]
pub struct RatedContentRestrictions(::windows::core::IUnknown);
impl RatedContentRestrictions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RatedContentRestrictions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetBrowsePolicyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRestrictionLevelAsync<'a, Param0: ::windows::core::IntoParam<'a, RatedContentDescription>>(&self, ratedcontentdescription: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ratedcontentdescription.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestContentAccessAsync<'a, Param0: ::windows::core::IntoParam<'a, RatedContentDescription>>(&self, ratedcontentdescription: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ratedcontentdescription.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RestrictionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRestrictionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CreateWithMaxAgeRating(maxagerating: u32) -> ::windows::core::Result<RatedContentRestrictions> {
        Self::IRatedContentRestrictionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), maxagerating, &mut result__).from_abi::<RatedContentRestrictions>(result__)
        })
    }
    pub fn IRatedContentRestrictionsFactory<R, F: FnOnce(&IRatedContentRestrictionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RatedContentRestrictions, IRatedContentRestrictionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::RuntimeType for RatedContentRestrictions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentRestrictions;{3f7f23cb-ba07-4401-a49d-8b9222205723})");
}
unsafe impl ::windows::core::Interface for RatedContentRestrictions {
    type Vtable = IRatedContentRestrictionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f7f23cb_ba07_4401_a49d_8b9222205723);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RatedContentRestrictions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RatedContentRestrictions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RatedContentRestrictions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RatedContentRestrictions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RatedContentRestrictions {}
unsafe impl ::core::marker::Sync for RatedContentRestrictions {}
