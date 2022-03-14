#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemUpdateItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemUpdateItem {
    type Vtable = ISystemUpdateItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x779740eb_5624_519e_a8e2_09e9173b3fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateItem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateItemState) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemUpdateLastErrorInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ee887f7_8a44_5b6e_bd07_7aece4116ea9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateLastErrorInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateManagerState) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub IsInteractive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemUpdateManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemUpdateManagerStatics {
    type Vtable = ISystemUpdateManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2d3fcef_2971_51be_b41a_8bd703bb701a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateManagerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserActiveHoursStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserActiveHoursStart: usize,
    #[cfg(feature = "Foundation")]
    pub UserActiveHoursEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserActiveHoursEnd: usize,
    pub UserActiveHoursMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TrySetUserActiveHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: super::super::Foundation::TimeSpan, end: super::super::Foundation::TimeSpan, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetUserActiveHours: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdateCheckTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdateCheckTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdateInstallTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdateInstallTime: usize,
    pub LastErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAutomaticRebootBlockIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAutomaticRebootBlockIds: usize,
    #[cfg(feature = "Foundation")]
    pub BlockAutomaticRebootAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BlockAutomaticRebootAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnblockAutomaticRebootAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnblockAutomaticRebootAsync: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUpdateItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUpdateItems: usize,
    pub AttentionRequiredReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateAttentionRequiredReason) -> ::windows::core::HRESULT,
    pub SetFlightRing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flightring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetFlightRing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StartInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: SystemUpdateStartInstallAction) -> ::windows::core::HRESULT,
    pub RebootToCompleteInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartCancelUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateAttentionRequiredReason(pub i32);
