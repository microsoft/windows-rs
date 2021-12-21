#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBar {
    type Vtable = IWebUICommandBarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4fc0016_dbe5_41ad_8d7b_14698bd6911d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebUICommandBarClosedDisplayMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIcon(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIconVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858f4f45_08d8_4a46_81ec_00015b0b1c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIconVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIconFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarBitmapIconFactory {
    type Vtable = IWebUICommandBarBitmapIconFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3f7d78a_7673_444a_be62_ac12d31c2231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIconFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarConfirmationButton(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButtonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e7824a_e3d5_4eb6_b2ff_8f018a172105);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarConfirmationButtonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct IWebUICommandBarElement(::windows::core::IUnknown);
impl IWebUICommandBarElement {}
impl ::core::convert::From<IWebUICommandBarElement> for ::windows::core::IInspectable {
    fn from(value: IWebUICommandBarElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarElement> for ::windows::core::IInspectable {
    fn from(value: &IWebUICommandBarElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebUICommandBarElement> for ::windows::core::IUnknown {
    fn from(value: IWebUICommandBarElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarElement> for ::windows::core::IUnknown {
    fn from(value: &IWebUICommandBarElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebUICommandBarElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUICommandBarElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUICommandBarElement {}
impl ::core::fmt::Debug for IWebUICommandBarElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUICommandBarElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebUICommandBarElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c9069ec2-284a-4633-8aad-637a27e282c3}");
}
unsafe impl ::windows::core::Interface for IWebUICommandBarElement {
    type Vtable = IWebUICommandBarElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9069ec2_284a_4633_8aad_637a27e282c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct IWebUICommandBarIcon(::windows::core::IUnknown);
impl IWebUICommandBarIcon {}
impl ::core::convert::From<IWebUICommandBarIcon> for ::windows::core::IInspectable {
    fn from(value: IWebUICommandBarIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarIcon> for ::windows::core::IInspectable {
    fn from(value: &IWebUICommandBarIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebUICommandBarIcon> for ::windows::core::IUnknown {
    fn from(value: IWebUICommandBarIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarIcon> for ::windows::core::IUnknown {
    fn from(value: &IWebUICommandBarIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebUICommandBarIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUICommandBarIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUICommandBarIcon {}
impl ::core::fmt::Debug for IWebUICommandBarIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUICommandBarIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebUICommandBarIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d587655d-2014-42be-969a-7d14ca6c8a49}");
}
unsafe impl ::windows::core::Interface for IWebUICommandBarIcon {
    type Vtable = IWebUICommandBarIconVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd587655d_2014_42be_969a_7d14ca6c8a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIconVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarIconButton(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButtonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1bc93a_3a7c_4842_a0cf_aff6ea308586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIconButtonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarItemInvokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x304edbdd_e741_41ef_bdc4_a45cea2a4f70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarItemInvokedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSizeChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf1e2f6_3029_4719_8378_92f82b87af1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSizeChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarStatics {
    type Vtable = IWebUICommandBarStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1449cdb9_a506_45be_8f42_b2837e2fe0c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarStaticsVtbl(
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
pub struct IWebUICommandBarSymbolIcon(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIconVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4935477_fd26_46ed_8658_1a3f4400e7b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIconVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIconFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIconFactory {
    type Vtable = IWebUICommandBarSymbolIconFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51be1a1f_3730_429e_b622_14e2b7bf6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIconFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct MenuClosedEventHandler(pub ::windows::core::IUnknown);
impl MenuClosedEventHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = MenuClosedEventHandlerBox::<F> { vtable: &MenuClosedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[repr(C)]
struct MenuClosedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + 'static> {
    vtable: *const MenuClosedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + 'static> MenuClosedEventHandlerBox<F> {
    const VTABLE: MenuClosedEventHandlerVtbl = MenuClosedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MenuClosedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for MenuClosedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MenuClosedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MenuClosedEventHandler {}
impl ::core::fmt::Debug for MenuClosedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuClosedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for MenuClosedEventHandler {
    type Vtable = MenuClosedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x435387c8_4dd0_4c52_9489_d390ce7721d2);
}
unsafe impl ::windows::core::RuntimeType for MenuClosedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{435387c8-4dd0-4c52-9489-d390ce7721d2}");
}
#[repr(C)]
#[doc(hidden)]
pub struct MenuClosedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct MenuOpenedEventHandler(pub ::windows::core::IUnknown);
impl MenuOpenedEventHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = MenuOpenedEventHandlerBox::<F> { vtable: &MenuOpenedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[repr(C)]
struct MenuOpenedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + 'static> {
    vtable: *const MenuOpenedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + 'static> MenuOpenedEventHandlerBox<F> {
    const VTABLE: MenuOpenedEventHandlerVtbl = MenuOpenedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MenuOpenedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for MenuOpenedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MenuOpenedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MenuOpenedEventHandler {}
impl ::core::fmt::Debug for MenuOpenedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuOpenedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for MenuOpenedEventHandler {
    type Vtable = MenuOpenedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18dc0ad3_678f_4c19_8963_cc1c49a5ef9e);
}
unsafe impl ::windows::core::RuntimeType for MenuOpenedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{18dc0ad3-678f-4c19-8963-cc1c49a5ef9e}");
}
#[repr(C)]
#[doc(hidden)]
pub struct MenuOpenedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct SizeChangedEventHandler(pub ::windows::core::IUnknown);
impl SizeChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = SizeChangedEventHandlerBox::<F> { vtable: &SizeChangedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, WebUICommandBarSizeChangedEventArgs>>(&self, eventargs: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), eventargs.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct SizeChangedEventHandlerBox<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const SizeChangedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + 'static> SizeChangedEventHandlerBox<F> {
    const VTABLE: SizeChangedEventHandlerVtbl = SizeChangedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<SizeChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, eventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&eventargs as *const <WebUICommandBarSizeChangedEventArgs as ::windows::core::Abi>::Abi as *const <WebUICommandBarSizeChangedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for SizeChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SizeChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SizeChangedEventHandler {}
impl ::core::fmt::Debug for SizeChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SizeChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for SizeChangedEventHandler {
    type Vtable = SizeChangedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd49cfe3c_dd2e_4c28_b627_303a7f911af5);
}
unsafe impl ::windows::core::RuntimeType for SizeChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d49cfe3c-dd2e-4c28-b627-303a7f911af5}");
}
#[repr(C)]
#[doc(hidden)]
pub struct SizeChangedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBar(::windows::core::IUnknown);
impl WebUICommandBar {
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn ClosedDisplayMode(&self) -> ::windows::core::Result<WebUICommandBarClosedDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: WebUICommandBarClosedDisplayMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebUICommandBarClosedDisplayMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetClosedDisplayMode(&self, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn IsOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuOpened<'a, Param0: ::windows::core::IntoParam<'a, MenuOpenedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuClosed<'a, Param0: ::windows::core::IntoParam<'a, MenuClosedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged<'a, Param0: ::windows::core::IntoParam<'a, SizeChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<WebUICommandBar> {
        Self::IWebUICommandBarStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebUICommandBar>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarStatics<R, F: FnOnce(&IWebUICommandBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBar, IWebUICommandBarStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebUICommandBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBar {}
impl ::core::fmt::Debug for WebUICommandBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBar;{a4fc0016-dbe5-41ad-8d7b-14698bd6911d})");
}
unsafe impl ::windows::core::Interface for WebUICommandBar {
    type Vtable = IWebUICommandBarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4fc0016_dbe5_41ad_8d7b_14698bd6911d);
}
impl ::windows::core::RuntimeName for WebUICommandBar {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBar";
}
impl ::core::convert::From<WebUICommandBar> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBar> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBar> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBar> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebUICommandBar {}
unsafe impl ::core::marker::Sync for WebUICommandBar {}
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBarBitmapIcon(::windows::core::IUnknown);
impl WebUICommandBarBitmapIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarBitmapIcon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<WebUICommandBarBitmapIcon> {
        Self::IWebUICommandBarBitmapIconFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<WebUICommandBarBitmapIcon>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarBitmapIconFactory<R, F: FnOnce(&IWebUICommandBarBitmapIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarBitmapIcon, IWebUICommandBarBitmapIconFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebUICommandBarBitmapIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarBitmapIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarBitmapIcon {}
impl ::core::fmt::Debug for WebUICommandBarBitmapIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarBitmapIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarBitmapIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon;{858f4f45-08d8-4a46-81ec-00015b0b1c6c})");
}
unsafe impl ::windows::core::Interface for WebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIconVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858f4f45_08d8_4a46_81ec_00015b0b1c6c);
}
impl ::windows::core::RuntimeName for WebUICommandBarBitmapIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon";
}
impl ::core::convert::From<WebUICommandBarBitmapIcon> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarBitmapIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarBitmapIcon> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarBitmapIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarBitmapIcon> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarBitmapIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarBitmapIcon> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarBitmapIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebUICommandBarBitmapIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarBitmapIcon) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarBitmapIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarBitmapIcon) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for &WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::core::convert::TryInto::<IWebUICommandBarIcon>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarBitmapIcon {}
unsafe impl ::core::marker::Sync for WebUICommandBarBitmapIcon {}
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBarClosedDisplayMode(pub i32);
impl WebUICommandBarClosedDisplayMode {
    pub const Default: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
    pub const Compact: Self = Self(2i32);
}
impl ::core::marker::Copy for WebUICommandBarClosedDisplayMode {}
impl ::core::clone::Clone for WebUICommandBarClosedDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WebUICommandBarClosedDisplayMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WebUICommandBarClosedDisplayMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarClosedDisplayMode {}
impl ::core::fmt::Debug for WebUICommandBarClosedDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarClosedDisplayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarClosedDisplayMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.Core.WebUICommandBarClosedDisplayMode;i4)");
}
impl ::windows::core::DefaultType for WebUICommandBarClosedDisplayMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBarConfirmationButton(::windows::core::IUnknown);
impl WebUICommandBarConfirmationButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarConfirmationButton, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WebUICommandBarConfirmationButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarConfirmationButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarConfirmationButton {}
impl ::core::fmt::Debug for WebUICommandBarConfirmationButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarConfirmationButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarConfirmationButton {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton;{86e7824a-e3d5-4eb6-b2ff-8f018a172105})");
}
unsafe impl ::windows::core::Interface for WebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButtonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e7824a_e3d5_4eb6_b2ff_8f018a172105);
}
impl ::windows::core::RuntimeName for WebUICommandBarConfirmationButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton";
}
impl ::core::convert::From<WebUICommandBarConfirmationButton> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarConfirmationButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarConfirmationButton> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarConfirmationButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarConfirmationButton> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarConfirmationButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarConfirmationButton> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarConfirmationButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebUICommandBarConfirmationButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarConfirmationButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarConfirmationButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarConfirmationButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for &WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::core::convert::TryInto::<IWebUICommandBarElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarConfirmationButton {}
unsafe impl ::core::marker::Sync for WebUICommandBarConfirmationButton {}
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBarIconButton(::windows::core::IUnknown);
impl WebUICommandBarIconButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarIconButton, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn IsToggleButton(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetIsToggleButton(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn IsChecked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Icon(&self) -> ::windows::core::Result<IWebUICommandBarIcon> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IWebUICommandBarIcon>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, IWebUICommandBarIcon>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WebUICommandBarIconButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarIconButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarIconButton {}
impl ::core::fmt::Debug for WebUICommandBarIconButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarIconButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarIconButton {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarIconButton;{8f1bc93a-3a7c-4842-a0cf-aff6ea308586})");
}
unsafe impl ::windows::core::Interface for WebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButtonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1bc93a_3a7c_4842_a0cf_aff6ea308586);
}
impl ::windows::core::RuntimeName for WebUICommandBarIconButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarIconButton";
}
impl ::core::convert::From<WebUICommandBarIconButton> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarIconButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarIconButton> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarIconButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarIconButton> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarIconButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarIconButton> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarIconButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebUICommandBarIconButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarIconButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarIconButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarIconButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for &WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::core::convert::TryInto::<IWebUICommandBarElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarIconButton {}
unsafe impl ::core::marker::Sync for WebUICommandBarIconButton {}
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBarItemInvokedEventArgs(::windows::core::IUnknown);
impl WebUICommandBarItemInvokedEventArgs {
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn IsPrimaryCommand(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for WebUICommandBarItemInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarItemInvokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarItemInvokedEventArgs {}
impl ::core::fmt::Debug for WebUICommandBarItemInvokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarItemInvokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarItemInvokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs;{304edbdd-e741-41ef-bdc4-a45cea2a4f70})");
}
unsafe impl ::windows::core::Interface for WebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x304edbdd_e741_41ef_bdc4_a45cea2a4f70);
}
impl ::windows::core::RuntimeName for WebUICommandBarItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs";
}
impl ::core::convert::From<WebUICommandBarItemInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarItemInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarItemInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarItemInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarItemInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarItemInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarItemInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarItemInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarItemInvokedEventArgs {}
unsafe impl ::core::marker::Sync for WebUICommandBarItemInvokedEventArgs {}
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBarSizeChangedEventArgs(::windows::core::IUnknown);
impl WebUICommandBarSizeChangedEventArgs {
    #[doc = "*Required features: 'UI_WebUI_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
}
impl ::core::clone::Clone for WebUICommandBarSizeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarSizeChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarSizeChangedEventArgs {}
impl ::core::fmt::Debug for WebUICommandBarSizeChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarSizeChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarSizeChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs;{fbf1e2f6-3029-4719-8378-92f82b87af1e})");
}
unsafe impl ::windows::core::Interface for WebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf1e2f6_3029_4719_8378_92f82b87af1e);
}
impl ::windows::core::RuntimeName for WebUICommandBarSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs";
}
impl ::core::convert::From<WebUICommandBarSizeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarSizeChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSizeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarSizeChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarSizeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarSizeChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSizeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarSizeChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarSizeChangedEventArgs {}
unsafe impl ::core::marker::Sync for WebUICommandBarSizeChangedEventArgs {}
#[doc = "*Required features: 'UI_WebUI_Core'*"]
#[repr(transparent)]
pub struct WebUICommandBarSymbolIcon(::windows::core::IUnknown);
impl WebUICommandBarSymbolIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarSymbolIcon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Symbol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn SetSymbol<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WebUI_Core'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(symbol: Param0) -> ::windows::core::Result<WebUICommandBarSymbolIcon> {
        Self::IWebUICommandBarSymbolIconFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), symbol.into_param().abi(), &mut result__).from_abi::<WebUICommandBarSymbolIcon>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarSymbolIconFactory<R, F: FnOnce(&IWebUICommandBarSymbolIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarSymbolIcon, IWebUICommandBarSymbolIconFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebUICommandBarSymbolIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarSymbolIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarSymbolIcon {}
impl ::core::fmt::Debug for WebUICommandBarSymbolIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarSymbolIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarSymbolIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon;{d4935477-fd26-46ed-8658-1a3f4400e7b3})");
}
unsafe impl ::windows::core::Interface for WebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIconVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4935477_fd26_46ed_8658_1a3f4400e7b3);
}
impl ::windows::core::RuntimeName for WebUICommandBarSymbolIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon";
}
impl ::core::convert::From<WebUICommandBarSymbolIcon> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarSymbolIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSymbolIcon> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarSymbolIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarSymbolIcon> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarSymbolIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSymbolIcon> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarSymbolIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebUICommandBarSymbolIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarSymbolIcon) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarSymbolIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarSymbolIcon) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for &WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::core::convert::TryInto::<IWebUICommandBarIcon>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarSymbolIcon {}
unsafe impl ::core::marker::Sync for WebUICommandBarSymbolIcon {}
