#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowManagementPreviewStatics {
    type Vtable = IWindowManagementPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f9725c6_c004_5a23_8fd2_8d092ce2704a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
#[doc = "*Required features: `\"UI_WindowManagement_Preview\"`*"]
#[repr(transparent)]
pub struct WindowManagementPreview(::windows::core::IUnknown);
impl WindowManagementPreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredMinSize<'a, P0>(window: P0, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::AppWindow>>,
    {
        Self::IWindowManagementPreviewStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPreferredMinSize)(::windows::core::Interface::as_raw(this), window.into().abi(), preferredframeminsize).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowManagementPreviewStatics<R, F: FnOnce(&IWindowManagementPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WindowManagementPreview, IWindowManagementPreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WindowManagementPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowManagementPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowManagementPreview {}
impl ::core::fmt::Debug for WindowManagementPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowManagementPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowManagementPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.Preview.WindowManagementPreview;{4ef55b0d-561d-513c-a67c-2c02b69cef41})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
    const IID: ::windows::core::GUID = <IWindowManagementPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.WindowManagementPreview";
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::core::IUnknown {
    fn from(value: WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::core::IUnknown {
    fn from(value: &WindowManagementPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WindowManagementPreview> for &::windows::core::IUnknown {
    fn from(value: &WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::core::IInspectable {
    fn from(value: WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::core::IInspectable {
    fn from(value: &WindowManagementPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WindowManagementPreview> for &::windows::core::IInspectable {
    fn from(value: &WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for WindowManagementPreview {}
unsafe impl ::core::marker::Sync for WindowManagementPreview {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
