#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_SocialInfo_Provider")]
pub mod Provider;
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialFeedChildItem(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialFeedChildItem {
    type Vtable = ISocialFeedChildItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b6a985a_d59d_40be_980c_488a2ab30a83);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedChildItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialFeedContent(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialFeedContent {
    type Vtable = ISocialFeedContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa234e429_3e39_494d_a37c_f462a2494514);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialFeedItem(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialFeedItem {
    type Vtable = ISocialFeedItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f1392ab_1f72_4d33_b695_de3e1db60317);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocialItemBadgeStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocialItemBadgeStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocialFeedItemStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocialFeedItemStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialFeedSharedItem(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialFeedSharedItem {
    type Vtable = ISocialFeedSharedItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bfb9e40_a6aa_45a7_9ff6_54c42105dd1f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedSharedItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialItemThumbnail(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialItemThumbnail {
    type Vtable = ISocialItemThumbnailVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cbf831a_3f08_497f_917f_57e09d84b141);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialItemThumbnailVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialUserInfo(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialUserInfo {
    type Vtable = ISocialUserInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e5e1bd1_90d0_4e1d_9554_844d46607f61);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialUserInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
);
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedChildItem(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialFeedChildItem {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocialFeedChildItem, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Author(&self) -> ::windows::core::Result<SocialUserInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialUserInfo>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn PrimaryContent(&self) -> ::windows::core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SecondaryContent(&self) -> ::windows::core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTargetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation_Collections', 'deprecated'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Thumbnails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SocialItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SocialItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SharedItem(&self) -> ::windows::core::Result<SocialFeedSharedItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedSharedItem>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSharedItem<'a, Param0: ::windows::core::IntoParam<'a, SocialFeedSharedItem>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedChildItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedChildItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedChildItem {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedChildItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedChildItem").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedChildItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedChildItem;{0b6a985a-d59d-40be-980c-488a2ab30a83})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialFeedChildItem {
    type Vtable = ISocialFeedChildItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b6a985a_d59d_40be_980c_488a2ab30a83);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialFeedChildItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedChildItem";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedChildItem> for ::windows::core::IUnknown {
    fn from(value: SocialFeedChildItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedChildItem> for ::windows::core::IUnknown {
    fn from(value: &SocialFeedChildItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialFeedChildItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocialFeedChildItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedChildItem> for ::windows::core::IInspectable {
    fn from(value: SocialFeedChildItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedChildItem> for ::windows::core::IInspectable {
    fn from(value: &SocialFeedChildItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialFeedChildItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocialFeedChildItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialFeedChildItem {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialFeedChildItem {}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedContent(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialFeedContent {
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTargetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedContent {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedContent").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedContent;{a234e429-3e39-494d-a37c-f462a2494514})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialFeedContent {
    type Vtable = ISocialFeedContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa234e429_3e39_494d_a37c_f462a2494514);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialFeedContent {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedContent";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedContent> for ::windows::core::IUnknown {
    fn from(value: SocialFeedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedContent> for ::windows::core::IUnknown {
    fn from(value: &SocialFeedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialFeedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocialFeedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedContent> for ::windows::core::IInspectable {
    fn from(value: SocialFeedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedContent> for ::windows::core::IInspectable {
    fn from(value: &SocialFeedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialFeedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocialFeedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialFeedContent {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialFeedContent {}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedItem(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialFeedItem {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocialFeedItem, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Author(&self) -> ::windows::core::Result<SocialUserInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialUserInfo>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn PrimaryContent(&self) -> ::windows::core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SecondaryContent(&self) -> ::windows::core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTargetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation_Collections', 'deprecated'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Thumbnails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SocialItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SocialItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SharedItem(&self) -> ::windows::core::Result<SocialFeedSharedItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedSharedItem>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSharedItem<'a, Param0: ::windows::core::IntoParam<'a, SocialFeedSharedItem>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn BadgeStyle(&self) -> ::windows::core::Result<SocialItemBadgeStyle> {
        let this = self;
        unsafe {
            let mut result__: SocialItemBadgeStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialItemBadgeStyle>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetBadgeStyle(&self, value: SocialItemBadgeStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn BadgeCountValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetBadgeCountValue(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ChildItem(&self) -> ::windows::core::Result<SocialFeedChildItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedChildItem>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetChildItem<'a, Param0: ::windows::core::IntoParam<'a, SocialFeedChildItem>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Style(&self) -> ::windows::core::Result<SocialFeedItemStyle> {
        let this = self;
        unsafe {
            let mut result__: SocialFeedItemStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedItemStyle>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetStyle(&self, value: SocialFeedItemStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedItem {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedItem").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedItem;{4f1392ab-1f72-4d33-b695-de3e1db60317})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialFeedItem {
    type Vtable = ISocialFeedItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f1392ab_1f72_4d33_b695_de3e1db60317);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialFeedItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedItem";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedItem> for ::windows::core::IUnknown {
    fn from(value: SocialFeedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedItem> for ::windows::core::IUnknown {
    fn from(value: &SocialFeedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialFeedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocialFeedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedItem> for ::windows::core::IInspectable {
    fn from(value: SocialFeedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedItem> for ::windows::core::IInspectable {
    fn from(value: &SocialFeedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialFeedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocialFeedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialFeedItem {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialFeedItem {}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedItemStyle(pub i32);
#[cfg(feature = "deprecated")]
impl SocialFeedItemStyle {
    pub const Default: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialFeedItemStyle {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedItemStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SocialFeedItemStyle {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedItemStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedItemStyle {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedItemStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedItemStyle").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedItemStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialFeedItemStyle;i4)");
}
#[cfg(feature = "deprecated")]
impl ::windows::core::DefaultType for SocialFeedItemStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedKind(pub i32);
#[cfg(feature = "deprecated")]
impl SocialFeedKind {
    pub const HomeFeed: Self = Self(0i32);
    pub const ContactFeed: Self = Self(1i32);
    pub const Dashboard: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialFeedKind {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SocialFeedKind {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedKind {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedKind").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialFeedKind;i4)");
}
#[cfg(feature = "deprecated")]
impl ::windows::core::DefaultType for SocialFeedKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedSharedItem(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialFeedSharedItem {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocialFeedSharedItem, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn OriginalSource(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetOriginalSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Content(&self) -> ::windows::core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTargetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, SocialItemThumbnail>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<SocialItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SocialItemThumbnail>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedSharedItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedSharedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedSharedItem {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedSharedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedSharedItem").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedSharedItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem;{7bfb9e40-a6aa-45a7-9ff6-54c42105dd1f})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialFeedSharedItem {
    type Vtable = ISocialFeedSharedItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bfb9e40_a6aa_45a7_9ff6_54c42105dd1f);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialFeedSharedItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedSharedItem> for ::windows::core::IUnknown {
    fn from(value: SocialFeedSharedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedSharedItem> for ::windows::core::IUnknown {
    fn from(value: &SocialFeedSharedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialFeedSharedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocialFeedSharedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedSharedItem> for ::windows::core::IInspectable {
    fn from(value: SocialFeedSharedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedSharedItem> for ::windows::core::IInspectable {
    fn from(value: &SocialFeedSharedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialFeedSharedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocialFeedSharedItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialFeedSharedItem {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialFeedSharedItem {}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedUpdateMode(pub i32);
#[cfg(feature = "deprecated")]
impl SocialFeedUpdateMode {
    pub const Append: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialFeedUpdateMode {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedUpdateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SocialFeedUpdateMode {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedUpdateMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedUpdateMode {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedUpdateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedUpdateMode").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedUpdateMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialFeedUpdateMode;i4)");
}
#[cfg(feature = "deprecated")]
impl ::windows::core::DefaultType for SocialFeedUpdateMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialItemBadgeStyle(pub i32);
#[cfg(feature = "deprecated")]
impl SocialItemBadgeStyle {
    pub const Hidden: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const VisibleWithCount: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialItemBadgeStyle {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialItemBadgeStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SocialItemBadgeStyle {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialItemBadgeStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialItemBadgeStyle {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialItemBadgeStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialItemBadgeStyle").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialItemBadgeStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialItemBadgeStyle;i4)");
}
#[cfg(feature = "deprecated")]
impl ::windows::core::DefaultType for SocialItemBadgeStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialItemThumbnail(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialItemThumbnail {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocialItemThumbnail, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTargetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ImageUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetImageUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Graphics_Imaging', 'deprecated'*"]
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub fn BitmapSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Graphics_Imaging', 'deprecated'*"]
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub fn SetBitmapSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'Storage_Streams', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetImageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, image: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), image.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialItemThumbnail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialItemThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialItemThumbnail {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialItemThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialItemThumbnail").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialItemThumbnail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialItemThumbnail;{5cbf831a-3f08-497f-917f-57e09d84b141})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialItemThumbnail {
    type Vtable = ISocialItemThumbnailVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cbf831a_3f08_497f_917f_57e09d84b141);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialItemThumbnail {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialItemThumbnail";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialItemThumbnail> for ::windows::core::IUnknown {
    fn from(value: SocialItemThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialItemThumbnail> for ::windows::core::IUnknown {
    fn from(value: &SocialItemThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocialItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialItemThumbnail> for ::windows::core::IInspectable {
    fn from(value: SocialItemThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialItemThumbnail> for ::windows::core::IInspectable {
    fn from(value: &SocialItemThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocialItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialItemThumbnail {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialItemThumbnail {}
#[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialUserInfo(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialUserInfo {
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetUserName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_SocialInfo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTargetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialUserInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialUserInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialUserInfo {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialUserInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialUserInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialUserInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialUserInfo;{9e5e1bd1-90d0-4e1d-9554-844d46607f61})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialUserInfo {
    type Vtable = ISocialUserInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e5e1bd1_90d0_4e1d_9554_844d46607f61);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialUserInfo {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialUserInfo";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialUserInfo> for ::windows::core::IUnknown {
    fn from(value: SocialUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialUserInfo> for ::windows::core::IUnknown {
    fn from(value: &SocialUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SocialUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialUserInfo> for ::windows::core::IInspectable {
    fn from(value: SocialUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialUserInfo> for ::windows::core::IInspectable {
    fn from(value: &SocialUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SocialUserInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialUserInfo {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialUserInfo {}