impl SystemUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const NetworkRequired: Self = Self(1i32);
    pub const InsufficientDiskSpace: Self = Self(2i32);
    pub const InsufficientBattery: Self = Self(3i32);
    pub const UpdateBlocked: Self = Self(4i32);
}
impl ::core::marker::Copy for SystemUpdateAttentionRequiredReason {}
impl ::core::clone::Clone for SystemUpdateAttentionRequiredReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateAttentionRequiredReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateAttentionRequiredReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateAttentionRequiredReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateAttentionRequiredReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateAttentionRequiredReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateAttentionRequiredReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
pub struct SystemUpdateItem(::windows::core::IUnknown);
impl SystemUpdateItem {
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn State(&self) -> ::windows::core::Result<SystemUpdateItemState> {
        let this = self;
        unsafe {
            let mut result__: SystemUpdateItemState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateItemState>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn Revision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Revision)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn DownloadProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn InstallProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemUpdateItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemUpdateItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemUpdateItem {}
impl ::core::fmt::Debug for SystemUpdateItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateItem;{779740eb-5624-519e-a8e2-09e9173b3fb7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemUpdateItem {
    type Vtable = ISystemUpdateItem_Vtbl;
    const IID: ::windows::core::GUID = <ISystemUpdateItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemUpdateItem {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateItem";
}
impl ::core::convert::From<SystemUpdateItem> for ::windows::core::IUnknown {
    fn from(value: SystemUpdateItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateItem> for ::windows::core::IUnknown {
    fn from(value: &SystemUpdateItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemUpdateItem> for ::windows::core::IInspectable {
    fn from(value: SystemUpdateItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateItem> for ::windows::core::IInspectable {
    fn from(value: &SystemUpdateItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemUpdateItem {}
unsafe impl ::core::marker::Sync for SystemUpdateItem {}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateItemState(pub i32);
impl SystemUpdateItemState {
    pub const NotStarted: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const Preparing: Self = Self(2i32);
    pub const Calculating: Self = Self(3i32);
    pub const Downloading: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const RebootRequired: Self = Self(7i32);
    pub const Error: Self = Self(8i32);
}
impl ::core::marker::Copy for SystemUpdateItemState {}
impl ::core::clone::Clone for SystemUpdateItemState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateItemState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateItemState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateItemState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateItemState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateItemState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateItemState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
pub struct SystemUpdateLastErrorInfo(::windows::core::IUnknown);
impl SystemUpdateLastErrorInfo {
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn State(&self) -> ::windows::core::Result<SystemUpdateManagerState> {
        let this = self;
        unsafe {
            let mut result__: SystemUpdateManagerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateManagerState>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn IsInteractive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInteractive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemUpdateLastErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemUpdateLastErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemUpdateLastErrorInfo {}
impl ::core::fmt::Debug for SystemUpdateLastErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateLastErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateLastErrorInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateLastErrorInfo;{7ee887f7-8a44-5b6e-bd07-7aece4116ea9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = <ISystemUpdateLastErrorInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemUpdateLastErrorInfo {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateLastErrorInfo";
}
impl ::core::convert::From<SystemUpdateLastErrorInfo> for ::windows::core::IUnknown {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateLastErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemUpdateLastErrorInfo> for ::windows::core::IInspectable {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateLastErrorInfo> for ::windows::core::IInspectable {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemUpdateLastErrorInfo {}
unsafe impl ::core::marker::Sync for SystemUpdateLastErrorInfo {}
#[doc = "*Required features: `\"System_Update\"`*"]
pub struct SystemUpdateManager {}
impl SystemUpdateManager {
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn State() -> ::windows::core::Result<SystemUpdateManagerState> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: SystemUpdateManagerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateManagerState>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn DownloadProgress() -> ::windows::core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn InstallProgress() -> ::windows::core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstallProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserActiveHoursStart() -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserActiveHoursStart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserActiveHoursEnd() -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserActiveHoursEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn UserActiveHoursMax() -> ::windows::core::Result<i32> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserActiveHoursMax)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetUserActiveHours<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(start: Param0, end: Param1) -> ::windows::core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetUserActiveHours)(::core::mem::transmute_copy(this), start.into_param().abi(), end.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdateCheckTime() -> ::windows::core::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastUpdateCheckTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdateInstallTime() -> ::windows::core::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastUpdateInstallTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn LastErrorInfo() -> ::windows::core::Result<SystemUpdateLastErrorInfo> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastErrorInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateLastErrorInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAutomaticRebootBlockIds() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAutomaticRebootBlockIds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BlockAutomaticRebootAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(lockid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BlockAutomaticRebootAsync)(::core::mem::transmute_copy(this), lockid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnblockAutomaticRebootAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(lockid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnblockAutomaticRebootAsync)(::core::mem::transmute_copy(this), lockid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn ExtendedError() -> ::windows::core::Result<::windows::core::HRESULT> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUpdateItems() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUpdateItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn AttentionRequiredReason() -> ::windows::core::Result<SystemUpdateAttentionRequiredReason> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: SystemUpdateAttentionRequiredReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttentionRequiredReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemUpdateAttentionRequiredReason>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn SetFlightRing<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(flightring: Param0) -> ::windows::core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetFlightRing)(::core::mem::transmute_copy(this), flightring.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn GetFlightRing() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFlightRing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn StartInstall(action: SystemUpdateStartInstallAction) -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).StartInstall)(::core::mem::transmute_copy(this), action).ok() })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn RebootToCompleteInstall() -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RebootToCompleteInstall)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"System_Update\"`*"]
    pub fn StartCancelUpdates() -> ::windows::core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).StartCancelUpdates)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ISystemUpdateManagerStatics<R, F: FnOnce(&ISystemUpdateManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemUpdateManager, ISystemUpdateManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SystemUpdateManager {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateManager";
}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateManagerState(pub i32);
impl SystemUpdateManagerState {
    pub const Idle: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const ReadyToDownload: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const ReadyToInstall: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const RebootRequired: Self = Self(6i32);
    pub const ReadyToFinalize: Self = Self(7i32);
    pub const Finalizing: Self = Self(8i32);
    pub const Completed: Self = Self(9i32);
    pub const AttentionRequired: Self = Self(10i32);
    pub const Error: Self = Self(11i32);
}
impl ::core::marker::Copy for SystemUpdateManagerState {}
impl ::core::clone::Clone for SystemUpdateManagerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateManagerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateManagerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateManagerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateManagerState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateManagerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateManagerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Update\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateStartInstallAction(pub i32);
impl SystemUpdateStartInstallAction {
    pub const UpToReboot: Self = Self(0i32);
    pub const AllowReboot: Self = Self(1i32);
}
impl ::core::marker::Copy for SystemUpdateStartInstallAction {}
impl ::core::clone::Clone for SystemUpdateStartInstallAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateStartInstallAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemUpdateStartInstallAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateStartInstallAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateStartInstallAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemUpdateStartInstallAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateStartInstallAction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
