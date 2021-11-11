#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: CachedFileOptions = CachedFileOptions(0u32);
    pub const RequireUpdateOnAccess: CachedFileOptions = CachedFileOptions(1u32);
    pub const UseCachedFileWhenOffline: CachedFileOptions = CachedFileOptions(2u32);
    pub const DenyAccessWhenOffline: CachedFileOptions = CachedFileOptions(4u32);
}
impl ::core::convert::From<u32> for CachedFileOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CachedFileOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CachedFileOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileOptions;u4)");
}
impl ::windows::core::DefaultType for CachedFileOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for CachedFileOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CachedFileOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CachedFileOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CachedFileOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CachedFileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: CachedFileTarget = CachedFileTarget(0i32);
    pub const Remote: CachedFileTarget = CachedFileTarget(1i32);
}
impl ::core::convert::From<i32> for CachedFileTarget {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CachedFileTarget {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CachedFileTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileTarget;i4)");
}
impl ::windows::core::DefaultType for CachedFileTarget {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
pub struct CachedFileUpdater {}
impl CachedFileUpdater {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetUpdateInformation<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contentid: Param1, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::Result<()> {
        Self::ICachedFileUpdaterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), contentid.into_param().abi(), readmode, writemode, options).ok() })
    }
    pub fn ICachedFileUpdaterStatics<R, F: FnOnce(&ICachedFileUpdaterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CachedFileUpdater, ICachedFileUpdaterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CachedFileUpdater {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdater";
}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CachedFileUpdaterUI(pub ::windows::core::IInspectable);
impl CachedFileUpdaterUI {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn UpdateTarget(&self) -> ::windows::core::Result<CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__: CachedFileTarget = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CachedFileTarget>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn FileUpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn RemoveFileUpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn UIRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn RemoveUIRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn UIStatus(&self) -> ::windows::core::Result<UIStatus> {
        let this = self;
        unsafe {
            let mut result__: UIStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UIStatus>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn UpdateRequest(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = &::windows::core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequest>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = &::windows::core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequestDeferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.CachedFileUpdaterUI;{9e6f41e6-baf2-4a97-b600-9333f5df80fd})");
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6f41e6_baf2_4a97_b600_9333f5df80fd);
}
impl ::windows::core::RuntimeName for CachedFileUpdaterUI {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdaterUI";
}
impl ::core::convert::From<CachedFileUpdaterUI> for ::windows::core::IUnknown {
    fn from(value: CachedFileUpdaterUI) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CachedFileUpdaterUI> for ::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterUI) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CachedFileUpdaterUI> for ::windows::core::IInspectable {
    fn from(value: CachedFileUpdaterUI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CachedFileUpdaterUI> for ::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct CloudFilesContract(pub u8);
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileUpdateRequest(pub ::windows::core::IInspectable);
impl FileUpdateRequest {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn File(&self) -> ::windows::core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Status(&self) -> ::windows::core::Result<FileUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__: FileUpdateStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateStatus>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetStatus(&self, value: FileUpdateStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequestDeferral>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn UpdateLocalFile<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn UserInputNeededMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetUserInputNeededMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequest;{40c82536-c1fe-4d93-a792-1e736bc70837})");
}
unsafe impl ::windows::core::Interface for FileUpdateRequest {
    type Vtable = IFileUpdateRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40c82536_c1fe_4d93_a792_1e736bc70837);
}
impl ::windows::core::RuntimeName for FileUpdateRequest {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequest";
}
impl ::core::convert::From<FileUpdateRequest> for ::windows::core::IUnknown {
    fn from(value: FileUpdateRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileUpdateRequest> for ::windows::core::IUnknown {
    fn from(value: &FileUpdateRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileUpdateRequest> for ::windows::core::IInspectable {
    fn from(value: FileUpdateRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileUpdateRequest> for ::windows::core::IInspectable {
    fn from(value: &FileUpdateRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileUpdateRequestDeferral(pub ::windows::core::IInspectable);
impl FileUpdateRequestDeferral {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestDeferral;{ffcedb2b-8ade-44a5-bb00-164c4e72f13a})");
}
unsafe impl ::windows::core::Interface for FileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffcedb2b_8ade_44a5_bb00_164c4e72f13a);
}
impl ::windows::core::RuntimeName for FileUpdateRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestDeferral";
}
impl ::core::convert::From<FileUpdateRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: FileUpdateRequestDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileUpdateRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &FileUpdateRequestDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileUpdateRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: FileUpdateRequestDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileUpdateRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &FileUpdateRequestDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileUpdateRequestedEventArgs(pub ::windows::core::IInspectable);
impl FileUpdateRequestedEventArgs {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Request(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestedEventArgs;{7b0a9342-3905-438d-aaef-78ae265f8dd2})");
}
unsafe impl ::windows::core::Interface for FileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0a9342_3905_438d_aaef_78ae265f8dd2);
}
impl ::windows::core::RuntimeName for FileUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestedEventArgs";
}
impl ::core::convert::From<FileUpdateRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileUpdateRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileUpdateRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileUpdateRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileUpdateRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileUpdateRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileUpdateRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileUpdateRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: FileUpdateStatus = FileUpdateStatus(0i32);
    pub const Complete: FileUpdateStatus = FileUpdateStatus(1i32);
    pub const UserInputNeeded: FileUpdateStatus = FileUpdateStatus(2i32);
    pub const CurrentlyUnavailable: FileUpdateStatus = FileUpdateStatus(3i32);
    pub const Failed: FileUpdateStatus = FileUpdateStatus(4i32);
    pub const CompleteAndRenamed: FileUpdateStatus = FileUpdateStatus(5i32);
}
impl ::core::convert::From<i32> for FileUpdateStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FileUpdateStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FileUpdateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.FileUpdateStatus;i4)");
}
impl ::windows::core::DefaultType for FileUpdateStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICachedFileUpdaterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterStatics {
    type Vtable = ICachedFileUpdaterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc90920_7bcf_4888_a81e_102d7034d7ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6f41e6_baf2_4a97_b600_9333f5df80fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CachedFileTarget) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UIStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI2 {
    type Vtable = ICachedFileUpdaterUI2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8856a21c_8699_4340_9f49_f7cad7fe8991);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileUpdateRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileUpdateRequest {
    type Vtable = IFileUpdateRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40c82536_c1fe_4d93_a792_1e736bc70837);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FileUpdateStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FileUpdateStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileUpdateRequest2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileUpdateRequest2 {
    type Vtable = IFileUpdateRequest2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82484648_bdbe_447b_a2ee_7afe6a032a94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileUpdateRequestDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffcedb2b_8ade_44a5_bb00_164c4e72f13a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileUpdateRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0a9342_3905_438d_aaef_78ae265f8dd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderError(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderError {
    type Vtable = IStorageProviderError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47f2780b_ef7f_5910_bf83_331d89256615);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderError_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderErrorCommand {
    type Vtable = IStorageProviderErrorCommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b18aed_bb65_5f26_86e4_1d3e34d54477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommand_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommandFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderErrorCommandFactory {
    type Vtable = IStorageProviderErrorCommandFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc1f555_3ab4_556f_8bb2_7e5515eed8dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, actionuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderErrorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderErrorFactory {
    type Vtable = IStorageProviderErrorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d6f240_61ab_51dc_9921_18bd0dbef79e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1955b9c1_0184_5a88_87df_4544f464365d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfoFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfoFactory {
    type Vtable = IStorageProviderFileTypeInfoFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa12c6f_cce6_5d5d_80b1_389e7cf92dbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconresource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderGetContentInfoForPathResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2564711d_aa89_4d12_82e3_f72a92e33966);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetContentInfoForPathResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderGetPathForContentUriResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63711a9d_4118_45a6_acb6_22c49d019f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetPathForContentUriResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Provider`*"]
pub struct IStorageProviderHandlerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderHandlerFactory {
    type Vtable = IStorageProviderHandlerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6154dc3a_fc1d_5aae_9e23_e8659a22c5f6);
}
impl IStorageProviderHandlerFactory {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetStatusSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, syncrootid: Param0) -> ::windows::core::Result<IStorageProviderStatusSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), syncrootid.into_param().abi(), &mut result__).from_abi::<IStorageProviderStatusSource>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderHandlerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6154dc3a-fc1d-5aae-9e23-e8659a22c5f6}");
}
impl ::core::convert::From<IStorageProviderHandlerFactory> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderHandlerFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageProviderHandlerFactory> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderHandlerFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageProviderHandlerFactory> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderHandlerFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageProviderHandlerFactory> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderHandlerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderHandlerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertiesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertiesStatics {
    type Vtable = IStorageProviderItemPropertiesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d2c1c97_2704_4729_8fa9_7e6b8e158c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::windows::core::RawPtr, itemproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderItemProperty(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476cb558_730b_4188_b7b5_63b716ed476d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertyDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b383bb_ff1f_4298_831e_ff1c08089690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertyDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Provider`*"]
pub struct IStorageProviderItemPropertySource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertySource {
    type Vtable = IStorageProviderItemPropertySource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f6f9c3e_f632_4a9b_8d99_d2d7a11df56a);
}
impl IStorageProviderItemPropertySource {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation_Collections`*"]
    pub fn GetItemProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, itempath: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), itempath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderItemPropertySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8f6f9c3e-f632-4a9b-8d99-d2d7a11df56a}");
}
impl ::core::convert::From<IStorageProviderItemPropertySource> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderItemPropertySource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageProviderItemPropertySource> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderItemPropertySource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageProviderItemPropertySource> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderItemPropertySource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageProviderItemPropertySource> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderItemPropertySource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertySource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itempath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Provider`*"]
pub struct IStorageProviderPropertyCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderPropertyCapabilities {
    type Vtable = IStorageProviderPropertyCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x658d2f0e_63b7_4567_acf9_51abe301dda5);
}
impl IStorageProviderPropertyCapabilities {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn IsPropertySupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertycanonicalname: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertycanonicalname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderPropertyCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{658d2f0e-63b7-4567-acf9-51abe301dda5}");
}
impl ::core::convert::From<IStorageProviderPropertyCapabilities> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderPropertyCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageProviderPropertyCapabilities> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderPropertyCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageProviderPropertyCapabilities> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderPropertyCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageProviderPropertyCapabilities> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderPropertyCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderPropertyCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderStatus(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderStatus {
    type Vtable = IStorageProviderStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff6e761d_fb8b_56c3_9e7a_05309d191fb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderStatusFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderStatusFactory {
    type Vtable = IStorageProviderStatusFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd64828c5_9b7a_5fa4_b126_90bd18936c7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, errormessages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Provider`*"]
pub struct IStorageProviderStatusSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderStatusSource {
    type Vtable = IStorageProviderStatusSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e316bb2_fd43_5335_b3c4_a962ee31d17e);
}
impl IStorageProviderStatusSource {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetStatus(&self) -> ::windows::core::Result<StorageProviderStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderStatusSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2e316bb2-fd43-5335-b3c4-a962ee31d17e}");
}
impl ::core::convert::From<IStorageProviderStatusSource> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderStatusSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageProviderStatusSource> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderStatusSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageProviderStatusSource> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderStatusSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageProviderStatusSource> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderStatusSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c1305c4_99f9_41ac_8904_ab055d654926);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo2 {
    type Vtable = IStorageProviderSyncRootInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf51b023_7cf1_5166_bdba_efd95f529e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo3 {
    type Vtable = IStorageProviderSyncRootInfo3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507a6617_bef6_56fd_855e_75ace2e45cf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics {
    type Vtable = IStorageProviderSyncRootManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e99fbbf_8fe3_4b40_abc7_f6fc3d74c98e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, syncrootinformation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics2 {
    type Vtable = IStorageProviderSyncRootManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefb6cfee_1374_544e_9df1_5598d2e9cfdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Provider`*"]
pub struct IStorageProviderUriSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProviderUriSource {
    type Vtable = IStorageProviderUriSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb29806d1_8be0_4962_8bb6_0d4c2e14d47a);
}
impl IStorageProviderUriSource {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetPathForContentUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, StorageProviderGetPathForContentUriResult>>(&self, contenturi: Param0, result: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contenturi.into_param().abi(), result.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetContentInfoForPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, StorageProviderGetContentInfoForPathResult>>(&self, path: Param0, result: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), path.into_param().abi(), result.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderUriSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b29806d1-8be0-4962-8bb6-0d4c2e14d47a}");
}
impl ::core::convert::From<IStorageProviderUriSource> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderUriSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageProviderUriSource> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderUriSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageProviderUriSource> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderUriSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageProviderUriSource> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderUriSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderUriSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contenturi: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: ReadActivationMode = ReadActivationMode(0i32);
    pub const BeforeAccess: ReadActivationMode = ReadActivationMode(1i32);
}
impl ::core::convert::From<i32> for ReadActivationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ReadActivationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ReadActivationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.ReadActivationMode;i4)");
}
impl ::windows::core::DefaultType for ReadActivationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderError(pub ::windows::core::IInspectable);
impl StorageProviderError {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn FilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetFilePath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn PrimaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetPrimaryAction<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderErrorCommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SecondaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetSecondaryAction<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderErrorCommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn InformationalLink(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetInformationalLink<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderErrorCommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0, title: Param1, message: Param2) -> ::windows::core::Result<StorageProviderError> {
        Self::IStorageProviderErrorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), title.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<StorageProviderError>(result__)
        })
    }
    pub fn IStorageProviderErrorFactory<R, F: FnOnce(&IStorageProviderErrorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderError, IStorageProviderErrorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderError;{47f2780b-ef7f-5910-bf83-331d89256615})");
}
unsafe impl ::windows::core::Interface for StorageProviderError {
    type Vtable = IStorageProviderError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47f2780b_ef7f_5910_bf83_331d89256615);
}
impl ::windows::core::RuntimeName for StorageProviderError {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderError";
}
impl ::core::convert::From<StorageProviderError> for ::windows::core::IUnknown {
    fn from(value: StorageProviderError) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderError> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderError) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderError> for ::windows::core::IInspectable {
    fn from(value: StorageProviderError) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderError> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderError {}
unsafe impl ::core::marker::Sync for StorageProviderError {}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderErrorCommand(pub ::windows::core::IInspectable);
impl StorageProviderErrorCommand {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn ActionUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(label: Param0, actionuri: Param1) -> ::windows::core::Result<StorageProviderErrorCommand> {
        Self::IStorageProviderErrorCommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), label.into_param().abi(), actionuri.into_param().abi(), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        })
    }
    pub fn IStorageProviderErrorCommandFactory<R, F: FnOnce(&IStorageProviderErrorCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderErrorCommand, IStorageProviderErrorCommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderErrorCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderErrorCommand;{b6b18aed-bb65-5f26-86e4-1d3e34d54477})");
}
unsafe impl ::windows::core::Interface for StorageProviderErrorCommand {
    type Vtable = IStorageProviderErrorCommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b18aed_bb65_5f26_86e4_1d3e34d54477);
}
impl ::windows::core::RuntimeName for StorageProviderErrorCommand {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderErrorCommand";
}
impl ::core::convert::From<StorageProviderErrorCommand> for ::windows::core::IUnknown {
    fn from(value: StorageProviderErrorCommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderErrorCommand> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderErrorCommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderErrorCommand> for ::windows::core::IInspectable {
    fn from(value: StorageProviderErrorCommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderErrorCommand> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderErrorCommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderErrorCommand {}
unsafe impl ::core::marker::Sync for StorageProviderErrorCommand {}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderFileTypeInfo(pub ::windows::core::IInspectable);
impl StorageProviderFileTypeInfo {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn FileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(fileextension: Param0, iconresource: Param1) -> ::windows::core::Result<StorageProviderFileTypeInfo> {
        Self::IStorageProviderFileTypeInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), fileextension.into_param().abi(), iconresource.into_param().abi(), &mut result__).from_abi::<StorageProviderFileTypeInfo>(result__)
        })
    }
    pub fn IStorageProviderFileTypeInfoFactory<R, F: FnOnce(&IStorageProviderFileTypeInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderFileTypeInfo, IStorageProviderFileTypeInfoFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderFileTypeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderFileTypeInfo;{1955b9c1-0184-5a88-87df-4544f464365d})");
}
unsafe impl ::windows::core::Interface for StorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1955b9c1_0184_5a88_87df_4544f464365d);
}
impl ::windows::core::RuntimeName for StorageProviderFileTypeInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderFileTypeInfo";
}
impl ::core::convert::From<StorageProviderFileTypeInfo> for ::windows::core::IUnknown {
    fn from(value: StorageProviderFileTypeInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderFileTypeInfo> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderFileTypeInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderFileTypeInfo> for ::windows::core::IInspectable {
    fn from(value: StorageProviderFileTypeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderFileTypeInfo> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderFileTypeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderFileTypeInfo {}
unsafe impl ::core::marker::Sync for StorageProviderFileTypeInfo {}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderGetContentInfoForPathResult(pub ::windows::core::IInspectable);
impl StorageProviderGetContentInfoForPathResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderGetContentInfoForPathResult, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderUriSourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderUriSourceStatus>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn ContentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetContentUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetContentId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderGetContentInfoForPathResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult;{2564711d-aa89-4d12-82e3-f72a92e33966})");
}
unsafe impl ::windows::core::Interface for StorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2564711d_aa89_4d12_82e3_f72a92e33966);
}
impl ::windows::core::RuntimeName for StorageProviderGetContentInfoForPathResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult";
}
impl ::core::convert::From<StorageProviderGetContentInfoForPathResult> for ::windows::core::IUnknown {
    fn from(value: StorageProviderGetContentInfoForPathResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderGetContentInfoForPathResult> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderGetContentInfoForPathResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderGetContentInfoForPathResult> for ::windows::core::IInspectable {
    fn from(value: StorageProviderGetContentInfoForPathResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderGetContentInfoForPathResult> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderGetContentInfoForPathResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderGetContentInfoForPathResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetContentInfoForPathResult {}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderGetPathForContentUriResult(pub ::windows::core::IInspectable);
impl StorageProviderGetPathForContentUriResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderGetPathForContentUriResult, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderUriSourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderUriSourceStatus>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderGetPathForContentUriResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetPathForContentUriResult;{63711a9d-4118-45a6-acb6-22c49d019f40})");
}
unsafe impl ::windows::core::Interface for StorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63711a9d_4118_45a6_acb6_22c49d019f40);
}
impl ::windows::core::RuntimeName for StorageProviderGetPathForContentUriResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetPathForContentUriResult";
}
impl ::core::convert::From<StorageProviderGetPathForContentUriResult> for ::windows::core::IUnknown {
    fn from(value: StorageProviderGetPathForContentUriResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderGetPathForContentUriResult> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderGetPathForContentUriResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderGetPathForContentUriResult> for ::windows::core::IInspectable {
    fn from(value: StorageProviderGetPathForContentUriResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderGetPathForContentUriResult> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderGetPathForContentUriResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderGetPathForContentUriResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetPathForContentUriResult {}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: StorageProviderHardlinkPolicy = StorageProviderHardlinkPolicy(0u32);
    pub const Allowed: StorageProviderHardlinkPolicy = StorageProviderHardlinkPolicy(1u32);
}
impl ::core::convert::From<u32> for StorageProviderHardlinkPolicy {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHardlinkPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHardlinkPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHardlinkPolicy;u4)");
}
impl ::windows::core::DefaultType for StorageProviderHardlinkPolicy {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHardlinkPolicy {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHardlinkPolicy {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(0i32);
    pub const Progressive: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(1i32);
    pub const Full: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(2i32);
    pub const AlwaysFull: StorageProviderHydrationPolicy = StorageProviderHydrationPolicy(3i32);
}
impl ::core::convert::From<i32> for StorageProviderHydrationPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHydrationPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHydrationPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicy;i4)");
}
impl ::windows::core::DefaultType for StorageProviderHydrationPolicy {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(0u32);
    pub const ValidationRequired: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(1u32);
    pub const StreamingAllowed: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(2u32);
    pub const AutoDehydrationAllowed: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(4u32);
    pub const AllowFullRestartHydration: StorageProviderHydrationPolicyModifier = StorageProviderHydrationPolicyModifier(8u32);
}
impl ::core::convert::From<u32> for StorageProviderHydrationPolicyModifier {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHydrationPolicyModifier {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHydrationPolicyModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicyModifier;u4)");
}
impl ::windows::core::DefaultType for StorageProviderHydrationPolicyModifier {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHydrationPolicyModifier {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHydrationPolicyModifier {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(0u32);
    pub const FileCreationTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(1u32);
    pub const FileReadOnlyAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(2u32);
    pub const FileHiddenAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(4u32);
    pub const FileSystemAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(8u32);
    pub const DirectoryCreationTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(16u32);
    pub const DirectoryReadOnlyAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(32u32);
    pub const DirectoryHiddenAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(64u32);
    pub const DirectorySystemAttribute: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(128u32);
    pub const FileLastWriteTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(256u32);
    pub const DirectoryLastWriteTime: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(512u32);
    pub const PreserveInsyncForSyncEngine: StorageProviderInSyncPolicy = StorageProviderInSyncPolicy(2147483648u32);
}
impl ::core::convert::From<u32> for StorageProviderInSyncPolicy {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderInSyncPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderInSyncPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderInSyncPolicy;u4)");
}
impl ::windows::core::DefaultType for StorageProviderInSyncPolicy {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderInSyncPolicy {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderInSyncPolicy {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for StorageProviderInSyncPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage_Provider`*"]
pub struct StorageProviderItemProperties {}
impl StorageProviderItemProperties {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>>(item: Param0, itemproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IStorageProviderItemPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), item.into_param().abi(), itemproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IStorageProviderItemPropertiesStatics<R, F: FnOnce(&IStorageProviderItemPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderItemProperties, IStorageProviderItemPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorageProviderItemProperties {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperties";
}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderItemProperty(pub ::windows::core::IInspectable);
impl StorageProviderItemProperty {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderItemProperty, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetIconResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderItemProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemProperty;{476cb558-730b-4188-b7b5-63b716ed476d})");
}
unsafe impl ::windows::core::Interface for StorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476cb558_730b_4188_b7b5_63b716ed476d);
}
impl ::windows::core::RuntimeName for StorageProviderItemProperty {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperty";
}
impl ::core::convert::From<StorageProviderItemProperty> for ::windows::core::IUnknown {
    fn from(value: StorageProviderItemProperty) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderItemProperty> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderItemProperty) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderItemProperty> for ::windows::core::IInspectable {
    fn from(value: StorageProviderItemProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderItemProperty> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderItemProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderItemProperty {}
unsafe impl ::core::marker::Sync for StorageProviderItemProperty {}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderItemPropertyDefinition(pub ::windows::core::IInspectable);
impl StorageProviderItemPropertyDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderItemPropertyDefinition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetDisplayNameResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderItemPropertyDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemPropertyDefinition;{c5b383bb-ff1f-4298-831e-ff1c08089690})");
}
unsafe impl ::windows::core::Interface for StorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b383bb_ff1f_4298_831e_ff1c08089690);
}
impl ::windows::core::RuntimeName for StorageProviderItemPropertyDefinition {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemPropertyDefinition";
}
impl ::core::convert::From<StorageProviderItemPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: StorageProviderItemPropertyDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderItemPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderItemPropertyDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderItemPropertyDefinition> for ::windows::core::IInspectable {
    fn from(value: StorageProviderItemPropertyDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderItemPropertyDefinition> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderItemPropertyDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderItemPropertyDefinition {}
unsafe impl ::core::marker::Sync for StorageProviderItemPropertyDefinition {}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: StorageProviderPopulationPolicy = StorageProviderPopulationPolicy(1i32);
    pub const AlwaysFull: StorageProviderPopulationPolicy = StorageProviderPopulationPolicy(2i32);
}
impl ::core::convert::From<i32> for StorageProviderPopulationPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderPopulationPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderPopulationPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderPopulationPolicy;i4)");
}
impl ::windows::core::DefaultType for StorageProviderPopulationPolicy {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: StorageProviderProtectionMode = StorageProviderProtectionMode(0i32);
    pub const Personal: StorageProviderProtectionMode = StorageProviderProtectionMode(1i32);
}
impl ::core::convert::From<i32> for StorageProviderProtectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderProtectionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderProtectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderProtectionMode;i4)");
}
impl ::windows::core::DefaultType for StorageProviderProtectionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: StorageProviderState = StorageProviderState(0i32);
    pub const Syncing: StorageProviderState = StorageProviderState(1i32);
    pub const Paused: StorageProviderState = StorageProviderState(2i32);
    pub const Error: StorageProviderState = StorageProviderState(3i32);
    pub const Warning: StorageProviderState = StorageProviderState(4i32);
    pub const Offline: StorageProviderState = StorageProviderState(5i32);
}
impl ::core::convert::From<i32> for StorageProviderState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderState;i4)");
}
impl ::windows::core::DefaultType for StorageProviderState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderStatus(pub ::windows::core::IInspectable);
impl StorageProviderStatus {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn State(&self) -> ::windows::core::Result<StorageProviderState> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderState>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation_Collections`*"]
    pub fn ErrorMessages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderError>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StorageProviderError>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn CreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(state: StorageProviderState, message: Param1) -> ::windows::core::Result<StorageProviderStatus> {
        Self::IStorageProviderStatusFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), state, message.into_param().abi(), &mut result__).from_abi::<StorageProviderStatus>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation_Collections`*"]
    pub fn CreateInstance2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorageProviderError>>>(state: StorageProviderState, message: Param1, errormessages: Param2) -> ::windows::core::Result<StorageProviderStatus> {
        Self::IStorageProviderStatusFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), state, message.into_param().abi(), errormessages.into_param().abi(), &mut result__).from_abi::<StorageProviderStatus>(result__)
        })
    }
    pub fn IStorageProviderStatusFactory<R, F: FnOnce(&IStorageProviderStatusFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderStatus, IStorageProviderStatusFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderStatus;{ff6e761d-fb8b-56c3-9e7a-05309d191fb4})");
}
unsafe impl ::windows::core::Interface for StorageProviderStatus {
    type Vtable = IStorageProviderStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff6e761d_fb8b_56c3_9e7a_05309d191fb4);
}
impl ::windows::core::RuntimeName for StorageProviderStatus {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderStatus";
}
impl ::core::convert::From<StorageProviderStatus> for ::windows::core::IUnknown {
    fn from(value: StorageProviderStatus) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderStatus> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderStatus) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderStatus> for ::windows::core::IInspectable {
    fn from(value: StorageProviderStatus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderStatus> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderStatus {}
unsafe impl ::core::marker::Sync for StorageProviderStatus {}
#[doc = "*Required features: `Storage_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProviderSyncRootInfo(pub ::windows::core::IInspectable);
impl StorageProviderSyncRootInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_Provider`, `Storage_Streams`*"]
    pub fn Context(&self) -> ::windows::core::Result<super::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_Provider`, `Storage_Streams`*"]
    pub fn SetContext<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Path(&self) -> ::windows::core::Result<super::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IStorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetDisplayNameResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetIconResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn HydrationPolicy(&self) -> ::windows::core::Result<StorageProviderHydrationPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderHydrationPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderHydrationPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetHydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn HydrationPolicyModifier(&self) -> ::windows::core::Result<StorageProviderHydrationPolicyModifier> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderHydrationPolicyModifier = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderHydrationPolicyModifier>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetHydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn PopulationPolicy(&self) -> ::windows::core::Result<StorageProviderPopulationPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderPopulationPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderPopulationPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetPopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn InSyncPolicy(&self) -> ::windows::core::Result<StorageProviderInSyncPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderInSyncPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderInSyncPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetInSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn HardlinkPolicy(&self) -> ::windows::core::Result<StorageProviderHardlinkPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderHardlinkPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderHardlinkPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetHardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn ShowSiblingsAsGroup(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetShowSiblingsAsGroup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn ProtectionMode(&self) -> ::windows::core::Result<StorageProviderProtectionMode> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderProtectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderProtectionMode>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetProtectionMode(&self, value: StorageProviderProtectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn AllowPinning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetAllowPinning(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation_Collections`*"]
    pub fn StorageProviderItemPropertyDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn RecycleBinUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation`*"]
    pub fn SetRecycleBinUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn SetProviderId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation_Collections`*"]
    pub fn FallbackFileTypeInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderSyncRootInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderSyncRootInfo;{7c1305c4-99f9-41ac-8904-ab055d654926})");
}
unsafe impl ::windows::core::Interface for StorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c1305c4_99f9_41ac_8904_ab055d654926);
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootInfo";
}
impl ::core::convert::From<StorageProviderSyncRootInfo> for ::windows::core::IUnknown {
    fn from(value: StorageProviderSyncRootInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProviderSyncRootInfo> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderSyncRootInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProviderSyncRootInfo> for ::windows::core::IInspectable {
    fn from(value: StorageProviderSyncRootInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProviderSyncRootInfo> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderSyncRootInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageProviderSyncRootInfo {}
unsafe impl ::core::marker::Sync for StorageProviderSyncRootInfo {}
#[doc = "*Required features: `Storage_Provider`*"]
pub struct StorageProviderSyncRootManager {}
impl StorageProviderSyncRootManager {
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Register<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderSyncRootInfo>>(syncrootinformation: Param0) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), syncrootinformation.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn Unregister<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetSyncRootInformationForFolder<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>>(folder: Param0) -> ::windows::core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<StorageProviderSyncRootInfo>(result__)
        })
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn GetSyncRootInformationForId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<StorageProviderSyncRootInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Provider`, `Foundation_Collections`*"]
    pub fn GetCurrentSyncRoots() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>>(result__)
        })
    }
    #[doc = "*Required features: `Storage_Provider`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IStorageProviderSyncRootManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IStorageProviderSyncRootManagerStatics<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStorageProviderSyncRootManagerStatics2<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootManager {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootManager";
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: StorageProviderUriSourceStatus = StorageProviderUriSourceStatus(0i32);
    pub const NoSyncRoot: StorageProviderUriSourceStatus = StorageProviderUriSourceStatus(1i32);
    pub const FileNotFound: StorageProviderUriSourceStatus = StorageProviderUriSourceStatus(2i32);
}
impl ::core::convert::From<i32> for StorageProviderUriSourceStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderUriSourceStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageProviderUriSourceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUriSourceStatus;i4)");
}
impl ::windows::core::DefaultType for StorageProviderUriSourceStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: UIStatus = UIStatus(0i32);
    pub const Hidden: UIStatus = UIStatus(1i32);
    pub const Visible: UIStatus = UIStatus(2i32);
    pub const Complete: UIStatus = UIStatus(3i32);
}
impl ::core::convert::From<i32> for UIStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UIStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UIStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.UIStatus;i4)");
}
impl ::windows::core::DefaultType for UIStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: WriteActivationMode = WriteActivationMode(0i32);
    pub const NotNeeded: WriteActivationMode = WriteActivationMode(1i32);
    pub const AfterWrite: WriteActivationMode = WriteActivationMode(2i32);
}
impl ::core::convert::From<i32> for WriteActivationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WriteActivationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WriteActivationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.WriteActivationMode;i4)");
}
impl ::windows::core::DefaultType for WriteActivationMode {
    type DefaultType = Self;
}
