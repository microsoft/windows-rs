#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemUpdateItem(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemUpdateItem {
    type Vtable = ISystemUpdateItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2006401259, 22052, 20894, [168, 226, 9, 233, 23, 59, 63, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemUpdateItemState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemUpdateLastErrorInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2129168375, 35396, 23406, [189, 7, 122, 236, 228, 17, 110, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateLastErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemUpdateManagerState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemUpdateManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemUpdateManagerStatics {
    type Vtable = ISystemUpdateManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3000237295, 10609, 20926, [180, 26, 139, 215, 3, 187, 112, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemUpdateManagerState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, start: super::super::Foundation::TimeSpan, end: super::super::Foundation::TimeSpan, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lockid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lockid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemUpdateAttentionRequiredReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flightring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: SystemUpdateStartInstallAction) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `System_Update`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemUpdateAttentionRequiredReason(pub i32);
impl SystemUpdateAttentionRequiredReason {
    pub const None: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(0i32);
    pub const NetworkRequired: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(1i32);
    pub const InsufficientDiskSpace: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(2i32);
    pub const InsufficientBattery: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(3i32);
    pub const UpdateBlocked: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(4i32);
}
impl ::std::convert::From<i32> for SystemUpdateAttentionRequiredReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemUpdateAttentionRequiredReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemUpdateAttentionRequiredReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateAttentionRequiredReason;i4)");
}
impl ::windows::runtime::DefaultType for SystemUpdateAttentionRequiredReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_Update`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemUpdateItem(pub ::windows::runtime::IInspectable);
impl SystemUpdateItem {
    #[doc = "*Required features: `System_Update`*"]
    pub fn State(&self) -> ::windows::runtime::Result<SystemUpdateItemState> {
        let this = self;
        unsafe {
            let mut result__: SystemUpdateItemState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateItemState>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn Revision(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn DownloadProgress(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn InstallProgress(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemUpdateItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateItem;{779740eb-5624-519e-a8e2-09e9173b3fb7})");
}
unsafe impl ::windows::runtime::Interface for SystemUpdateItem {
    type Vtable = ISystemUpdateItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2006401259, 22052, 20894, [168, 226, 9, 233, 23, 59, 63, 183]);
}
impl ::windows::runtime::RuntimeName for SystemUpdateItem {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateItem";
}
impl ::std::convert::From<SystemUpdateItem> for ::windows::runtime::IUnknown {
    fn from(value: SystemUpdateItem) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SystemUpdateItem> for ::windows::runtime::IUnknown {
    fn from(value: &SystemUpdateItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemUpdateItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SystemUpdateItem> for ::windows::runtime::IInspectable {
    fn from(value: SystemUpdateItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemUpdateItem> for ::windows::runtime::IInspectable {
    fn from(value: &SystemUpdateItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemUpdateItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemUpdateItem {}
unsafe impl ::std::marker::Sync for SystemUpdateItem {}
#[doc = "*Required features: `System_Update`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemUpdateItemState(pub i32);
impl SystemUpdateItemState {
    pub const NotStarted: SystemUpdateItemState = SystemUpdateItemState(0i32);
    pub const Initializing: SystemUpdateItemState = SystemUpdateItemState(1i32);
    pub const Preparing: SystemUpdateItemState = SystemUpdateItemState(2i32);
    pub const Calculating: SystemUpdateItemState = SystemUpdateItemState(3i32);
    pub const Downloading: SystemUpdateItemState = SystemUpdateItemState(4i32);
    pub const Installing: SystemUpdateItemState = SystemUpdateItemState(5i32);
    pub const Completed: SystemUpdateItemState = SystemUpdateItemState(6i32);
    pub const RebootRequired: SystemUpdateItemState = SystemUpdateItemState(7i32);
    pub const Error: SystemUpdateItemState = SystemUpdateItemState(8i32);
}
impl ::std::convert::From<i32> for SystemUpdateItemState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemUpdateItemState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemUpdateItemState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateItemState;i4)");
}
impl ::windows::runtime::DefaultType for SystemUpdateItemState {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_Update`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemUpdateLastErrorInfo(pub ::windows::runtime::IInspectable);
impl SystemUpdateLastErrorInfo {
    #[doc = "*Required features: `System_Update`*"]
    pub fn State(&self) -> ::windows::runtime::Result<SystemUpdateManagerState> {
        let this = self;
        unsafe {
            let mut result__: SystemUpdateManagerState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateManagerState>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn IsInteractive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemUpdateLastErrorInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateLastErrorInfo;{7ee887f7-8a44-5b6e-bd07-7aece4116ea9})");
}
unsafe impl ::windows::runtime::Interface for SystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2129168375, 35396, 23406, [189, 7, 122, 236, 228, 17, 110, 169]);
}
impl ::windows::runtime::RuntimeName for SystemUpdateLastErrorInfo {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateLastErrorInfo";
}
impl ::std::convert::From<SystemUpdateLastErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SystemUpdateLastErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SystemUpdateLastErrorInfo> for ::windows::runtime::IInspectable {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemUpdateLastErrorInfo> for ::windows::runtime::IInspectable {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemUpdateLastErrorInfo {}
unsafe impl ::std::marker::Sync for SystemUpdateLastErrorInfo {}
#[doc = "*Required features: `System_Update`*"]
pub struct SystemUpdateManager {}
impl SystemUpdateManager {
    #[doc = "*Required features: `System_Update`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn State() -> ::windows::runtime::Result<SystemUpdateManagerState> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: SystemUpdateManagerState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateManagerState>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn DownloadProgress() -> ::windows::runtime::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn InstallProgress() -> ::windows::runtime::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn UserActiveHoursStart() -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn UserActiveHoursEnd() -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn UserActiveHoursMax() -> ::windows::runtime::Result<i32> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn TrySetUserActiveHours<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(start: Param0, end: Param1) -> ::windows::runtime::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), start.into_param().abi(), end.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn LastUpdateCheckTime() -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn LastUpdateInstallTime() -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn LastErrorInfo() -> ::windows::runtime::Result<SystemUpdateLastErrorInfo> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateLastErrorInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Update`, `Foundation_Collections`*"]
    pub fn GetAutomaticRebootBlockIds() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn BlockAutomaticRebootAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(lockid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), lockid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Update`, `Foundation`*"]
    pub fn UnblockAutomaticRebootAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(lockid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), lockid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn ExtendedError() -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Update`, `Foundation_Collections`*"]
    pub fn GetUpdateItems() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn AttentionRequiredReason() -> ::windows::runtime::Result<SystemUpdateAttentionRequiredReason> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: SystemUpdateAttentionRequiredReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateAttentionRequiredReason>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn SetFlightRing<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(flightring: Param0) -> ::windows::runtime::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), flightring.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn GetFlightRing() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn StartInstall(action: SystemUpdateStartInstallAction) -> ::windows::runtime::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), action).ok() })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn RebootToCompleteInstall() -> ::windows::runtime::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `System_Update`*"]
    pub fn StartCancelUpdates() -> ::windows::runtime::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok() })
    }
    pub fn ISystemUpdateManagerStatics<R, F: FnOnce(&ISystemUpdateManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemUpdateManager, ISystemUpdateManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SystemUpdateManager {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateManager";
}
#[doc = "*Required features: `System_Update`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemUpdateManagerState(pub i32);
impl SystemUpdateManagerState {
    pub const Idle: SystemUpdateManagerState = SystemUpdateManagerState(0i32);
    pub const Detecting: SystemUpdateManagerState = SystemUpdateManagerState(1i32);
    pub const ReadyToDownload: SystemUpdateManagerState = SystemUpdateManagerState(2i32);
    pub const Downloading: SystemUpdateManagerState = SystemUpdateManagerState(3i32);
    pub const ReadyToInstall: SystemUpdateManagerState = SystemUpdateManagerState(4i32);
    pub const Installing: SystemUpdateManagerState = SystemUpdateManagerState(5i32);
    pub const RebootRequired: SystemUpdateManagerState = SystemUpdateManagerState(6i32);
    pub const ReadyToFinalize: SystemUpdateManagerState = SystemUpdateManagerState(7i32);
    pub const Finalizing: SystemUpdateManagerState = SystemUpdateManagerState(8i32);
    pub const Completed: SystemUpdateManagerState = SystemUpdateManagerState(9i32);
    pub const AttentionRequired: SystemUpdateManagerState = SystemUpdateManagerState(10i32);
    pub const Error: SystemUpdateManagerState = SystemUpdateManagerState(11i32);
}
impl ::std::convert::From<i32> for SystemUpdateManagerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemUpdateManagerState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemUpdateManagerState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateManagerState;i4)");
}
impl ::windows::runtime::DefaultType for SystemUpdateManagerState {
    type DefaultType = Self;
}
#[doc = "*Required features: `System_Update`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemUpdateStartInstallAction(pub i32);
impl SystemUpdateStartInstallAction {
    pub const UpToReboot: SystemUpdateStartInstallAction = SystemUpdateStartInstallAction(0i32);
    pub const AllowReboot: SystemUpdateStartInstallAction = SystemUpdateStartInstallAction(1i32);
}
impl ::std::convert::From<i32> for SystemUpdateStartInstallAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemUpdateStartInstallAction {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemUpdateStartInstallAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateStartInstallAction;i4)");
}
impl ::windows::runtime::DefaultType for SystemUpdateStartInstallAction {
    type DefaultType = Self;
}
