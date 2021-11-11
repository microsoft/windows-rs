#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Core_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreAppWindowPreview(pub ::windows::runtime::IInspectable);
impl CoreAppWindowPreview {
    #[cfg(feature = "UI_WindowManagement")]
    #[doc = "*Required features: `UI_Core_Preview`, `UI_WindowManagement`*"]
    pub fn GetIdFromWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows::runtime::Result<i32> {
        Self::ICoreAppWindowPreviewStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn ICoreAppWindowPreviewStatics<R, F: FnOnce(&ICoreAppWindowPreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreAppWindowPreview, ICoreAppWindowPreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreAppWindowPreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.CoreAppWindowPreview;{a4f6e665-365e-5fde-87a5-9543c3a15aa8})");
}
unsafe impl ::windows::runtime::Interface for CoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
}
impl ::windows::runtime::RuntimeName for CoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.CoreAppWindowPreview";
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows::runtime::IUnknown {
    fn from(value: CoreAppWindowPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows::runtime::IUnknown {
    fn from(value: &CoreAppWindowPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows::runtime::IInspectable {
    fn from(value: CoreAppWindowPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows::runtime::IInspectable {
    fn from(value: &CoreAppWindowPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreAppWindowPreview {}
unsafe impl ::core::marker::Sync for CoreAppWindowPreview {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreAppWindowPreview(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreAppWindowPreviewStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreAppWindowPreviewStatics {
    type Vtable = ICoreAppWindowPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33ac21be_423b_5db6_8a8e_4dc87353b75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x83d00de1_cbe5_4f31_8414_361da046518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreview(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xec5f0488_6425_4777_a536_cb5634427f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreviewStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemNavigationManagerPreviewStatics {
    type Vtable = ISystemNavigationManagerPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0e971360_df74_4bce_84cb_bd1181ac0a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Core_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(pub ::windows::runtime::IInspectable);
impl SystemNavigationCloseRequestedPreviewEventArgs {
    #[doc = "*Required features: `UI_Core_Preview`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_Preview`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_Preview`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemNavigationCloseRequestedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs;{83d00de1-cbe5-4f31-8414-361da046518f})");
}
unsafe impl ::windows::runtime::Interface for SystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x83d00de1_cbe5_4f31_8414_361da046518f);
}
impl ::windows::runtime::RuntimeName for SystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemNavigationCloseRequestedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for SystemNavigationCloseRequestedPreviewEventArgs {}
#[doc = "*Required features: `UI_Core_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemNavigationManagerPreview(pub ::windows::runtime::IInspectable);
impl SystemNavigationManagerPreview {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_Preview`, `Foundation`*"]
    pub fn CloseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_Preview`, `Foundation`*"]
    pub fn RemoveCloseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Core_Preview`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<SystemNavigationManagerPreview> {
        Self::ISystemNavigationManagerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemNavigationManagerPreview>(result__)
        })
    }
    pub fn ISystemNavigationManagerPreviewStatics<R, F: FnOnce(&ISystemNavigationManagerPreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemNavigationManagerPreview, ISystemNavigationManagerPreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemNavigationManagerPreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationManagerPreview;{ec5f0488-6425-4777-a536-cb5634427f0d})");
}
unsafe impl ::windows::runtime::Interface for SystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xec5f0488_6425_4777_a536_cb5634427f0d);
}
impl ::windows::runtime::RuntimeName for SystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationManagerPreview";
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows::runtime::IUnknown {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows::runtime::IUnknown {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows::runtime::IInspectable {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows::runtime::IInspectable {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemNavigationManagerPreview {}
unsafe impl ::core::marker::Sync for SystemNavigationManagerPreview {}
