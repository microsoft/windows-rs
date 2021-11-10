#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowManagementPreview(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWindowManagementPreview {
    type Vtable = IWindowManagementPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWindowManagementPreviewStatics {
    type Vtable = IWindowManagementPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0f9725c6_c004_5a23_8fd2_8d092ce2704a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: ::windows::runtime::RawPtr, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `UI_WindowManagement_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WindowManagementPreview(pub ::windows::runtime::IInspectable);
impl WindowManagementPreview {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WindowManagement_Preview`, `Foundation`*"]
    pub fn SetPreferredMinSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::AppWindow>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Size>>(window: Param0, preferredframeminsize: Param1) -> ::windows::runtime::Result<()> {
        Self::IWindowManagementPreviewStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi(), preferredframeminsize.into_param().abi()).ok() })
    }
    pub fn IWindowManagementPreviewStatics<R, F: FnOnce(&IWindowManagementPreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WindowManagementPreview, IWindowManagementPreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WindowManagementPreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.Preview.WindowManagementPreview;{4ef55b0d-561d-513c-a67c-2c02b69cef41})");
}
unsafe impl ::windows::runtime::Interface for WindowManagementPreview {
    type Vtable = IWindowManagementPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
impl ::windows::runtime::RuntimeName for WindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.WindowManagementPreview";
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::runtime::IUnknown {
    fn from(value: WindowManagementPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::runtime::IUnknown {
    fn from(value: &WindowManagementPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WindowManagementPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::runtime::IInspectable {
    fn from(value: WindowManagementPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::runtime::IInspectable {
    fn from(value: &WindowManagementPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WindowManagementPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WindowManagementPreview {}
unsafe impl ::core::marker::Sync for WindowManagementPreview {}
