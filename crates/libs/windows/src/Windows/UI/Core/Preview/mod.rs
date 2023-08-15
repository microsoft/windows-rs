#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAppWindowPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_Vtbl;
}
impl ::core::clone::Clone for ICoreAppWindowPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreAppWindowPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAppWindowPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAppWindowPreviewStatics {
    type Vtable = ICoreAppWindowPreviewStatics_Vtbl;
}
impl ::core::clone::Clone for ICoreAppWindowPreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICoreAppWindowPreviewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33ac21be_423b_5db6_8a8e_4dc87353b75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub GetIdFromWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    GetIdFromWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISystemNavigationCloseRequestedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemNavigationCloseRequestedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83d00de1_cbe5_4f31_8414_361da046518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManagerPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_Vtbl;
}
impl ::core::clone::Clone for ISystemNavigationManagerPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemNavigationManagerPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec5f0488_6425_4777_a536_cb5634427f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCloseRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManagerPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemNavigationManagerPreviewStatics {
    type Vtable = ISystemNavigationManagerPreviewStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemNavigationManagerPreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemNavigationManagerPreviewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e971360_df74_4bce_84cb_bd1181ac0a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"UI_Core_Preview\"`*"]
#[repr(transparent)]
pub struct CoreAppWindowPreview(::windows_core::IUnknown);
impl CoreAppWindowPreview {
    #[doc = "*Required features: `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn GetIdFromWindow<P0>(window: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<super::super::WindowManagement::AppWindow>,
    {
        Self::ICoreAppWindowPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIdFromWindow)(::windows_core::Interface::as_raw(this), window.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreAppWindowPreviewStatics<R, F: FnOnce(&ICoreAppWindowPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CoreAppWindowPreview, ICoreAppWindowPreviewStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CoreAppWindowPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAppWindowPreview {}
impl ::core::fmt::Debug for CoreAppWindowPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAppWindowPreview").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreAppWindowPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.CoreAppWindowPreview;{a4f6e665-365e-5fde-87a5-9543c3a15aa8})");
}
impl ::core::clone::Clone for CoreAppWindowPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreAppWindowPreview {
    const IID: ::windows_core::GUID = <ICoreAppWindowPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.CoreAppWindowPreview";
}
::windows_core::imp::interface_hierarchy!(CoreAppWindowPreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CoreAppWindowPreview {}
unsafe impl ::core::marker::Sync for CoreAppWindowPreview {}
#[doc = "*Required features: `\"UI_Core_Preview\"`*"]
#[repr(transparent)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(::windows_core::IUnknown);
impl SystemNavigationCloseRequestedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SystemNavigationCloseRequestedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemNavigationCloseRequestedPreviewEventArgs {}
impl ::core::fmt::Debug for SystemNavigationCloseRequestedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemNavigationCloseRequestedPreviewEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemNavigationCloseRequestedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs;{83d00de1-cbe5-4f31-8414-361da046518f})");
}
impl ::core::clone::Clone for SystemNavigationCloseRequestedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemNavigationCloseRequestedPreviewEventArgs {
    const IID: ::windows_core::GUID = <ISystemNavigationCloseRequestedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(SystemNavigationCloseRequestedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemNavigationCloseRequestedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for SystemNavigationCloseRequestedPreviewEventArgs {}
#[doc = "*Required features: `\"UI_Core_Preview\"`*"]
#[repr(transparent)]
pub struct SystemNavigationManagerPreview(::windows_core::IUnknown);
impl SystemNavigationManagerPreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CloseRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloseRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCloseRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCloseRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<SystemNavigationManagerPreview> {
        Self::ISystemNavigationManagerPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemNavigationManagerPreviewStatics<R, F: FnOnce(&ISystemNavigationManagerPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemNavigationManagerPreview, ISystemNavigationManagerPreviewStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SystemNavigationManagerPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemNavigationManagerPreview {}
impl ::core::fmt::Debug for SystemNavigationManagerPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemNavigationManagerPreview").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemNavigationManagerPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationManagerPreview;{ec5f0488-6425-4777-a536-cb5634427f0d})");
}
impl ::core::clone::Clone for SystemNavigationManagerPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemNavigationManagerPreview {
    const IID: ::windows_core::GUID = <ISystemNavigationManagerPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationManagerPreview";
}
::windows_core::imp::interface_hierarchy!(SystemNavigationManagerPreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemNavigationManagerPreview {}
unsafe impl ::core::marker::Sync for SystemNavigationManagerPreview {}
