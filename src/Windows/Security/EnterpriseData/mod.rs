#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BufferProtectUnprotectResult(pub ::windows::runtime::IInspectable);
impl BufferProtectUnprotectResult {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Storage_Streams`*"]
    pub fn Buffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn ProtectionInfo(&self) -> ::windows::runtime::Result<DataProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataProtectionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BufferProtectUnprotectResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.BufferProtectUnprotectResult;{47995edc-6cec-4e3a-b251-9e7485d79e7a})");
}
unsafe impl ::windows::runtime::Interface for BufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47995edc_6cec_4e3a_b251_9e7485d79e7a);
}
impl ::windows::runtime::RuntimeName for BufferProtectUnprotectResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.BufferProtectUnprotectResult";
}
impl ::core::convert::From<BufferProtectUnprotectResult> for ::windows::runtime::IUnknown {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BufferProtectUnprotectResult> for ::windows::runtime::IUnknown {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BufferProtectUnprotectResult> for ::windows::runtime::IInspectable {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BufferProtectUnprotectResult> for ::windows::runtime::IInspectable {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BufferProtectUnprotectResult {}
unsafe impl ::core::marker::Sync for BufferProtectUnprotectResult {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DataProtectionInfo(pub ::windows::runtime::IInspectable);
impl DataProtectionInfo {
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<DataProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__: DataProtectionStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataProtectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Identity(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataProtectionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.DataProtectionInfo;{8420b0c1-5e31-4405-9540-3f943af0cb26})");
}
unsafe impl ::windows::runtime::Interface for DataProtectionInfo {
    type Vtable = IDataProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8420b0c1_5e31_4405_9540_3f943af0cb26);
}
impl ::windows::runtime::RuntimeName for DataProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionInfo";
}
impl ::core::convert::From<DataProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: DataProtectionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DataProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &DataProtectionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DataProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DataProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: DataProtectionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DataProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &DataProtectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DataProtectionInfo {}
unsafe impl ::core::marker::Sync for DataProtectionInfo {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
pub struct DataProtectionManager {}
impl DataProtectionManager {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage_Streams`*"]
    pub fn ProtectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(data: Param0, identity: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), identity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage_Streams`*"]
    pub fn UnprotectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(data: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage_Streams`*"]
    pub fn ProtectStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(unprotectedstream: Param0, identity: Param1, protectedstream: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), unprotectedstream.into_param().abi(), identity.into_param().abi(), protectedstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage_Streams`*"]
    pub fn UnprotectStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(protectedstream: Param0, unprotectedstream: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), protectedstream.into_param().abi(), unprotectedstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage_Streams`*"]
    pub fn GetProtectionInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(protecteddata: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), protecteddata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage_Streams`*"]
    pub fn GetStreamProtectionInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(protectedstream: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), protectedstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    pub fn IDataProtectionManagerStatics<R, F: FnOnce(&IDataProtectionManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataProtectionManager, IDataProtectionManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for DataProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionManager";
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataProtectionStatus(pub i32);
impl DataProtectionStatus {
    pub const ProtectedToOtherIdentity: DataProtectionStatus = DataProtectionStatus(0i32);
    pub const Protected: DataProtectionStatus = DataProtectionStatus(1i32);
    pub const Revoked: DataProtectionStatus = DataProtectionStatus(2i32);
    pub const Unprotected: DataProtectionStatus = DataProtectionStatus(3i32);
    pub const LicenseExpired: DataProtectionStatus = DataProtectionStatus(4i32);
    pub const AccessSuspended: DataProtectionStatus = DataProtectionStatus(5i32);
}
impl ::core::convert::From<i32> for DataProtectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DataProtectionStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DataProtectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.DataProtectionStatus;i4)");
}
impl ::windows::runtime::DefaultType for DataProtectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: EnforcementLevel = EnforcementLevel(0i32);
    pub const Silent: EnforcementLevel = EnforcementLevel(1i32);
    pub const Override: EnforcementLevel = EnforcementLevel(2i32);
    pub const Block: EnforcementLevel = EnforcementLevel(3i32);
}
impl ::core::convert::From<i32> for EnforcementLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EnforcementLevel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EnforcementLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.EnforcementLevel;i4)");
}
impl ::windows::runtime::DefaultType for EnforcementLevel {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct EnterpriseDataContract(pub u8);
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileProtectionInfo(pub ::windows::runtime::IInspectable);
impl FileProtectionInfo {
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<FileProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__: FileProtectionStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProtectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn IsRoamable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Identity(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn IsProtectWhileOpenSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IFileProtectionInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileProtectionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileProtectionInfo;{4ee96486-147e-4dd0-8faf-5253ed91ad0c})");
}
unsafe impl ::windows::runtime::Interface for FileProtectionInfo {
    type Vtable = IFileProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ee96486_147e_4dd0_8faf_5253ed91ad0c);
}
impl ::windows::runtime::RuntimeName for FileProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionInfo";
}
impl ::core::convert::From<FileProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: FileProtectionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &FileProtectionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: FileProtectionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &FileProtectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FileProtectionInfo {}
unsafe impl ::core::marker::Sync for FileProtectionInfo {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
pub struct FileProtectionManager {}
impl FileProtectionManager {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn ProtectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(target: Param0, identity: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), target.into_param().abi(), identity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn CopyProtectionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(source: Param0, target: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), source.into_param().abi(), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn GetProtectionInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(source: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn SaveFileAsContainerAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(protectedfile: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), protectedfile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn LoadFileFromContainerAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(containerfile: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), containerfile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn LoadFileFromContainerWithTargetAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(containerfile: Param0, target: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), containerfile.into_param().abi(), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn CreateProtectedAndOpenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(parentfolder: Param0, desiredname: Param1, identity: Param2, collisionoption: super::super::Storage::CreationCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), parentfolder.into_param().abi(), desiredname.into_param().abi(), identity.into_param().abi(), collisionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn IsContainerAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(containerfile: Param0, target: Param1, collisionoption: super::super::Storage::NameCollisionOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), containerfile.into_param().abi(), target.into_param().abi(), collisionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Foundation_Collections`, `Storage`*"]
    pub fn SaveFileAsContainerWithSharingAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(protectedfile: Param0, sharedwithidentities: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), protectedfile.into_param().abi(), sharedwithidentities.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn UnprotectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(target: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn UnprotectWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::runtime::IntoParam<'a, FileUnprotectOptions>>(target: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), target.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    pub fn IFileProtectionManagerStatics<R, F: FnOnce(&IFileProtectionManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFileProtectionManagerStatics2<R, F: FnOnce(&IFileProtectionManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFileProtectionManagerStatics3<R, F: FnOnce(&IFileProtectionManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for FileProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionManager";
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileProtectionStatus(pub i32);
impl FileProtectionStatus {
    pub const Undetermined: FileProtectionStatus = FileProtectionStatus(0i32);
    pub const Unknown: FileProtectionStatus = FileProtectionStatus(0i32);
    pub const Unprotected: FileProtectionStatus = FileProtectionStatus(1i32);
    pub const Revoked: FileProtectionStatus = FileProtectionStatus(2i32);
    pub const Protected: FileProtectionStatus = FileProtectionStatus(3i32);
    pub const ProtectedByOtherUser: FileProtectionStatus = FileProtectionStatus(4i32);
    pub const ProtectedToOtherEnterprise: FileProtectionStatus = FileProtectionStatus(5i32);
    pub const NotProtectable: FileProtectionStatus = FileProtectionStatus(6i32);
    pub const ProtectedToOtherIdentity: FileProtectionStatus = FileProtectionStatus(7i32);
    pub const LicenseExpired: FileProtectionStatus = FileProtectionStatus(8i32);
    pub const AccessSuspended: FileProtectionStatus = FileProtectionStatus(9i32);
    pub const FileInUse: FileProtectionStatus = FileProtectionStatus(10i32);
}
impl ::core::convert::From<i32> for FileProtectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FileProtectionStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FileProtectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.FileProtectionStatus;i4)");
}
impl ::windows::runtime::DefaultType for FileProtectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
pub struct FileRevocationManager {}
impl FileRevocationManager {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn ProtectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(storageitem: Param0, enterpriseidentity: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), storageitem.into_param().abi(), enterpriseidentity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn CopyProtectionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(sourcestorageitem: Param0, targetstorageitem: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sourcestorageitem.into_param().abi(), targetstorageitem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Revoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(enterpriseidentity: Param0) -> ::windows::runtime::Result<()> {
        Self::IFileRevocationManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), enterpriseidentity.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn GetStatusAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>>(storageitem: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), storageitem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    pub fn IFileRevocationManagerStatics<R, F: FnOnce(&IFileRevocationManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileRevocationManager, IFileRevocationManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for FileRevocationManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileRevocationManager";
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileUnprotectOptions(pub ::windows::runtime::IInspectable);
impl FileUnprotectOptions {
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SetAudit(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Audit(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Create(audit: bool) -> ::windows::runtime::Result<FileUnprotectOptions> {
        Self::IFileUnprotectOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audit, &mut result__).from_abi::<FileUnprotectOptions>(result__)
        })
    }
    pub fn IFileUnprotectOptionsFactory<R, F: FnOnce(&IFileUnprotectOptionsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileUnprotectOptions, IFileUnprotectOptionsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileUnprotectOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileUnprotectOptions;{7d1312f1-3b0d-4dd8-a1f8-1ec53822e2f3})");
}
unsafe impl ::windows::runtime::Interface for FileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7d1312f1_3b0d_4dd8_a1f8_1ec53822e2f3);
}
impl ::windows::runtime::RuntimeName for FileUnprotectOptions {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileUnprotectOptions";
}
impl ::core::convert::From<FileUnprotectOptions> for ::windows::runtime::IUnknown {
    fn from(value: FileUnprotectOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileUnprotectOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FileUnprotectOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileUnprotectOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileUnprotectOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileUnprotectOptions> for ::windows::runtime::IInspectable {
    fn from(value: FileUnprotectOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileUnprotectOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FileUnprotectOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileUnprotectOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileUnprotectOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FileUnprotectOptions {}
unsafe impl ::core::marker::Sync for FileUnprotectOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBufferProtectUnprotectResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47995edc_6cec_4e3a_b251_9e7485d79e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferProtectUnprotectResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataProtectionInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProtectionInfo {
    type Vtable = IDataProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8420b0c1_5e31_4405_9540_3f943af0cb26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DataProtectionStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataProtectionManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProtectionManagerStatics {
    type Vtable = IDataProtectionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb6149b74_9144_4ee4_8a8a_30b5f361430e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unprotectedstream: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, protectedstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protectedstream: ::windows::runtime::RawPtr, unprotectedstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protecteddata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protectedstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileProtectionInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionInfo {
    type Vtable = IFileProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ee96486_147e_4dd0_8faf_5253ed91ad0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FileProtectionStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileProtectionInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionInfo2 {
    type Vtable = IFileProtectionInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x82123a4c_557a_498d_8e94_944cd5836432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionManagerStatics {
    type Vtable = IFileProtectionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5846fc9b_e613_426b_bb38_88cba1dc9adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protectedfile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, containerfile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, containerfile: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentfolder: ::windows::runtime::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, collisionoption: super::super::Storage::CreationCollisionOption, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionManagerStatics2 {
    type Vtable = IFileProtectionManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x83d2a745_0483_41ab_b2d5_bc7f23d74ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, containerfile: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr, collisionoption: super::super::Storage::NameCollisionOption, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protectedfile: ::windows::runtime::RawPtr, sharedwithidentities: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionManagerStatics3 {
    type Vtable = IFileProtectionManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6918849a_624f_46d6_b241_e9cd5fdf3e3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileRevocationManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileRevocationManagerStatics {
    type Vtable = IFileRevocationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x256bbc3d_1c5d_4260_8c75_9144cfb78ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRevocationManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storageitem: ::windows::runtime::RawPtr, enterpriseidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestorageitem: ::windows::runtime::RawPtr, targetstorageitem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enterpriseidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storageitem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileUnprotectOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7d1312f1_3b0d_4dd8_a1f8_1ec53822e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileUnprotectOptionsFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileUnprotectOptionsFactory {
    type Vtable = IFileUnprotectOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51aeb39c_da8c_4c3f_9bfb_cb73a7cce0dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audit: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectedAccessResumedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xac4dca59_5d80_4e95_8c5f_8539450eebe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessResumedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectedAccessSuspendingEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x75a193e0_a344_429f_b975_04fc1f88c185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessSuspendingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectedContainerExportResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3948ef95_f7fb_4b42_afb0_df70b41543c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerExportResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ProtectedImportExportStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectedContainerImportResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcdb780d1_e7bb_4d1a_9339_34dc41149f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerImportResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ProtectedImportExportStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectedContentRevokedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x63686821_58b9_47ee_93d9_f0f741cf43f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContentRevokedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectedFileCreateResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x28e3ed6a_e9e7_4a03_9f53_bdb16172699b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedFileCreateResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x425ab7e4_feb7_44fc_b3bb_c3c4d7ecbebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ProtectionPolicyAuditAction) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ProtectionPolicyAuditAction) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfoFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyAuditInfoFactory {
    type Vtable = IProtectionPolicyAuditInfoFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7ed4180b_92e8_42d5_83d4_25440b423549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, sourcedescription: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, targetdescription: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd5703e18_a08d_47e6_a240_9934d7165eb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyManager2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManager2 {
    type Vtable = IProtectionPolicyManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xabf7527a_8435_417f_99b6_51beaf365888);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics {
    type Vtable = IProtectionPolicyManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc0bffc66_8c3d_4d56_8804_c68f0ad32ec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, endpointhost: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics2 {
    type Vtable = IProtectionPolicyManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb68f9a8c_39e0_4649_b2e4_070ab8a579b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, since: super::super::Foundation::DateTime, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut EnforcementLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics3 {
    type Vtable = IProtectionPolicyManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x48ff9e8c_6a6f_4d9f_bced_18ab537aa015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics4 {
    type Vtable = IProtectionPolicyManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20b794db_ccbd_490f_8c83_49ccb77aea6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceidentity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceitemlist: ::windows::runtime::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceitemlist: ::windows::runtime::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, auditinfo: ::windows::runtime::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceitemlist: ::windows::runtime::RawPtr, processid: u32, auditinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceitemlist: ::windows::runtime::RawPtr, processid: u32, auditinfo: ::windows::runtime::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentfolder: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, desiredname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IThreadNetworkContext(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IThreadNetworkContext {
    type Vtable = IThreadNetworkContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfa4ea8e9_ef13_405a_b12c_d7348c6f41fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadNetworkContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectedAccessResumedEventArgs(pub ::windows::runtime::IInspectable);
impl ProtectedAccessResumedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation_Collections`*"]
    pub fn Identities(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedAccessResumedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs;{ac4dca59-5d80-4e95-8c5f-8539450eebe0})");
}
unsafe impl ::windows::runtime::Interface for ProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xac4dca59_5d80_4e95_8c5f_8539450eebe0);
}
impl ::windows::runtime::RuntimeName for ProtectedAccessResumedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs";
}
impl ::core::convert::From<ProtectedAccessResumedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectedAccessResumedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectedAccessResumedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectedAccessResumedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectedAccessResumedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessResumedEventArgs {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectedAccessSuspendingEventArgs(pub ::windows::runtime::IInspectable);
impl ProtectedAccessSuspendingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation_Collections`*"]
    pub fn Identities(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedAccessSuspendingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs;{75a193e0-a344-429f-b975-04fc1f88c185})");
}
unsafe impl ::windows::runtime::Interface for ProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x75a193e0_a344_429f_b975_04fc1f88c185);
}
impl ::windows::runtime::RuntimeName for ProtectedAccessSuspendingEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs";
}
impl ::core::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectedAccessSuspendingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectedAccessSuspendingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectedAccessSuspendingEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessSuspendingEventArgs {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectedContainerExportResult(pub ::windows::runtime::IInspectable);
impl ProtectedContainerExportResult {
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__: ProtectedImportExportStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedContainerExportResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerExportResult;{3948ef95-f7fb-4b42-afb0-df70b41543c1})");
}
unsafe impl ::windows::runtime::Interface for ProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3948ef95_f7fb_4b42_afb0_df70b41543c1);
}
impl ::windows::runtime::RuntimeName for ProtectedContainerExportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerExportResult";
}
impl ::core::convert::From<ProtectedContainerExportResult> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedContainerExportResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectedContainerExportResult> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectedContainerExportResult> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedContainerExportResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectedContainerExportResult> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectedContainerExportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerExportResult {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectedContainerImportResult(pub ::windows::runtime::IInspectable);
impl ProtectedContainerImportResult {
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__: ProtectedImportExportStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedContainerImportResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerImportResult;{cdb780d1-e7bb-4d1a-9339-34dc41149f9b})");
}
unsafe impl ::windows::runtime::Interface for ProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcdb780d1_e7bb_4d1a_9339_34dc41149f9b);
}
impl ::windows::runtime::RuntimeName for ProtectedContainerImportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerImportResult";
}
impl ::core::convert::From<ProtectedContainerImportResult> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedContainerImportResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectedContainerImportResult> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectedContainerImportResult> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedContainerImportResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectedContainerImportResult> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectedContainerImportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerImportResult {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectedContentRevokedEventArgs(pub ::windows::runtime::IInspectable);
impl ProtectedContentRevokedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation_Collections`*"]
    pub fn Identities(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedContentRevokedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs;{63686821-58b9-47ee-93d9-f0f741cf43f0})");
}
unsafe impl ::windows::runtime::Interface for ProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x63686821_58b9_47ee_93d9_f0f741cf43f0);
}
impl ::windows::runtime::RuntimeName for ProtectedContentRevokedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs";
}
impl ::core::convert::From<ProtectedContentRevokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectedContentRevokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectedContentRevokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectedContentRevokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectedContentRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedContentRevokedEventArgs {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectedFileCreateResult(pub ::windows::runtime::IInspectable);
impl ProtectedFileCreateResult {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Storage_Streams`*"]
    pub fn Stream(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn ProtectionInfo(&self) -> ::windows::runtime::Result<FileProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProtectionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedFileCreateResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedFileCreateResult;{28e3ed6a-e9e7-4a03-9f53-bdb16172699b})");
}
unsafe impl ::windows::runtime::Interface for ProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x28e3ed6a_e9e7_4a03_9f53_bdb16172699b);
}
impl ::windows::runtime::RuntimeName for ProtectedFileCreateResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedFileCreateResult";
}
impl ::core::convert::From<ProtectedFileCreateResult> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedFileCreateResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectedFileCreateResult> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectedFileCreateResult> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedFileCreateResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectedFileCreateResult> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectedFileCreateResult {}
unsafe impl ::core::marker::Sync for ProtectedFileCreateResult {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProtectedImportExportStatus(pub i32);
impl ProtectedImportExportStatus {
    pub const Ok: ProtectedImportExportStatus = ProtectedImportExportStatus(0i32);
    pub const Undetermined: ProtectedImportExportStatus = ProtectedImportExportStatus(1i32);
    pub const Unprotected: ProtectedImportExportStatus = ProtectedImportExportStatus(2i32);
    pub const Revoked: ProtectedImportExportStatus = ProtectedImportExportStatus(3i32);
    pub const NotRoamable: ProtectedImportExportStatus = ProtectedImportExportStatus(4i32);
    pub const ProtectedToOtherIdentity: ProtectedImportExportStatus = ProtectedImportExportStatus(5i32);
    pub const LicenseExpired: ProtectedImportExportStatus = ProtectedImportExportStatus(6i32);
    pub const AccessSuspended: ProtectedImportExportStatus = ProtectedImportExportStatus(7i32);
}
impl ::core::convert::From<i32> for ProtectedImportExportStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectedImportExportStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedImportExportStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectedImportExportStatus;i4)");
}
impl ::windows::runtime::DefaultType for ProtectedImportExportStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(0i32);
    pub const CopyToLocation: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(1i32);
    pub const SendToRecipient: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(2i32);
    pub const Other: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(3i32);
}
impl ::core::convert::From<i32> for ProtectionPolicyAuditAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectionPolicyAuditAction {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyAuditAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyAuditAction;i4)");
}
impl ::windows::runtime::DefaultType for ProtectionPolicyAuditAction {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectionPolicyAuditInfo(pub ::windows::runtime::IInspectable);
impl ProtectionPolicyAuditInfo {
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SetAction(&self, value: ProtectionPolicyAuditAction) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Action(&self) -> ::windows::runtime::Result<ProtectionPolicyAuditAction> {
        let this = self;
        unsafe {
            let mut result__: ProtectionPolicyAuditAction = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectionPolicyAuditAction>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SetDataDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn DataDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SetSourceDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SourceDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SetTargetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn TargetDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(action: ProtectionPolicyAuditAction, datadescription: Param1, sourcedescription: Param2, targetdescription: Param3) -> ::windows::runtime::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), action, datadescription.into_param().abi(), sourcedescription.into_param().abi(), targetdescription.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn CreateWithActionAndDataDescription<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(action: ProtectionPolicyAuditAction, datadescription: Param1) -> ::windows::runtime::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), action, datadescription.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    pub fn IProtectionPolicyAuditInfoFactory<R, F: FnOnce(&IProtectionPolicyAuditInfoFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProtectionPolicyAuditInfo, IProtectionPolicyAuditInfoFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyAuditInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo;{425ab7e4-feb7-44fc-b3bb-c3c4d7ecbebb})");
}
unsafe impl ::windows::runtime::Interface for ProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x425ab7e4_feb7_44fc_b3bb_c3c4d7ecbebb);
}
impl ::windows::runtime::RuntimeName for ProtectionPolicyAuditInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo";
}
impl ::core::convert::From<ProtectionPolicyAuditInfo> for ::windows::runtime::IUnknown {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectionPolicyAuditInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectionPolicyAuditInfo> for ::windows::runtime::IInspectable {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectionPolicyAuditInfo> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectionPolicyAuditInfo {}
unsafe impl ::core::marker::Sync for ProtectionPolicyAuditInfo {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(0i32);
    pub const Blocked: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(1i32);
    pub const ConsentRequired: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(2i32);
}
impl ::core::convert::From<i32> for ProtectionPolicyEvaluationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectionPolicyEvaluationResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyEvaluationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult;i4)");
}
impl ::windows::runtime::DefaultType for ProtectionPolicyEvaluationResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtectionPolicyManager(pub ::windows::runtime::IInspectable);
impl ProtectionPolicyManager {
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SetIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn Identity(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn IsIdentityManaged<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn TryApplyProcessUIPolicy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn ClearProcessUIPolicy() -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn CreateCurrentThreadNetworkContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<ThreadNetworkContext> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<ThreadNetworkContext>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Networking`*"]
    pub fn GetPrimaryManagedIdentityForNetworkEndpointAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Networking::HostName>>(endpointhost: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), endpointhost.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn RevokeContent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), identity.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<ProtectionPolicyManager> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectionPolicyManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn ProtectedAccessSuspending<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RemoveProtectedAccessSuspending<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn ProtectedAccessResumed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RemoveProtectedAccessResumed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn ProtectedContentRevoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RemoveProtectedContentRevoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn CheckAccess<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(sourceidentity: Param0, targetidentity: Param1) -> ::windows::runtime::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ProtectionPolicyEvaluationResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(sourceidentity: Param0, targetidentity: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn HasContentBeenRevokedSince<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(identity: Param0, since: Param1) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), identity.into_param().abi(), since.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn CheckAccessForApp<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1) -> ::windows::runtime::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: ProtectionPolicyEvaluationResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessForAppAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn GetEnforcementLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<EnforcementLevel> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: EnforcementLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<EnforcementLevel>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn IsUserDecryptionAllowed<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn IsProtectionUnderLockRequired<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn PolicyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RemovePolicyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn IsProtectionEnabled() -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn SetShowEnterpriseIndicator(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn ShowEnterpriseIndicator(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessWithAuditingInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessWithMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        sourceidentity: Param0,
        targetidentity: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessForAppWithAuditingInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, apppackagefamilyname: Param1, auditinfo: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessForAppWithMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        sourceidentity: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn LogAuditEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn IsRoamableProtectionEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessWithBehaviorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        sourceidentity: Param0,
        targetidentity: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn RequestAccessForAppWithBehaviorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        sourceidentity: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Foundation_Collections`, `Storage`*"]
    pub fn RequestAccessToFilesForAppAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceitemlist: Param0, apppackagefamilyname: Param1, auditinfo: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Foundation_Collections`, `Storage`*"]
    pub fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        sourceitemlist: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Foundation_Collections`, `Storage`*"]
    pub fn RequestAccessToFilesForProcessAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceitemlist: Param0, processid: u32, auditinfo: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), processid, auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Foundation_Collections`, `Storage`*"]
    pub fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        sourceitemlist: Param0,
        processid: u32,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), processid, auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn IsFileProtectionRequiredAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(target: Param0, identity: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), target.into_param().abi(), identity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`, `Storage`*"]
    pub fn IsFileProtectionRequiredForNewFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(parentfolder: Param0, identity: Param1, desiredname: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), parentfolder.into_param().abi(), identity.into_param().abi(), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn PrimaryManagedIdentity() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Security_EnterpriseData`*"]
    pub fn GetPrimaryManagedIdentityForIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identity: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IProtectionPolicyManagerStatics<R, F: FnOnce(&IProtectionPolicyManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics2<R, F: FnOnce(&IProtectionPolicyManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics3<R, F: FnOnce(&IProtectionPolicyManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics4<R, F: FnOnce(&IProtectionPolicyManagerStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyManager;{d5703e18-a08d-47e6-a240-9934d7165eb5})");
}
unsafe impl ::windows::runtime::Interface for ProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd5703e18_a08d_47e6_a240_9934d7165eb5);
}
impl ::windows::runtime::RuntimeName for ProtectionPolicyManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyManager";
}
impl ::core::convert::From<ProtectionPolicyManager> for ::windows::runtime::IUnknown {
    fn from(value: ProtectionPolicyManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtectionPolicyManager> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectionPolicyManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectionPolicyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtectionPolicyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtectionPolicyManager> for ::windows::runtime::IInspectable {
    fn from(value: ProtectionPolicyManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtectionPolicyManager> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectionPolicyManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectionPolicyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectionPolicyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProtectionPolicyManager {}
unsafe impl ::core::marker::Sync for ProtectionPolicyManager {}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: ProtectionPolicyRequestAccessBehavior = ProtectionPolicyRequestAccessBehavior(0i32);
    pub const TreatOverridePolicyAsBlock: ProtectionPolicyRequestAccessBehavior = ProtectionPolicyRequestAccessBehavior(1i32);
}
impl ::core::convert::From<i32> for ProtectionPolicyRequestAccessBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectionPolicyRequestAccessBehavior {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyRequestAccessBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyRequestAccessBehavior;i4)");
}
impl ::windows::runtime::DefaultType for ProtectionPolicyRequestAccessBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_EnterpriseData`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ThreadNetworkContext(pub ::windows::runtime::IInspectable);
impl ThreadNetworkContext {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_EnterpriseData`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ThreadNetworkContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ThreadNetworkContext;{fa4ea8e9-ef13-405a-b12c-d7348c6f41fc})");
}
unsafe impl ::windows::runtime::Interface for ThreadNetworkContext {
    type Vtable = IThreadNetworkContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfa4ea8e9_ef13_405a_b12c_d7348c6f41fc);
}
impl ::windows::runtime::RuntimeName for ThreadNetworkContext {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ThreadNetworkContext";
}
impl ::core::convert::From<ThreadNetworkContext> for ::windows::runtime::IUnknown {
    fn from(value: ThreadNetworkContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ThreadNetworkContext> for ::windows::runtime::IUnknown {
    fn from(value: &ThreadNetworkContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ThreadNetworkContext> for ::windows::runtime::IInspectable {
    fn from(value: ThreadNetworkContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ThreadNetworkContext> for ::windows::runtime::IInspectable {
    fn from(value: &ThreadNetworkContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ThreadNetworkContext> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ThreadNetworkContext) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ThreadNetworkContext> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ThreadNetworkContext) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ThreadNetworkContext {}
unsafe impl ::core::marker::Sync for ThreadNetworkContext {}
