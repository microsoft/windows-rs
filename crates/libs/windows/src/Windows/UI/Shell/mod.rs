#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_Shell'*"]
pub struct AdaptiveCardBuilder {}
impl AdaptiveCardBuilder {
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<IAdaptiveCard> {
        Self::IAdaptiveCardBuilderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<IAdaptiveCard>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAdaptiveCardBuilderStatics<R, F: FnOnce(&IAdaptiveCardBuilderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AdaptiveCardBuilder, IAdaptiveCardBuilderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AdaptiveCardBuilder {
    const NAME: &'static str = "Windows.UI.Shell.AdaptiveCardBuilder";
}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct IAdaptiveCard(::windows::core::IUnknown);
impl IAdaptiveCard {
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAdaptiveCard> for ::windows::core::IInspectable {
    fn from(value: IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows::core::IInspectable {
    fn from(value: &IAdaptiveCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdaptiveCard> for ::windows::core::IUnknown {
    fn from(value: IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows::core::IUnknown {
    fn from(value: &IAdaptiveCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdaptiveCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveCard {}
impl ::core::fmt::Debug for IAdaptiveCard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveCard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveCard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72d0568c-a274-41cd-82a8-989d40b9b05e}");
}
unsafe impl ::windows::core::Interface for IAdaptiveCard {
    type Vtable = IAdaptiveCardVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d0568c_a274_41cd_82a8_989d40b9b05e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCardVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct IAdaptiveCardBuilderStatics(::windows::core::IUnknown);
impl IAdaptiveCardBuilderStatics {
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<IAdaptiveCard>(result__)
        }
    }
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows::core::IInspectable {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows::core::IInspectable {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows::core::IUnknown {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows::core::IUnknown {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdaptiveCardBuilderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveCardBuilderStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveCardBuilderStatics {}
impl ::core::fmt::Debug for IAdaptiveCardBuilderStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveCardBuilderStatics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveCardBuilderStatics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{766d8f08-d3fe-4347-a0bc-b9ea9a6dc28e}");
}
unsafe impl ::windows::core::Interface for IAdaptiveCardBuilderStatics {
    type Vtable = IAdaptiveCardBuilderStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x766d8f08_d3fe_4347_a0bc_b9ea9a6dc28e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCardBuilderStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecurityAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISecurityAppManager {
    type Vtable = ISecurityAppManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96ac500c_aed4_561d_bde8_953520343a2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityAppManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, detailsuri: ::windows::core::RawPtr, registerperuser: bool, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4578dc09_a523_5756_a995_e4feb991fff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WindowId) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ShareWindowCommand) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ShareWindowCommand) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareWindowCommandSource {
    type Vtable = IShareWindowCommandSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShareWindowCommandSourceStatics {
    type Vtable = IShareWindowCommandSourceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0eb6656_9cac_517c_b6c7_8ef715084295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManager {
    type Vtable = ITaskbarManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManager2 {
    type Vtable = ITaskbarManager2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f0a06e_7b02_4911_918c_dee0bbd20ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, secondarytile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_StartScreen")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITaskbarManagerStatics {
    type Vtable = ITaskbarManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb32ab74_de52_4fe6_b7b6_95ff9f8395df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: Self = Self(0i32);
}
impl ::core::marker::Copy for SecurityAppKind {}
impl ::core::clone::Clone for SecurityAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecurityAppKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecurityAppKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecurityAppKind {}
impl ::core::fmt::Debug for SecurityAppKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppKind;i4)");
}
impl ::windows::core::DefaultType for SecurityAppKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct SecurityAppManager(::windows::core::IUnknown);
impl SecurityAppManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecurityAppManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Register<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, kind: SecurityAppKind, displayname: Param1, detailsuri: Param2, registerperuser: bool) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind, displayname.into_param().abi(), detailsuri.into_param().abi(), registerperuser, &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn Unregister<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, kind: SecurityAppKind, guidregistration: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), kind, guidregistration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateState<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, kind: SecurityAppKind, guidregistration: Param1, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: Param4) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), kind, guidregistration.into_param().abi(), state, substatus, detailsuri.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SecurityAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecurityAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecurityAppManager {}
impl ::core::fmt::Debug for SecurityAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.SecurityAppManager;{96ac500c-aed4-561d-bde8-953520343a2d})");
}
unsafe impl ::windows::core::Interface for SecurityAppManager {
    type Vtable = ISecurityAppManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96ac500c_aed4_561d_bde8_953520343a2d);
}
impl ::windows::core::RuntimeName for SecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.SecurityAppManager";
}
impl ::core::convert::From<SecurityAppManager> for ::windows::core::IUnknown {
    fn from(value: SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows::core::IUnknown {
    fn from(value: &SecurityAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecurityAppManager> for ::windows::core::IInspectable {
    fn from(value: SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows::core::IInspectable {
    fn from(value: &SecurityAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecurityAppManager {}
unsafe impl ::core::marker::Sync for SecurityAppManager {}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for SecurityAppState {}
impl ::core::clone::Clone for SecurityAppState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecurityAppState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecurityAppState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecurityAppState {}
impl ::core::fmt::Debug for SecurityAppState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppState;i4)");
}
impl ::windows::core::DefaultType for SecurityAppState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: Self = Self(0i32);
    pub const NoActionNeeded: Self = Self(1i32);
    pub const ActionRecommended: Self = Self(2i32);
    pub const ActionNeeded: Self = Self(3i32);
}
impl ::core::marker::Copy for SecurityAppSubstatus {}
impl ::core::clone::Clone for SecurityAppSubstatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SecurityAppSubstatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SecurityAppSubstatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecurityAppSubstatus {}
impl ::core::fmt::Debug for SecurityAppSubstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppSubstatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppSubstatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppSubstatus;i4)");
}
impl ::windows::core::DefaultType for SecurityAppSubstatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: Self = Self(0i32);
    pub const StartSharing: Self = Self(1i32);
    pub const StopSharing: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareWindowCommand {}
impl ::core::clone::Clone for ShareWindowCommand {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ShareWindowCommand {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ShareWindowCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareWindowCommand {}
impl ::core::fmt::Debug for ShareWindowCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.ShareWindowCommand;i4)");
}
impl ::windows::core::DefaultType for ShareWindowCommand {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct ShareWindowCommandEventArgs(::windows::core::IUnknown);
impl ShareWindowCommandEventArgs {
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn WindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn Command(&self) -> ::windows::core::Result<ShareWindowCommand> {
        let this = self;
        unsafe {
            let mut result__: ShareWindowCommand = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ShareWindowCommand>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn SetCommand(&self, value: ShareWindowCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for ShareWindowCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareWindowCommandEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareWindowCommandEventArgs {}
impl ::core::fmt::Debug for ShareWindowCommandEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommandEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommandEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandEventArgs;{4578dc09-a523-5756-a995-e4feb991fff0})");
}
unsafe impl ::windows::core::Interface for ShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4578dc09_a523_5756_a995_e4feb991fff0);
}
impl ::windows::core::RuntimeName for ShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandEventArgs";
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandEventArgs {}
unsafe impl ::core::marker::Sync for ShareWindowCommandEventArgs {}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct ShareWindowCommandSource(::windows::core::IUnknown);
impl ShareWindowCommandSource {
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn ReportCommandChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CommandRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCommandRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CommandInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCommandInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<ShareWindowCommandSource> {
        Self::IShareWindowCommandSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ShareWindowCommandSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IShareWindowCommandSourceStatics<R, F: FnOnce(&IShareWindowCommandSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ShareWindowCommandSource, IShareWindowCommandSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ShareWindowCommandSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareWindowCommandSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareWindowCommandSource {}
impl ::core::fmt::Debug for ShareWindowCommandSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommandSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommandSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandSource;{cb3b7ae3-6b9c-561e-bccc-61e68e0abfef})");
}
unsafe impl ::windows::core::Interface for ShareWindowCommandSource {
    type Vtable = IShareWindowCommandSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
}
impl ::windows::core::RuntimeName for ShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandSource";
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows::core::IUnknown {
    fn from(value: ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows::core::IInspectable {
    fn from(value: ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandSource {}
unsafe impl ::core::marker::Sync for ShareWindowCommandSource {}
#[doc = "*Required features: 'UI_Shell'*"]
#[repr(transparent)]
pub struct TaskbarManager(::windows::core::IUnknown);
impl TaskbarManager {
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn IsPinningAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IsCurrentAppPinnedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'ApplicationModel_Core', 'Foundation'*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn IsAppListEntryPinnedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestPinCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'ApplicationModel_Core', 'Foundation'*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestPinAppListEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSecondaryTilePinnedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation', 'UI_StartScreen'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    pub fn RequestPinSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::StartScreen::SecondaryTile>>(&self, secondarytile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), secondarytile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUnpinSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Shell'*"]
    pub fn GetDefault() -> ::windows::core::Result<TaskbarManager> {
        Self::ITaskbarManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TaskbarManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITaskbarManagerStatics<R, F: FnOnce(&ITaskbarManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TaskbarManager, ITaskbarManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TaskbarManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TaskbarManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TaskbarManager {}
impl ::core::fmt::Debug for TaskbarManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TaskbarManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TaskbarManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.TaskbarManager;{87490a19-1ad9-49f4-b2e8-86738dc5ac40})");
}
unsafe impl ::windows::core::Interface for TaskbarManager {
    type Vtable = ITaskbarManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
}
impl ::windows::core::RuntimeName for TaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.TaskbarManager";
}
impl ::core::convert::From<TaskbarManager> for ::windows::core::IUnknown {
    fn from(value: TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows::core::IUnknown {
    fn from(value: &TaskbarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TaskbarManager> for ::windows::core::IInspectable {
    fn from(value: TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows::core::IInspectable {
    fn from(value: &TaskbarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TaskbarManager {}
unsafe impl ::core::marker::Sync for TaskbarManager {}
