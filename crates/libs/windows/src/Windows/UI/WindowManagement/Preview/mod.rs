#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
}
impl ::core::clone::Clone for IWindowManagementPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWindowManagementPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowManagementPreviewStatics {
    type Vtable = IWindowManagementPreviewStatics_Vtbl;
}
impl ::core::clone::Clone for IWindowManagementPreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWindowManagementPreviewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f9725c6_c004_5a23_8fd2_8d092ce2704a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
#[doc = "*Required features: `\"UI_WindowManagement_Preview\"`*"]
#[repr(transparent)]
pub struct WindowManagementPreview(::windows_core::IUnknown);
impl WindowManagementPreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredMinSize<P0>(window: P0, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AppWindow>,
    {
        Self::IWindowManagementPreviewStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetPreferredMinSize)(::windows_core::Interface::as_raw(this), window.into_param().abi(), preferredframeminsize).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowManagementPreviewStatics<R, F: FnOnce(&IWindowManagementPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WindowManagementPreview, IWindowManagementPreviewStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for WindowManagementPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.Preview.WindowManagementPreview;{4ef55b0d-561d-513c-a67c-2c02b69cef41})");
}
impl ::core::clone::Clone for WindowManagementPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowManagementPreview {
    const IID: ::windows_core::GUID = <IWindowManagementPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.WindowManagementPreview";
}
::windows_core::imp::interface_hierarchy!(WindowManagementPreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WindowManagementPreview {}
unsafe impl ::core::marker::Sync for WindowManagementPreview {}
