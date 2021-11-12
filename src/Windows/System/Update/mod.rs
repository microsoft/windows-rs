#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemUpdateItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemUpdateItem {
    type Vtable = ISystemUpdateItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x779740eb_5624_519e_a8e2_09e9173b3fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SystemUpdateItemState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemUpdateLastErrorInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ee887f7_8a44_5b6e_bd07_7aece4116ea9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateLastErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SystemUpdateManagerState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemUpdateManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemUpdateManagerStatics {
    type Vtable = ISystemUpdateManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2d3fcef_2971_51be_b41a_8bd703bb701a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SystemUpdateManagerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, start: super::super::Foundation::TimeSpan, end: super::super::Foundation::TimeSpan, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lockid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lockid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SystemUpdateAttentionRequiredReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flightring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: SystemUpdateStartInstallAction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemUpdateAttentionRequiredReason(pub i32);
impl SystemUpdateAttentionRequiredReason {
    pub const None: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(0i32);
    pub const NetworkRequired: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(1i32);
    pub const InsufficientDiskSpace: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(2i32);
    pub const InsufficientBattery: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(3i32);
    pub const UpdateBlocked: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(4i32);
}
impl ::core::convert::From<i32> for SystemUpdateAttentionRequiredReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateAttentionRequiredReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateAttentionRequiredReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateAttentionRequiredReason;i4)");
}
impl ::windows::core::DefaultType for SystemUpdateAttentionRequiredReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemUpdateItem(pub ::windows::core::IInspectable);
impl SystemUpdateItem {
    pub fn State(&self) -> ::windows::core::Result<SystemUpdateItemState> {
        let this = self;
        unsafe {
            let mut result__: SystemUpdateItemState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateItemState>(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn DownloadProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn InstallProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateItem;{779740eb-5624-519e-a8e2-09e9173b3fb7})");
}
unsafe impl ::windows::core::Interface for SystemUpdateItem {
    type Vtable = ISystemUpdateItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x779740eb_5624_519e_a8e2_09e9173b3fb7);
}
impl ::windows::core::RuntimeName for SystemUpdateItem {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateItem";
}
impl ::core::convert::From<SystemUpdateItem> for ::windows::core::IUnknown {
    fn from(value: SystemUpdateItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemUpdateItem> for ::windows::core::IUnknown {
    fn from(value: &SystemUpdateItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemUpdateItem> for ::windows::core::IInspectable {
    fn from(value: SystemUpdateItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemUpdateItem> for ::windows::core::IInspectable {
    fn from(value: &SystemUpdateItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemUpdateItem {}
unsafe impl ::core::marker::Sync for SystemUpdateItem {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for SystemUpdateItemState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateItemState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateItemState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateItemState;i4)");
}
impl ::windows::core::DefaultType for SystemUpdateItemState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemUpdateLastErrorInfo(pub ::windows::core::IInspectable);
impl SystemUpdateLastErrorInfo {
    pub fn State(&self) -> ::windows::core::Result<SystemUpdateManagerState> {
        let this = self;
        unsafe {
            let mut result__: SystemUpdateManagerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateManagerState>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn IsInteractive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateLastErrorInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateLastErrorInfo;{7ee887f7-8a44-5b6e-bd07-7aece4116ea9})");
}
unsafe impl ::windows::core::Interface for SystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ee887f7_8a44_5b6e_bd07_7aece4116ea9);
}
impl ::windows::core::RuntimeName for SystemUpdateLastErrorInfo {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateLastErrorInfo";
}
impl ::core::convert::From<SystemUpdateLastErrorInfo> for ::windows::core::IUnknown {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemUpdateLastErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemUpdateLastErrorInfo> for ::windows::core::IInspectable {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemUpdateLastErrorInfo> for ::windows::core::IInspectable {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemUpdateLastErrorInfo {}
unsafe impl ::core::marker::Sync for SystemUpdateLastErrorInfo {}
pub struct SystemUpdateManager {}
impl SystemUpdateManager {
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn State() -> ::windows::core::Result<SystemUpdateManagerState> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: SystemUpdateManagerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateManagerState>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn DownloadProgress() -> ::windows::core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    pub fn InstallProgress() -> ::windows::core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn UserActiveHoursStart() -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn UserActiveHoursEnd() -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    pub fn UserActiveHoursMax() -> ::windows::core::Result<i32> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn TrySetUserActiveHours<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(start: Param0, end: Param1) -> ::windows::core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), start.into_param().abi(), end.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LastUpdateCheckTime() -> ::windows::core::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LastUpdateInstallTime() -> ::windows::core::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        })
    }
    pub fn LastErrorInfo() -> ::windows::core::Result<SystemUpdateLastErrorInfo> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateLastErrorInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAutomaticRebootBlockIds() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn BlockAutomaticRebootAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(lockid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), lockid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn UnblockAutomaticRebootAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(lockid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), lockid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ExtendedError() -> ::windows::core::Result<::windows::core::HRESULT> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUpdateItems() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>>(result__)
        })
    }
    pub fn AttentionRequiredReason() -> ::windows::core::Result<SystemUpdateAttentionRequiredReason> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: SystemUpdateAttentionRequiredReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateAttentionRequiredReason>(result__)
        })
    }
    pub fn SetFlightRing<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(flightring: Param0) -> ::windows::core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), flightring.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn GetFlightRing() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn StartInstall(action: SystemUpdateStartInstallAction) -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), action).ok() })
    }
    pub fn RebootToCompleteInstall() -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn StartCancelUpdates() -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn ISystemUpdateManagerStatics<R, F: FnOnce(&ISystemUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemUpdateManager, ISystemUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SystemUpdateManager {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateManager";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for SystemUpdateManagerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateManagerState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateManagerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateManagerState;i4)");
}
impl ::windows::core::DefaultType for SystemUpdateManagerState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemUpdateStartInstallAction(pub i32);
impl SystemUpdateStartInstallAction {
    pub const UpToReboot: SystemUpdateStartInstallAction = SystemUpdateStartInstallAction(0i32);
    pub const AllowReboot: SystemUpdateStartInstallAction = SystemUpdateStartInstallAction(1i32);
}
impl ::core::convert::From<i32> for SystemUpdateStartInstallAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateStartInstallAction {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateStartInstallAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateStartInstallAction;i4)");
}
impl ::windows::core::DefaultType for SystemUpdateStartInstallAction {
    type DefaultType = Self;
}
