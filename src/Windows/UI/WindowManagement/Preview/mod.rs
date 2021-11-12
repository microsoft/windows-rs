#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowManagementPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowManagementPreview {
    type Vtable = IWindowManagementPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowManagementPreviewStatics {
    type Vtable = IWindowManagementPreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f9725c6_c004_5a23_8fd2_8d092ce2704a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: ::windows::core::RawPtr, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WindowManagementPreview(pub ::windows::core::IInspectable);
impl WindowManagementPreview {
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredMinSize<'a, Param0: ::windows::core::IntoParam<'a, super::AppWindow>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(window: Param0, preferredframeminsize: Param1) -> ::windows::core::Result<()> {
        Self::IWindowManagementPreviewStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi(), preferredframeminsize.into_param().abi()).ok() })
    }
    pub fn IWindowManagementPreviewStatics<R, F: FnOnce(&IWindowManagementPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowManagementPreview, IWindowManagementPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WindowManagementPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.Preview.WindowManagementPreview;{4ef55b0d-561d-513c-a67c-2c02b69cef41})");
}
unsafe impl ::windows::core::Interface for WindowManagementPreview {
    type Vtable = IWindowManagementPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
impl ::windows::core::RuntimeName for WindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.WindowManagementPreview";
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::core::IUnknown {
    fn from(value: WindowManagementPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::core::IUnknown {
    fn from(value: &WindowManagementPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::core::IInspectable {
    fn from(value: WindowManagementPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::core::IInspectable {
    fn from(value: &WindowManagementPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WindowManagementPreview {}
unsafe impl ::core::marker::Sync for WindowManagementPreview {}
