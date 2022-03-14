#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct BufferProtectUnprotectResult(::windows::core::IUnknown);
impl BufferProtectUnprotectResult {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Buffer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn ProtectionInfo(&self) -> ::windows::core::Result<DataProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataProtectionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for BufferProtectUnprotectResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BufferProtectUnprotectResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BufferProtectUnprotectResult {}
impl ::core::fmt::Debug for BufferProtectUnprotectResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BufferProtectUnprotectResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BufferProtectUnprotectResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.BufferProtectUnprotectResult;{47995edc-6cec-4e3a-b251-9e7485d79e7a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_Vtbl;
    const IID: ::windows::core::GUID = <IBufferProtectUnprotectResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BufferProtectUnprotectResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.BufferProtectUnprotectResult";
}
impl ::core::convert::From<BufferProtectUnprotectResult> for ::windows::core::IUnknown {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferProtectUnprotectResult> for ::windows::core::IUnknown {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BufferProtectUnprotectResult> for ::windows::core::IInspectable {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferProtectUnprotectResult> for ::windows::core::IInspectable {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BufferProtectUnprotectResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BufferProtectUnprotectResult {}
unsafe impl ::core::marker::Sync for BufferProtectUnprotectResult {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct DataProtectionInfo(::windows::core::IUnknown);
impl DataProtectionInfo {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<DataProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__: DataProtectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataProtectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Identity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DataProtectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataProtectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataProtectionInfo {}
impl ::core::fmt::Debug for DataProtectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProtectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DataProtectionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.DataProtectionInfo;{8420b0c1-5e31-4405-9540-3f943af0cb26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DataProtectionInfo {
    type Vtable = IDataProtectionInfo_Vtbl;
    const IID: ::windows::core::GUID = <IDataProtectionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DataProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionInfo";
}
impl ::core::convert::From<DataProtectionInfo> for ::windows::core::IUnknown {
    fn from(value: DataProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataProtectionInfo> for ::windows::core::IUnknown {
    fn from(value: &DataProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DataProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataProtectionInfo> for ::windows::core::IInspectable {
    fn from(value: DataProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataProtectionInfo> for ::windows::core::IInspectable {
    fn from(value: &DataProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DataProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DataProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DataProtectionInfo {}
unsafe impl ::core::marker::Sync for DataProtectionInfo {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
pub struct DataProtectionManager {}
impl DataProtectionManager {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(data: Param0, identity: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectAsync)(::core::mem::transmute_copy(this), data.into_param().abi(), identity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(data: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectAsync)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(unprotectedstream: Param0, identity: Param1, protectedstream: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectStreamAsync)(::core::mem::transmute_copy(this), unprotectedstream.into_param().abi(), identity.into_param().abi(), protectedstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(protectedstream: Param0, unprotectedstream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectStreamAsync)(::core::mem::transmute_copy(this), protectedstream.into_param().abi(), unprotectedstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetProtectionInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(protecteddata: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProtectionInfoAsync)(::core::mem::transmute_copy(this), protecteddata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetStreamProtectionInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(protectedstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>> {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStreamProtectionInfoAsync)(::core::mem::transmute_copy(this), protectedstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataProtectionManagerStatics<R, F: FnOnce(&IDataProtectionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataProtectionManager, IDataProtectionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DataProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionManager";
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataProtectionStatus(pub i32);
impl DataProtectionStatus {
    pub const ProtectedToOtherIdentity: Self = Self(0i32);
    pub const Protected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Unprotected: Self = Self(3i32);
    pub const LicenseExpired: Self = Self(4i32);
    pub const AccessSuspended: Self = Self(5i32);
}
impl ::core::marker::Copy for DataProtectionStatus {}
impl ::core::clone::Clone for DataProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DataProtectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProtectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DataProtectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.DataProtectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: Self = Self(0i32);
    pub const Silent: Self = Self(1i32);
    pub const Override: Self = Self(2i32);
    pub const Block: Self = Self(3i32);
}
impl ::core::marker::Copy for EnforcementLevel {}
impl ::core::clone::Clone for EnforcementLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnforcementLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EnforcementLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnforcementLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnforcementLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnforcementLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.EnforcementLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct FileProtectionInfo(::windows::core::IUnknown);
impl FileProtectionInfo {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<FileProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__: FileProtectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProtectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn IsRoamable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRoamable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Identity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn IsProtectWhileOpenSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFileProtectionInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsProtectWhileOpenSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for FileProtectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileProtectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileProtectionInfo {}
impl ::core::fmt::Debug for FileProtectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileProtectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileProtectionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileProtectionInfo;{4ee96486-147e-4dd0-8faf-5253ed91ad0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileProtectionInfo {
    type Vtable = IFileProtectionInfo_Vtbl;
    const IID: ::windows::core::GUID = <IFileProtectionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionInfo";
}
impl ::core::convert::From<FileProtectionInfo> for ::windows::core::IUnknown {
    fn from(value: FileProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileProtectionInfo> for ::windows::core::IUnknown {
    fn from(value: &FileProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileProtectionInfo> for ::windows::core::IInspectable {
    fn from(value: FileProtectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileProtectionInfo> for ::windows::core::IInspectable {
    fn from(value: &FileProtectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileProtectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FileProtectionInfo {}
unsafe impl ::core::marker::Sync for FileProtectionInfo {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
pub struct FileProtectionManager {}
impl FileProtectionManager {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(target: Param0, identity: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectAsync)(::core::mem::transmute_copy(this), target.into_param().abi(), identity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CopyProtectionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(source: Param0, target: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyProtectionAsync)(::core::mem::transmute_copy(this), source.into_param().abi(), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetProtectionInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(source: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProtectionInfoAsync)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveFileAsContainerAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(protectedfile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveFileAsContainerAsync)(::core::mem::transmute_copy(this), protectedfile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(containerfile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFileFromContainerAsync)(::core::mem::transmute_copy(this), containerfile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerWithTargetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(containerfile: Param0, target: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFileFromContainerWithTargetAsync)(::core::mem::transmute_copy(this), containerfile.into_param().abi(), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateProtectedAndOpenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(parentfolder: Param0, desiredname: Param1, identity: Param2, collisionoption: super::super::Storage::CreationCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateProtectedAndOpenAsync)(::core::mem::transmute_copy(this), parentfolder.into_param().abi(), desiredname.into_param().abi(), identity.into_param().abi(), collisionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsContainerAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsContainerAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(containerfile: Param0, target: Param1, collisionoption: super::super::Storage::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFileFromContainerWithTargetAndNameCollisionOptionAsync)(::core::mem::transmute_copy(this), containerfile.into_param().abi(), target.into_param().abi(), collisionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SaveFileAsContainerWithSharingAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(protectedfile: Param0, sharedwithidentities: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveFileAsContainerWithSharingAsync)(::core::mem::transmute_copy(this), protectedfile.into_param().abi(), sharedwithidentities.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn UnprotectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(target: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectAsync)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn UnprotectWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::core::IntoParam<'a, FileUnprotectOptions>>(target: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>> {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectWithOptionsAsync)(::core::mem::transmute_copy(this), target.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileProtectionManagerStatics<R, F: FnOnce(&IFileProtectionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFileProtectionManagerStatics2<R, F: FnOnce(&IFileProtectionManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFileProtectionManagerStatics3<R, F: FnOnce(&IFileProtectionManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for FileProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionManager";
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FileProtectionStatus(pub i32);
impl FileProtectionStatus {
    pub const Undetermined: Self = Self(0i32);
    pub const Unknown: Self = Self(0i32);
    pub const Unprotected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Protected: Self = Self(3i32);
    pub const ProtectedByOtherUser: Self = Self(4i32);
    pub const ProtectedToOtherEnterprise: Self = Self(5i32);
    pub const NotProtectable: Self = Self(6i32);
    pub const ProtectedToOtherIdentity: Self = Self(7i32);
    pub const LicenseExpired: Self = Self(8i32);
    pub const AccessSuspended: Self = Self(9i32);
    pub const FileInUse: Self = Self(10i32);
}
impl ::core::marker::Copy for FileProtectionStatus {}
impl ::core::clone::Clone for FileProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileProtectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileProtectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileProtectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.FileProtectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct FileRevocationManager {}
#[cfg(feature = "deprecated")]
impl FileRevocationManager {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn ProtectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(storageitem: Param0, enterpriseidentity: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectAsync)(::core::mem::transmute_copy(this), storageitem.into_param().abi(), enterpriseidentity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn CopyProtectionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(sourcestorageitem: Param0, targetstorageitem: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyProtectionAsync)(::core::mem::transmute_copy(this), sourcestorageitem.into_param().abi(), targetstorageitem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Revoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(enterpriseidentity: Param0) -> ::windows::core::Result<()> {
        Self::IFileRevocationManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Revoke)(::core::mem::transmute_copy(this), enterpriseidentity.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn GetStatusAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>>(storageitem: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatusAsync)(::core::mem::transmute_copy(this), storageitem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IFileRevocationManagerStatics<R, F: FnOnce(&IFileRevocationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileRevocationManager, IFileRevocationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FileRevocationManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileRevocationManager";
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct FileUnprotectOptions(::windows::core::IUnknown);
impl FileUnprotectOptions {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SetAudit(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Audit(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Audit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Create(audit: bool) -> ::windows::core::Result<FileUnprotectOptions> {
        Self::IFileUnprotectOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), audit, &mut result__).from_abi::<FileUnprotectOptions>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileUnprotectOptionsFactory<R, F: FnOnce(&IFileUnprotectOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileUnprotectOptions, IFileUnprotectOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FileUnprotectOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUnprotectOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUnprotectOptions {}
impl ::core::fmt::Debug for FileUnprotectOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUnprotectOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUnprotectOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileUnprotectOptions;{7d1312f1-3b0d-4dd8-a1f8-1ec53822e2f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_Vtbl;
    const IID: ::windows::core::GUID = <IFileUnprotectOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileUnprotectOptions {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileUnprotectOptions";
}
impl ::core::convert::From<FileUnprotectOptions> for ::windows::core::IUnknown {
    fn from(value: FileUnprotectOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUnprotectOptions> for ::windows::core::IUnknown {
    fn from(value: &FileUnprotectOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileUnprotectOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileUnprotectOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileUnprotectOptions> for ::windows::core::IInspectable {
    fn from(value: FileUnprotectOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUnprotectOptions> for ::windows::core::IInspectable {
    fn from(value: &FileUnprotectOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileUnprotectOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileUnprotectOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FileUnprotectOptions {}
unsafe impl ::core::marker::Sync for FileUnprotectOptions {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferProtectUnprotectResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47995edc_6cec_4e3a_b251_9e7485d79e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferProtectUnprotectResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataProtectionInfo {
    type Vtable = IDataProtectionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8420b0c1_5e31_4405_9540_3f943af0cb26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataProtectionStatus) -> ::windows::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataProtectionManagerStatics {
    type Vtable = IDataProtectionManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6149b74_9144_4ee4_8a8a_30b5f361430e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unprotectedstream: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectedstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedstream: ::windows::core::RawPtr, unprotectedstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protecteddata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetStreamProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetStreamProtectionInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileProtectionInfo {
    type Vtable = IFileProtectionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ee96486_147e_4dd0_8faf_5253ed91ad0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileProtectionStatus) -> ::windows::core::HRESULT,
    pub IsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileProtectionInfo2 {
    type Vtable = IFileProtectionInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82123a4c_557a_498d_8e94_944cd5836432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsProtectWhileOpenSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileProtectionManagerStatics {
    type Vtable = IFileProtectionManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5846fc9b_e613_426b_bb38_88cba1dc9adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CopyProtectionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveFileAsContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveFileAsContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerWithTargetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerWithTargetAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateProtectedAndOpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolder: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, collisionoption: super::super::Storage::CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateProtectedAndOpenAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileProtectionManagerStatics2 {
    type Vtable = IFileProtectionManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d2a745_0483_41ab_b2d5_bc7f23d74ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: ::windows::core::RawPtr, target: ::windows::core::RawPtr, collisionoption: super::super::Storage::NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveFileAsContainerWithSharingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedfile: ::windows::core::RawPtr, sharedwithidentities: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveFileAsContainerWithSharingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileProtectionManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileProtectionManagerStatics3 {
    type Vtable = IFileProtectionManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6918849a_624f_46d6_b241_e9cd5fdf3e3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub UnprotectWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    UnprotectWithOptionsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileRevocationManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileRevocationManagerStatics {
    type Vtable = IFileRevocationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x256bbc3d_1c5d_4260_8c75_9144cfb78ba9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileRevocationManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, enterpriseidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestorageitem: ::windows::core::RawPtr, targetstorageitem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    CopyProtectionAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Revoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterpriseidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Revoke: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    GetStatusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUnprotectOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1312f1_3b0d_4dd8_a1f8_1ec53822e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Audit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUnprotectOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUnprotectOptionsFactory {
    type Vtable = IFileUnprotectOptionsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51aeb39c_da8c_4c3f_9bfb_cb73a7cce0dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptionsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audit: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedAccessResumedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac4dca59_5d80_4e95_8c5f_8539450eebe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessResumedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedAccessSuspendingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75a193e0_a344_429f_b975_04fc1f88c185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessSuspendingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedContainerExportResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3948ef95_f7fb_4b42_afb0_df70b41543c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerExportResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedContainerImportResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdb780d1_e7bb_4d1a_9339_34dc41149f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerImportResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedContentRevokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63686821_58b9_47ee_93d9_f0f741cf43f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContentRevokedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectedFileCreateResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28e3ed6a_e9e7_4a03_9f53_bdb16172699b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedFileCreateResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x425ab7e4_feb7_44fc_b3bb_c3c4d7ecbebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProtectionPolicyAuditAction) -> ::windows::core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectionPolicyAuditAction) -> ::windows::core::HRESULT,
    pub SetDataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSourceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTargetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TargetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyAuditInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyAuditInfoFactory {
    type Vtable = IProtectionPolicyAuditInfoFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed4180b_92e8_42d5_83d4_25440b423549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfoFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sourcedescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetdescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithActionAndDataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5703e18_a08d_47e6_a240_9934d7165eb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyManager2 {
    type Vtable = IProtectionPolicyManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabf7527a_8435_417f_99b6_51beaf365888);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerStatics {
    type Vtable = IProtectionPolicyManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0bffc66_8c3d_4d56_8804_c68f0ad32ec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsIdentityManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryApplyProcessUIPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ClearProcessUIPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCurrentThreadNetworkContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub GetPrimaryManagedIdentityForNetworkEndpointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointhost: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))]
    GetPrimaryManagedIdentityForNetworkEndpointAsync: usize,
    pub RevokeContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtectedAccessSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedAccessSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedAccessSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedAccessSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectedAccessResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedAccessResumed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedAccessResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedAccessResumed: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectedContentRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedContentRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedContentRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedContentRevoked: usize,
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerStatics2 {
    type Vtable = IProtectionPolicyManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb68f9a8c_39e0_4649_b2e4_070ab8a579b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub HasContentBeenRevokedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, since: super::super::Foundation::DateTime, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HasContentBeenRevokedSince: usize,
    pub CheckAccessForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppAsync: usize,
    pub GetEnforcementLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut EnforcementLevel) -> ::windows::core::HRESULT,
    pub IsUserDecryptionAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsProtectionUnderLockRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PolicyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePolicyChanged: usize,
    pub IsProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerStatics3 {
    type Vtable = IProtectionPolicyManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48ff9e8c_6a6f_4d9f_bced_18ab537aa015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithAuditingInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithAuditingInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithMessageAsync: usize,
    pub LogAuditEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionPolicyManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerStatics4 {
    type Vtable = IProtectionPolicyManagerStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20b794db_ccbd_490f_8c83_49ccb77aea6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsRoamableProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithBehaviorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForAppAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForAppWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForAppWithMessageAndBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForProcessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, processid: u32, auditinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForProcessAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: ::windows::core::RawPtr, processid: u32, auditinfo: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsFileProtectionRequiredAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsFileProtectionRequiredAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsFileProtectionRequiredForNewFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolder: ::windows::core::RawPtr, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsFileProtectionRequiredForNewFileAsync: usize,
    pub PrimaryManagedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetPrimaryManagedIdentityForIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThreadNetworkContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IThreadNetworkContext {
    type Vtable = IThreadNetworkContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa4ea8e9_ef13_405a_b12c_d7348c6f41fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadNetworkContext_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectedAccessResumedEventArgs(::windows::core::IUnknown);
impl ProtectedAccessResumedEventArgs {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Identities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedAccessResumedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedAccessResumedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedAccessResumedEventArgs {}
impl ::core::fmt::Debug for ProtectedAccessResumedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedAccessResumedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectedAccessResumedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs;{ac4dca59-5d80-4e95-8c5f-8539450eebe0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IProtectedAccessResumedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectedAccessResumedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs";
}
impl ::core::convert::From<ProtectedAccessResumedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessResumedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedAccessResumedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessResumedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectedAccessResumedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedAccessResumedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessResumedEventArgs {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectedAccessSuspendingEventArgs(::windows::core::IUnknown);
impl ProtectedAccessSuspendingEventArgs {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Identities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedAccessSuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedAccessSuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedAccessSuspendingEventArgs {}
impl ::core::fmt::Debug for ProtectedAccessSuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedAccessSuspendingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectedAccessSuspendingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs;{75a193e0-a344-429f-b975-04fc1f88c185})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IProtectedAccessSuspendingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectedAccessSuspendingEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs";
}
impl ::core::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessSuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedAccessSuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectedAccessSuspendingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedAccessSuspendingEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessSuspendingEventArgs {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectedContainerExportResult(::windows::core::IUnknown);
impl ProtectedContainerExportResult {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__: ProtectedImportExportStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedContainerExportResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedContainerExportResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedContainerExportResult {}
impl ::core::fmt::Debug for ProtectedContainerExportResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedContainerExportResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectedContainerExportResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerExportResult;{3948ef95-f7fb-4b42-afb0-df70b41543c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_Vtbl;
    const IID: ::windows::core::GUID = <IProtectedContainerExportResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectedContainerExportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerExportResult";
}
impl ::core::convert::From<ProtectedContainerExportResult> for ::windows::core::IUnknown {
    fn from(value: ProtectedContainerExportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerExportResult> for ::windows::core::IUnknown {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedContainerExportResult> for ::windows::core::IInspectable {
    fn from(value: ProtectedContainerExportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerExportResult> for ::windows::core::IInspectable {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectedContainerExportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedContainerExportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerExportResult {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectedContainerImportResult(::windows::core::IUnknown);
impl ProtectedContainerImportResult {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__: ProtectedImportExportStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedContainerImportResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedContainerImportResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedContainerImportResult {}
impl ::core::fmt::Debug for ProtectedContainerImportResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedContainerImportResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectedContainerImportResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerImportResult;{cdb780d1-e7bb-4d1a-9339-34dc41149f9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_Vtbl;
    const IID: ::windows::core::GUID = <IProtectedContainerImportResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectedContainerImportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerImportResult";
}
impl ::core::convert::From<ProtectedContainerImportResult> for ::windows::core::IUnknown {
    fn from(value: ProtectedContainerImportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerImportResult> for ::windows::core::IUnknown {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedContainerImportResult> for ::windows::core::IInspectable {
    fn from(value: ProtectedContainerImportResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContainerImportResult> for ::windows::core::IInspectable {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectedContainerImportResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedContainerImportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerImportResult {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectedContentRevokedEventArgs(::windows::core::IUnknown);
impl ProtectedContentRevokedEventArgs {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Identities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedContentRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedContentRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedContentRevokedEventArgs {}
impl ::core::fmt::Debug for ProtectedContentRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedContentRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectedContentRevokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs;{63686821-58b9-47ee-93d9-f0f741cf43f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IProtectedContentRevokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectedContentRevokedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs";
}
impl ::core::convert::From<ProtectedContentRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContentRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedContentRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedContentRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectedContentRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedContentRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedContentRevokedEventArgs {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectedFileCreateResult(::windows::core::IUnknown);
impl ProtectedFileCreateResult {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn ProtectionInfo(&self) -> ::windows::core::Result<FileProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProtectionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectedFileCreateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectedFileCreateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectedFileCreateResult {}
impl ::core::fmt::Debug for ProtectedFileCreateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedFileCreateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectedFileCreateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedFileCreateResult;{28e3ed6a-e9e7-4a03-9f53-bdb16172699b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_Vtbl;
    const IID: ::windows::core::GUID = <IProtectedFileCreateResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectedFileCreateResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedFileCreateResult";
}
impl ::core::convert::From<ProtectedFileCreateResult> for ::windows::core::IUnknown {
    fn from(value: ProtectedFileCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedFileCreateResult> for ::windows::core::IUnknown {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectedFileCreateResult> for ::windows::core::IInspectable {
    fn from(value: ProtectedFileCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectedFileCreateResult> for ::windows::core::IInspectable {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectedFileCreateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectedFileCreateResult {}
unsafe impl ::core::marker::Sync for ProtectedFileCreateResult {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectedImportExportStatus(pub i32);
impl ProtectedImportExportStatus {
    pub const Ok: Self = Self(0i32);
    pub const Undetermined: Self = Self(1i32);
    pub const Unprotected: Self = Self(2i32);
    pub const Revoked: Self = Self(3i32);
    pub const NotRoamable: Self = Self(4i32);
    pub const ProtectedToOtherIdentity: Self = Self(5i32);
    pub const LicenseExpired: Self = Self(6i32);
    pub const AccessSuspended: Self = Self(7i32);
}
impl ::core::marker::Copy for ProtectedImportExportStatus {}
impl ::core::clone::Clone for ProtectedImportExportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectedImportExportStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProtectedImportExportStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectedImportExportStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedImportExportStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectedImportExportStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectedImportExportStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: Self = Self(0i32);
    pub const CopyToLocation: Self = Self(1i32);
    pub const SendToRecipient: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for ProtectionPolicyAuditAction {}
impl ::core::clone::Clone for ProtectionPolicyAuditAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyAuditAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProtectionPolicyAuditAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionPolicyAuditAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyAuditAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectionPolicyAuditAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyAuditAction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectionPolicyAuditInfo(::windows::core::IUnknown);
impl ProtectionPolicyAuditInfo {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SetAction(&self, value: ProtectionPolicyAuditAction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAction)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Action(&self) -> ::windows::core::Result<ProtectionPolicyAuditAction> {
        let this = self;
        unsafe {
            let mut result__: ProtectionPolicyAuditAction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Action)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectionPolicyAuditAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SetDataDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDataDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn DataDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SetSourceDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSourceDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SourceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SetTargetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn TargetDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Create<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(action: ProtectionPolicyAuditAction, datadescription: Param1, sourcedescription: Param2, targetdescription: Param3) -> ::windows::core::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), action, datadescription.into_param().abi(), sourcedescription.into_param().abi(), targetdescription.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn CreateWithActionAndDataDescription<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(action: ProtectionPolicyAuditAction, datadescription: Param1) -> ::windows::core::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithActionAndDataDescription)(::core::mem::transmute_copy(this), action, datadescription.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyAuditInfoFactory<R, F: FnOnce(&IProtectionPolicyAuditInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProtectionPolicyAuditInfo, IProtectionPolicyAuditInfoFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProtectionPolicyAuditInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectionPolicyAuditInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectionPolicyAuditInfo {}
impl ::core::fmt::Debug for ProtectionPolicyAuditInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyAuditInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectionPolicyAuditInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo;{425ab7e4-feb7-44fc-b3bb-c3c4d7ecbebb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_Vtbl;
    const IID: ::windows::core::GUID = <IProtectionPolicyAuditInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectionPolicyAuditInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo";
}
impl ::core::convert::From<ProtectionPolicyAuditInfo> for ::windows::core::IUnknown {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyAuditInfo> for ::windows::core::IUnknown {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectionPolicyAuditInfo> for ::windows::core::IInspectable {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyAuditInfo> for ::windows::core::IInspectable {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectionPolicyAuditInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectionPolicyAuditInfo {}
unsafe impl ::core::marker::Sync for ProtectionPolicyAuditInfo {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: Self = Self(0i32);
    pub const Blocked: Self = Self(1i32);
    pub const ConsentRequired: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionPolicyEvaluationResult {}
impl ::core::clone::Clone for ProtectionPolicyEvaluationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyEvaluationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProtectionPolicyEvaluationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionPolicyEvaluationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyEvaluationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectionPolicyEvaluationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectionPolicyManager(::windows::core::IUnknown);
impl ProtectionPolicyManager {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SetIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIdentity)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn Identity(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Identity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn SetShowEnterpriseIndicator(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetShowEnterpriseIndicator)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn ShowEnterpriseIndicator(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowEnterpriseIndicator)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn IsIdentityManaged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsIdentityManaged)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn TryApplyProcessUIPolicy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryApplyProcessUIPolicy)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn ClearProcessUIPolicy() -> ::windows::core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ClearProcessUIPolicy)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn CreateCurrentThreadNetworkContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<ThreadNetworkContext> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCurrentThreadNetworkContext)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<ThreadNetworkContext>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Networking\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub fn GetPrimaryManagedIdentityForNetworkEndpointAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::HostName>>(endpointhost: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPrimaryManagedIdentityForNetworkEndpointAsync)(::core::mem::transmute_copy(this), endpointhost.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn RevokeContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RevokeContent)(::core::mem::transmute_copy(this), identity.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<ProtectionPolicyManager> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProtectionPolicyManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectedAccessSuspending<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectedAccessSuspending)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedAccessSuspending<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveProtectedAccessSuspending)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectedAccessResumed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectedAccessResumed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedAccessResumed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveProtectedAccessResumed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectedContentRevoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectedContentRevoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedContentRevoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveProtectedContentRevoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn CheckAccess<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1) -> ::windows::core::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ProtectionPolicyEvaluationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckAccess)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HasContentBeenRevokedSince<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(identity: Param0, since: Param1) -> ::windows::core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasContentBeenRevokedSince)(::core::mem::transmute_copy(this), identity.into_param().abi(), since.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn CheckAccessForApp<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1) -> ::windows::core::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: ProtectionPolicyEvaluationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckAccessForApp)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), &mut result__).from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessForAppAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn GetEnforcementLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<EnforcementLevel> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: EnforcementLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEnforcementLevel)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<EnforcementLevel>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn IsUserDecryptionAllowed<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUserDecryptionAllowed)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn IsProtectionUnderLockRequired<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsProtectionUnderLockRequired)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PolicyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PolicyChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePolicyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePolicyChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn IsProtectionEnabled() -> ::windows::core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsProtectionEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithAuditingInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessWithAuditingInfoAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2, messagefromapp: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessWithMessageAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithAuditingInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, apppackagefamilyname: Param1, auditinfo: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessForAppWithAuditingInfoAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1, auditinfo: Param2, messagefromapp: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessForAppWithMessageAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn LogAuditEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2) -> ::windows::core::Result<()> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).LogAuditEvent)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn IsRoamableProtectionEnabled<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<bool> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRoamableProtectionEnabled)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithBehaviorAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, targetidentity: Param1, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessWithBehaviorAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithBehaviorAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceidentity: Param0, apppackagefamilyname: Param1, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessForAppWithBehaviorAsync)(::core::mem::transmute_copy(this), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForAppAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceitemlist: Param0, apppackagefamilyname: Param1, auditinfo: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessToFilesForAppAsync)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceitemlist: Param0, apppackagefamilyname: Param1, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessToFilesForAppWithMessageAndBehaviorAsync)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForProcessAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>>(sourceitemlist: Param0, processid: u32, auditinfo: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessToFilesForProcessAsync)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), processid, auditinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, Param2: ::windows::core::IntoParam<'a, ProtectionPolicyAuditInfo>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceitemlist: Param0, processid: u32, auditinfo: Param2, messagefromapp: Param3, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessToFilesForProcessWithMessageAndBehaviorAsync)(::core::mem::transmute_copy(this), sourceitemlist.into_param().abi(), processid, auditinfo.into_param().abi(), messagefromapp.into_param().abi(), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsFileProtectionRequiredAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageItem>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(target: Param0, identity: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFileProtectionRequiredAsync)(::core::mem::transmute_copy(this), target.into_param().abi(), identity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsFileProtectionRequiredForNewFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(parentfolder: Param0, identity: Param1, desiredname: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFileProtectionRequiredForNewFileAsync)(::core::mem::transmute_copy(this), parentfolder.into_param().abi(), identity.into_param().abi(), desiredname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn PrimaryManagedIdentity() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryManagedIdentity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
    pub fn GetPrimaryManagedIdentityForIdentity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(identity: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPrimaryManagedIdentityForIdentity)(::core::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics<R, F: FnOnce(&IProtectionPolicyManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics2<R, F: FnOnce(&IProtectionPolicyManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics3<R, F: FnOnce(&IProtectionPolicyManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics4<R, F: FnOnce(&IProtectionPolicyManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProtectionPolicyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectionPolicyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectionPolicyManager {}
impl ::core::fmt::Debug for ProtectionPolicyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectionPolicyManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyManager;{d5703e18-a08d-47e6-a240-9934d7165eb5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_Vtbl;
    const IID: ::windows::core::GUID = <IProtectionPolicyManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectionPolicyManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyManager";
}
impl ::core::convert::From<ProtectionPolicyManager> for ::windows::core::IUnknown {
    fn from(value: ProtectionPolicyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyManager> for ::windows::core::IUnknown {
    fn from(value: &ProtectionPolicyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtectionPolicyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtectionPolicyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectionPolicyManager> for ::windows::core::IInspectable {
    fn from(value: ProtectionPolicyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionPolicyManager> for ::windows::core::IInspectable {
    fn from(value: &ProtectionPolicyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtectionPolicyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtectionPolicyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectionPolicyManager {}
unsafe impl ::core::marker::Sync for ProtectionPolicyManager {}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: Self = Self(0i32);
    pub const TreatOverridePolicyAsBlock: Self = Self(1i32);
}
impl ::core::marker::Copy for ProtectionPolicyRequestAccessBehavior {}
impl ::core::clone::Clone for ProtectionPolicyRequestAccessBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyRequestAccessBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProtectionPolicyRequestAccessBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionPolicyRequestAccessBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyRequestAccessBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectionPolicyRequestAccessBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyRequestAccessBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ThreadNetworkContext(::windows::core::IUnknown);
impl ThreadNetworkContext {
    #[doc = "*Required features: `\"Security_EnterpriseData\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ThreadNetworkContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ThreadNetworkContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ThreadNetworkContext {}
impl ::core::fmt::Debug for ThreadNetworkContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThreadNetworkContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThreadNetworkContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ThreadNetworkContext;{fa4ea8e9-ef13-405a-b12c-d7348c6f41fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ThreadNetworkContext {
    type Vtable = IThreadNetworkContext_Vtbl;
    const IID: ::windows::core::GUID = <IThreadNetworkContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ThreadNetworkContext {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ThreadNetworkContext";
}
impl ::core::convert::From<ThreadNetworkContext> for ::windows::core::IUnknown {
    fn from(value: ThreadNetworkContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThreadNetworkContext> for ::windows::core::IUnknown {
    fn from(value: &ThreadNetworkContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ThreadNetworkContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ThreadNetworkContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ThreadNetworkContext> for ::windows::core::IInspectable {
    fn from(value: ThreadNetworkContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThreadNetworkContext> for ::windows::core::IInspectable {
    fn from(value: &ThreadNetworkContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ThreadNetworkContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ThreadNetworkContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ThreadNetworkContext> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ThreadNetworkContext) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ThreadNetworkContext> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ThreadNetworkContext) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ThreadNetworkContext {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ThreadNetworkContext {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ThreadNetworkContext {}
unsafe impl ::core::marker::Sync for ThreadNetworkContext {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
