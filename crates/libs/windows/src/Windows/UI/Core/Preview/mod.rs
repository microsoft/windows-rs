#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Core_Preview\"`*"]
#[repr(transparent)]
pub struct CoreAppWindowPreview(::windows::core::IUnknown);
impl CoreAppWindowPreview {
    #[doc = "*Required features: `\"UI_Core_Preview\"`, `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn GetIdFromWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows::core::Result<i32> {
        Self::ICoreAppWindowPreviewStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIdFromWindow)(::core::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreAppWindowPreviewStatics<R, F: FnOnce(&ICoreAppWindowPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreAppWindowPreview, ICoreAppWindowPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreAppWindowPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CoreAppWindowPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.CoreAppWindowPreview;{a4f6e665-365e-5fde-87a5-9543c3a15aa8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_Vtbl;
    const IID: ::windows::core::GUID = <ICoreAppWindowPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.CoreAppWindowPreview";
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows::core::IUnknown {
    fn from(value: CoreAppWindowPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows::core::IUnknown {
    fn from(value: &CoreAppWindowPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows::core::IInspectable {
    fn from(value: CoreAppWindowPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows::core::IInspectable {
    fn from(value: &CoreAppWindowPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreAppWindowPreview {}
unsafe impl ::core::marker::Sync for CoreAppWindowPreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAppWindowPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAppWindowPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreAppWindowPreviewStatics {
    type Vtable = ICoreAppWindowPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ac21be_423b_5db6_8a8e_4dc87353b75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub GetIdFromWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    GetIdFromWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d00de1_cbe5_4f31_8414_361da046518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManagerPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec5f0488_6425_4777_a536_cb5634427f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCloseRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManagerPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemNavigationManagerPreviewStatics {
    type Vtable = ISystemNavigationManagerPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e971360_df74_4bce_84cb_bd1181ac0a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Core_Preview\"`*"]
#[repr(transparent)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(::windows::core::IUnknown);
impl SystemNavigationCloseRequestedPreviewEventArgs {
    #[doc = "*Required features: `\"UI_Core_Preview\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core_Preview\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Core_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemNavigationCloseRequestedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SystemNavigationCloseRequestedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs;{83d00de1-cbe5-4f31-8414-361da046518f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISystemNavigationCloseRequestedPreviewEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemNavigationCloseRequestedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for SystemNavigationCloseRequestedPreviewEventArgs {}
#[doc = "*Required features: `\"UI_Core_Preview\"`*"]
#[repr(transparent)]
pub struct SystemNavigationManagerPreview(::windows::core::IUnknown);
impl SystemNavigationManagerPreview {
    #[doc = "*Required features: `\"UI_Core_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CloseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CloseRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCloseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCloseRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Core_Preview\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<SystemNavigationManagerPreview> {
        Self::ISystemNavigationManagerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemNavigationManagerPreview>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemNavigationManagerPreviewStatics<R, F: FnOnce(&ISystemNavigationManagerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemNavigationManagerPreview, ISystemNavigationManagerPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemNavigationManagerPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SystemNavigationManagerPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationManagerPreview;{ec5f0488-6425-4777-a536-cb5634427f0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_Vtbl;
    const IID: ::windows::core::GUID = <ISystemNavigationManagerPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationManagerPreview";
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows::core::IUnknown {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows::core::IUnknown {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows::core::IInspectable {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows::core::IInspectable {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemNavigationManagerPreview {}
unsafe impl ::core::marker::Sync for SystemNavigationManagerPreview {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
