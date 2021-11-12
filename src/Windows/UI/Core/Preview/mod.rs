#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreAppWindowPreview(pub ::windows::core::IInspectable);
impl CoreAppWindowPreview {
    #[cfg(feature = "UI_WindowManagement")]
    pub fn GetIdFromWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows::core::Result<i32> {
        Self::ICoreAppWindowPreviewStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn ICoreAppWindowPreviewStatics<R, F: FnOnce(&ICoreAppWindowPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreAppWindowPreview, ICoreAppWindowPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreAppWindowPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.CoreAppWindowPreview;{a4f6e665-365e-5fde-87a5-9543c3a15aa8})");
}
unsafe impl ::windows::core::Interface for CoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
}
impl ::windows::core::RuntimeName for CoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.CoreAppWindowPreview";
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows::core::IUnknown {
    fn from(value: CoreAppWindowPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows::core::IUnknown {
    fn from(value: &CoreAppWindowPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows::core::IInspectable {
    fn from(value: CoreAppWindowPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows::core::IInspectable {
    fn from(value: &CoreAppWindowPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreAppWindowPreview {}
unsafe impl ::core::marker::Sync for CoreAppWindowPreview {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreAppWindowPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreAppWindowPreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreAppWindowPreviewStatics {
    type Vtable = ICoreAppWindowPreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ac21be_423b_5db6_8a8e_4dc87353b75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d00de1_cbe5_4f31_8414_361da046518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec5f0488_6425_4777_a536_cb5634427f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemNavigationManagerPreviewStatics {
    type Vtable = ISystemNavigationManagerPreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e971360_df74_4bce_84cb_bd1181ac0a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(pub ::windows::core::IInspectable);
impl SystemNavigationCloseRequestedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemNavigationCloseRequestedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs;{83d00de1-cbe5-4f31-8414-361da046518f})");
}
unsafe impl ::windows::core::Interface for SystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d00de1_cbe5_4f31_8414_361da046518f);
}
impl ::windows::core::RuntimeName for SystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemNavigationCloseRequestedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for SystemNavigationCloseRequestedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemNavigationManagerPreview(pub ::windows::core::IInspectable);
impl SystemNavigationManagerPreview {
    #[cfg(feature = "Foundation")]
    pub fn CloseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCloseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<SystemNavigationManagerPreview> {
        Self::ISystemNavigationManagerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemNavigationManagerPreview>(result__)
        })
    }
    pub fn ISystemNavigationManagerPreviewStatics<R, F: FnOnce(&ISystemNavigationManagerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemNavigationManagerPreview, ISystemNavigationManagerPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemNavigationManagerPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationManagerPreview;{ec5f0488-6425-4777-a536-cb5634427f0d})");
}
unsafe impl ::windows::core::Interface for SystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec5f0488_6425_4777_a536_cb5634427f0d);
}
impl ::windows::core::RuntimeName for SystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationManagerPreview";
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows::core::IUnknown {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows::core::IUnknown {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows::core::IInspectable {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows::core::IInspectable {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemNavigationManagerPreview {}
unsafe impl ::core::marker::Sync for SystemNavigationManagerPreview {}
