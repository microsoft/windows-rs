#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BufferProtectUnprotectResult(::windows::runtime::IInspectable);
impl BufferProtectUnprotectResult {
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn ProtectionInfo(&self) -> ::windows::runtime::Result<DataProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DataProtectionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BufferProtectUnprotectResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.BufferProtectUnprotectResult;{47995edc-6cec-4e3a-b251-9e7485d79e7a})" ) ;
}
unsafe impl ::windows::runtime::Interface for BufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1201233628,
        27884,
        20026,
        [178, 81, 158, 116, 133, 215, 158, 122],
    );
}
impl ::windows::runtime::RuntimeName for BufferProtectUnprotectResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.BufferProtectUnprotectResult";
}
impl ::std::convert::From<BufferProtectUnprotectResult> for ::windows::runtime::IUnknown {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BufferProtectUnprotectResult> for ::windows::runtime::IUnknown {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BufferProtectUnprotectResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BufferProtectUnprotectResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BufferProtectUnprotectResult> for ::windows::runtime::IInspectable {
    fn from(value: BufferProtectUnprotectResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BufferProtectUnprotectResult> for ::windows::runtime::IInspectable {
    fn from(value: &BufferProtectUnprotectResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BufferProtectUnprotectResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BufferProtectUnprotectResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BufferProtectUnprotectResult {}
unsafe impl ::std::marker::Sync for BufferProtectUnprotectResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DataProtectionInfo(::windows::runtime::IInspectable);
impl DataProtectionInfo {
    pub fn Status(&self) -> ::windows::runtime::Result<DataProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__: DataProtectionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DataProtectionStatus>(result__)
        }
    }
    pub fn Identity(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataProtectionInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.DataProtectionInfo;{8420b0c1-5e31-4405-9540-3f943af0cb26})" ) ;
}
unsafe impl ::windows::runtime::Interface for DataProtectionInfo {
    type Vtable = IDataProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2216734913,
        24113,
        17413,
        [149, 64, 63, 148, 58, 240, 203, 38],
    );
}
impl ::windows::runtime::RuntimeName for DataProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionInfo";
}
impl ::std::convert::From<DataProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: DataProtectionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &DataProtectionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DataProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: DataProtectionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &DataProtectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DataProtectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DataProtectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataProtectionInfo {}
unsafe impl ::std::marker::Sync for DataProtectionInfo {}
pub struct DataProtectionManager {}
impl DataProtectionManager {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        data: Param0,
        identity: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>,
    > {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                data.into_param().abi(),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>(
                result__,
            )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>,
    >(
        data: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>,
    > {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                data.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>(
                result__,
            )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectStreamAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream>,
    >(
        unprotectedstream: Param0,
        identity: Param1,
        protectedstream: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                unprotectedstream.into_param().abi(),
                identity.into_param().abi(),
                protectedstream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectStreamAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream>,
    >(
        protectedstream: Param0,
        unprotectedstream: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                protectedstream.into_param().abi(),
                unprotectedstream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetProtectionInfoAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>,
    >(
        protecteddata: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                protecteddata.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetStreamProtectionInfoAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>,
    >(
        protectedstream: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                protectedstream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>(result__)
        })
    }
    pub fn IDataProtectionManagerStatics<
        R,
        F: FnOnce(&IDataProtectionManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DataProtectionManager,
            IDataProtectionManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for DataProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionManager";
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for DataProtectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DataProtectionStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DataProtectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Security.EnterpriseData.DataProtectionStatus;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: EnforcementLevel = EnforcementLevel(0i32);
    pub const Silent: EnforcementLevel = EnforcementLevel(1i32);
    pub const Override: EnforcementLevel = EnforcementLevel(2i32);
    pub const Block: EnforcementLevel = EnforcementLevel(3i32);
}
impl ::std::convert::From<i32> for EnforcementLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EnforcementLevel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EnforcementLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Security.EnterpriseData.EnforcementLevel;i4)",
    );
}
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct EnterpriseDataContract(pub u8);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct FileProtectionInfo(::windows::runtime::IInspectable);
impl FileProtectionInfo {
    pub fn Status(&self) -> ::windows::runtime::Result<FileProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__: FileProtectionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FileProtectionStatus>(result__)
        }
    }
    pub fn IsRoamable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Identity(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn IsProtectWhileOpenSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IFileProtectionInfo2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileProtectionInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.FileProtectionInfo;{4ee96486-147e-4dd0-8faf-5253ed91ad0c})" ) ;
}
unsafe impl ::windows::runtime::Interface for FileProtectionInfo {
    type Vtable = IFileProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1323918470,
        5246,
        19920,
        [143, 175, 82, 83, 237, 145, 173, 12],
    );
}
impl ::windows::runtime::RuntimeName for FileProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionInfo";
}
impl ::std::convert::From<FileProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: FileProtectionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileProtectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &FileProtectionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FileProtectionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<FileProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: FileProtectionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileProtectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &FileProtectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for FileProtectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a FileProtectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FileProtectionInfo {}
unsafe impl ::std::marker::Sync for FileProtectionInfo {}
pub struct FileProtectionManager {}
impl FileProtectionManager {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        target: Param0,
        identity: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                target.into_param().abi(),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CopyProtectionAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
    >(
        source: Param0,
        target: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetProtectionInfoAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
    >(
        source: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveFileAsContainerAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
    >(
        protectedfile: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>,
    > {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                protectedfile.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>(
                result__,
            )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
    >(
        containerfile: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>,
    > {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                containerfile.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(
                result__,
            )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerWithTargetAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
    >(
        containerfile: Param0,
        target: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>,
    > {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                containerfile.into_param().abi(),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(
                result__,
            )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateProtectedAndOpenAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        parentfolder: Param0,
        desiredname: Param1,
        identity: Param2,
        collisionoption: super::super::Storage::CreationCollisionOption,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>,
    > {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                parentfolder.into_param().abi(),
                desiredname.into_param().abi(),
                identity.into_param().abi(),
                collisionoption,
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>>(
                result__,
            )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsContainerAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
    >(
        file: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                file.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
    >(
        containerfile: Param0,
        target: Param1,
        collisionoption: super::super::Storage::NameCollisionOption,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>,
    > {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                containerfile.into_param().abi(),
                target.into_param().abi(),
                collisionoption,
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>(
                result__,
            )
        })
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub fn SaveFileAsContainerWithSharingAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        protectedfile: Param0,
        sharedwithidentities: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>,
    > {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                protectedfile.into_param().abi(),
                sharedwithidentities.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>(
                result__,
            )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn UnprotectAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
    >(
        target: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn UnprotectWithOptionsAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
        Param1: ::windows::runtime::IntoParam<'a, FileUnprotectOptions>,
    >(
        target: Param0,
        options: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                target.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>(result__)
        })
    }
    pub fn IFileProtectionManagerStatics<
        R,
        F: FnOnce(&IFileProtectionManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            FileProtectionManager,
            IFileProtectionManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFileProtectionManagerStatics2<
        R,
        F: FnOnce(&IFileProtectionManagerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            FileProtectionManager,
            IFileProtectionManagerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFileProtectionManagerStatics3<
        R,
        F: FnOnce(&IFileProtectionManagerStatics3) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            FileProtectionManager,
            IFileProtectionManagerStatics3,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for FileProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionManager";
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for FileProtectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FileProtectionStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FileProtectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Security.EnterpriseData.FileProtectionStatus;i4)",
    );
}
pub struct FileRevocationManager {}
impl FileRevocationManager {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        storageitem: Param0,
        enterpriseidentity: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>
    {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                storageitem.into_param().abi(),
                enterpriseidentity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CopyProtectionAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
    >(
        sourcestorageitem: Param0,
        targetstorageitem: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                sourcestorageitem.into_param().abi(),
                targetstorageitem.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Revoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        enterpriseidentity: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                enterpriseidentity.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetStatusAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
    >(
        storageitem: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>
    {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                storageitem.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>(result__)
        })
    }
    pub fn IFileRevocationManagerStatics<
        R,
        F: FnOnce(&IFileRevocationManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            FileRevocationManager,
            IFileRevocationManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for FileRevocationManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileRevocationManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct FileUnprotectOptions(::windows::runtime::IInspectable);
impl FileUnprotectOptions {
    pub fn SetAudit(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Audit(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Create(audit: bool) -> ::windows::runtime::Result<FileUnprotectOptions> {
        Self::IFileUnprotectOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                audit,
                &mut result__,
            )
            .from_abi::<FileUnprotectOptions>(result__)
        })
    }
    pub fn IFileUnprotectOptionsFactory<
        R,
        F: FnOnce(&IFileUnprotectOptionsFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            FileUnprotectOptions,
            IFileUnprotectOptionsFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileUnprotectOptions {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.FileUnprotectOptions;{7d1312f1-3b0d-4dd8-a1f8-1ec53822e2f3})" ) ;
}
unsafe impl ::windows::runtime::Interface for FileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2098402033,
        15117,
        19928,
        [161, 248, 30, 197, 56, 34, 226, 243],
    );
}
impl ::windows::runtime::RuntimeName for FileUnprotectOptions {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileUnprotectOptions";
}
impl ::std::convert::From<FileUnprotectOptions> for ::windows::runtime::IUnknown {
    fn from(value: FileUnprotectOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileUnprotectOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FileUnprotectOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileUnprotectOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FileUnprotectOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<FileUnprotectOptions> for ::windows::runtime::IInspectable {
    fn from(value: FileUnprotectOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileUnprotectOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FileUnprotectOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for FileUnprotectOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a FileUnprotectOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FileUnprotectOptions {}
unsafe impl ::std::marker::Sync for FileUnprotectOptions {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBufferProtectUnprotectResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1201233628,
        27884,
        20026,
        [178, 81, 158, 116, 133, 215, 158, 122],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferProtectUnprotectResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDataProtectionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProtectionInfo {
    type Vtable = IDataProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2216734913,
        24113,
        17413,
        [149, 64, 63, 148, 58, 240, 203, 38],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut DataProtectionStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDataProtectionManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProtectionManagerStatics {
    type Vtable = IDataProtectionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3054803828,
        37188,
        20196,
        [138, 138, 48, 181, 243, 97, 67, 14],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        data: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        data: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unprotectedstream: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        protectedstream: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protectedstream: ::windows::runtime::RawPtr,
        unprotectedstream: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protecteddata: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protectedstream: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileProtectionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionInfo {
    type Vtable = IFileProtectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1323918470,
        5246,
        19920,
        [143, 175, 82, 83, 237, 145, 173, 12],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FileProtectionStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileProtectionInfo2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionInfo2 {
    type Vtable = IFileProtectionInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2182232652,
        21882,
        18829,
        [142, 148, 148, 76, 213, 131, 100, 50],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionManagerStatics {
    type Vtable = IFileProtectionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1481047195,
        58899,
        17003,
        [187, 56, 136, 203, 161, 220, 154, 219],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        source: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        source: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protectedfile: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        containerfile: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        containerfile: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parentfolder: ::windows::runtime::RawPtr,
        desiredname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        collisionoption: super::super::Storage::CreationCollisionOption,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionManagerStatics2 {
    type Vtable = IFileProtectionManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2211620677,
        1155,
        16811,
        [178, 213, 188, 127, 35, 215, 78, 187],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        file: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        containerfile: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        collisionoption: super::super::Storage::NameCollisionOption,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        protectedfile: ::windows::runtime::RawPtr,
        sharedwithidentities: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileProtectionManagerStatics3 {
    type Vtable = IFileProtectionManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1763214490,
        25167,
        18134,
        [178, 65, 233, 205, 95, 223, 62, 63],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        options: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileRevocationManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileRevocationManagerStatics {
    type Vtable = IFileRevocationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        627817533,
        7261,
        16992,
        [140, 117, 145, 68, 207, 183, 139, 169],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRevocationManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        storageitem: ::windows::runtime::RawPtr,
        enterpriseidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourcestorageitem: ::windows::runtime::RawPtr,
        targetstorageitem: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enterpriseidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        storageitem: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileUnprotectOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2098402033,
        15117,
        19928,
        [161, 248, 30, 197, 56, 34, 226, 243],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptions_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFileUnprotectOptionsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileUnprotectOptionsFactory {
    type Vtable = IFileUnprotectOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1370403740,
        55948,
        19519,
        [155, 251, 203, 115, 167, 204, 224, 221],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptionsFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        audit: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectedAccessResumedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2890779225,
        23936,
        20117,
        [140, 95, 133, 57, 69, 14, 235, 224],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessResumedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectedAccessSuspendingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1973523424,
        41796,
        17055,
        [185, 117, 4, 252, 31, 136, 193, 133],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessSuspendingEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectedContainerExportResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        961081237,
        63483,
        19266,
        [175, 176, 223, 112, 180, 21, 67, 193],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerExportResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ProtectedImportExportStatus,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectedContainerImportResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3451355345,
        59323,
        19738,
        [147, 57, 52, 220, 65, 20, 159, 155],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerImportResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ProtectedImportExportStatus,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectedContentRevokedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1667786785,
        22713,
        18414,
        [147, 217, 240, 247, 65, 207, 67, 240],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContentRevokedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectedFileCreateResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        686026090,
        59879,
        18947,
        [159, 83, 189, 177, 97, 114, 105, 155],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedFileCreateResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1113241572,
        65207,
        17660,
        [179, 187, 195, 196, 215, 236, 190, 187],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ProtectionPolicyAuditAction,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ProtectionPolicyAuditAction,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfoFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyAuditInfoFactory {
    type Vtable = IProtectionPolicyAuditInfoFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2127829003,
        37608,
        17109,
        [131, 212, 37, 68, 11, 66, 53, 73],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfoFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        action: ProtectionPolicyAuditAction,
        datadescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        sourcedescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetdescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        action: ProtectionPolicyAuditAction,
        datadescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3580902936,
        41101,
        18406,
        [162, 64, 153, 52, 215, 22, 94, 181],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManager2 {
    type Vtable = IProtectionPolicyManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2885112442,
        33845,
        16767,
        [153, 182, 81, 190, 175, 54, 88, 136],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics {
    type Vtable = IProtectionPolicyManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3233807462,
        35901,
        19798,
        [136, 4, 198, 143, 10, 211, 46, 197],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        endpointhost: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ProtectionPolicyEvaluationResult,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics2 {
    type Vtable = IProtectionPolicyManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3062864524,
        14816,
        17993,
        [178, 228, 7, 10, 184, 165, 121, 179],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        since: super::super::Foundation::DateTime,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ProtectionPolicyEvaluationResult,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut EnforcementLevel,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics3 {
    type Vtable = IProtectionPolicyManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1224711820,
        27247,
        19871,
        [188, 237, 24, 171, 83, 122, 160, 21],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerStatics4 {
    type Vtable = IProtectionPolicyManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        548902107,
        52413,
        18703,
        [140, 131, 73, 204, 183, 122, 234, 108],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: ProtectionPolicyRequestAccessBehavior,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: ProtectionPolicyRequestAccessBehavior,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceitemlist: ::windows::runtime::RawPtr,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    )))]
    usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceitemlist: ::windows::runtime::RawPtr,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfo: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: ProtectionPolicyRequestAccessBehavior,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    )))]
    usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceitemlist: ::windows::runtime::RawPtr,
        processid: u32,
        auditinfo: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    )))]
    usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourceitemlist: ::windows::runtime::RawPtr,
        processid: u32,
        auditinfo: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: ProtectionPolicyRequestAccessBehavior,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    )))]
    usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parentfolder: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        desiredname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IThreadNetworkContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IThreadNetworkContext {
    type Vtable = IThreadNetworkContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4199459049,
        61203,
        16474,
        [177, 44, 215, 52, 140, 111, 65, 252],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadNetworkContext_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectedAccessResumedEventArgs(::windows::runtime::IInspectable);
impl ProtectedAccessResumedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedAccessResumedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs;{ac4dca59-5d80-4e95-8c5f-8539450eebe0})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2890779225,
        23936,
        20117,
        [140, 95, 133, 57, 69, 14, 235, 224],
    );
}
impl ::windows::runtime::RuntimeName for ProtectedAccessResumedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs";
}
impl ::std::convert::From<ProtectedAccessResumedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectedAccessResumedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectedAccessResumedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectedAccessResumedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectedAccessResumedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedAccessResumedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectedAccessResumedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedAccessResumedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectedAccessResumedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectedAccessResumedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectedAccessResumedEventArgs {}
unsafe impl ::std::marker::Sync for ProtectedAccessResumedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectedAccessSuspendingEventArgs(::windows::runtime::IInspectable);
impl ProtectedAccessSuspendingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedAccessSuspendingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs;{75a193e0-a344-429f-b975-04fc1f88c185})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1973523424,
        41796,
        17055,
        [185, 117, 4, 252, 31, 136, 193, 133],
    );
}
impl ::windows::runtime::RuntimeName for ProtectedAccessSuspendingEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs";
}
impl ::std::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectedAccessSuspendingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectedAccessSuspendingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectedAccessSuspendingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectedAccessSuspendingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedAccessSuspendingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectedAccessSuspendingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &ProtectedAccessSuspendingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectedAccessSuspendingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectedAccessSuspendingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectedAccessSuspendingEventArgs {}
unsafe impl ::std::marker::Sync for ProtectedAccessSuspendingEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectedContainerExportResult(::windows::runtime::IInspectable);
impl ProtectedContainerExportResult {
    pub fn Status(&self) -> ::windows::runtime::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__: ProtectedImportExportStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedContainerExportResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectedContainerExportResult;{3948ef95-f7fb-4b42-afb0-df70b41543c1})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        961081237,
        63483,
        19266,
        [175, 176, 223, 112, 180, 21, 67, 193],
    );
}
impl ::windows::runtime::RuntimeName for ProtectedContainerExportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerExportResult";
}
impl ::std::convert::From<ProtectedContainerExportResult> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedContainerExportResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectedContainerExportResult> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectedContainerExportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectedContainerExportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectedContainerExportResult> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedContainerExportResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectedContainerExportResult> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedContainerExportResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectedContainerExportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectedContainerExportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectedContainerExportResult {}
unsafe impl ::std::marker::Sync for ProtectedContainerExportResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectedContainerImportResult(::windows::runtime::IInspectable);
impl ProtectedContainerImportResult {
    pub fn Status(&self) -> ::windows::runtime::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__: ProtectedImportExportStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ProtectedImportExportStatus>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedContainerImportResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectedContainerImportResult;{cdb780d1-e7bb-4d1a-9339-34dc41149f9b})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3451355345,
        59323,
        19738,
        [147, 57, 52, 220, 65, 20, 159, 155],
    );
}
impl ::windows::runtime::RuntimeName for ProtectedContainerImportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerImportResult";
}
impl ::std::convert::From<ProtectedContainerImportResult> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedContainerImportResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectedContainerImportResult> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectedContainerImportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectedContainerImportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectedContainerImportResult> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedContainerImportResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectedContainerImportResult> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedContainerImportResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectedContainerImportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectedContainerImportResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectedContainerImportResult {}
unsafe impl ::std::marker::Sync for ProtectedContainerImportResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectedContentRevokedEventArgs(::windows::runtime::IInspectable);
impl ProtectedContentRevokedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedContentRevokedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs;{63686821-58b9-47ee-93d9-f0f741cf43f0})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1667786785,
        22713,
        18414,
        [147, 217, 240, 247, 65, 207, 67, 240],
    );
}
impl ::windows::runtime::RuntimeName for ProtectedContentRevokedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs";
}
impl ::std::convert::From<ProtectedContentRevokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectedContentRevokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectedContentRevokedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectedContentRevokedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectedContentRevokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedContentRevokedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectedContentRevokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedContentRevokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectedContentRevokedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectedContentRevokedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectedContentRevokedEventArgs {}
unsafe impl ::std::marker::Sync for ProtectedContentRevokedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectedFileCreateResult(::windows::runtime::IInspectable);
impl ProtectedFileCreateResult {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(
        &self,
    ) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn ProtectionInfo(&self) -> ::windows::runtime::Result<FileProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FileProtectionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedFileCreateResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectedFileCreateResult;{28e3ed6a-e9e7-4a03-9f53-bdb16172699b})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        686026090,
        59879,
        18947,
        [159, 83, 189, 177, 97, 114, 105, 155],
    );
}
impl ::windows::runtime::RuntimeName for ProtectedFileCreateResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedFileCreateResult";
}
impl ::std::convert::From<ProtectedFileCreateResult> for ::windows::runtime::IUnknown {
    fn from(value: ProtectedFileCreateResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectedFileCreateResult> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectedFileCreateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectedFileCreateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectedFileCreateResult> for ::windows::runtime::IInspectable {
    fn from(value: ProtectedFileCreateResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectedFileCreateResult> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectedFileCreateResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectedFileCreateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectedFileCreateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectedFileCreateResult {}
unsafe impl ::std::marker::Sync for ProtectedFileCreateResult {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ProtectedImportExportStatus(pub i32);
impl ProtectedImportExportStatus {
    pub const Ok: ProtectedImportExportStatus = ProtectedImportExportStatus(0i32);
    pub const Undetermined: ProtectedImportExportStatus = ProtectedImportExportStatus(1i32);
    pub const Unprotected: ProtectedImportExportStatus = ProtectedImportExportStatus(2i32);
    pub const Revoked: ProtectedImportExportStatus = ProtectedImportExportStatus(3i32);
    pub const NotRoamable: ProtectedImportExportStatus = ProtectedImportExportStatus(4i32);
    pub const ProtectedToOtherIdentity: ProtectedImportExportStatus =
        ProtectedImportExportStatus(5i32);
    pub const LicenseExpired: ProtectedImportExportStatus = ProtectedImportExportStatus(6i32);
    pub const AccessSuspended: ProtectedImportExportStatus = ProtectedImportExportStatus(7i32);
}
impl ::std::convert::From<i32> for ProtectedImportExportStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectedImportExportStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectedImportExportStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Security.EnterpriseData.ProtectedImportExportStatus;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(0i32);
    pub const CopyToLocation: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(1i32);
    pub const SendToRecipient: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(2i32);
    pub const Other: ProtectionPolicyAuditAction = ProtectionPolicyAuditAction(3i32);
}
impl ::std::convert::From<i32> for ProtectionPolicyAuditAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectionPolicyAuditAction {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyAuditAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Security.EnterpriseData.ProtectionPolicyAuditAction;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectionPolicyAuditInfo(::windows::runtime::IInspectable);
impl ProtectionPolicyAuditInfo {
    pub fn SetAction(&self, value: ProtectionPolicyAuditAction) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Action(&self) -> ::windows::runtime::Result<ProtectionPolicyAuditAction> {
        let this = self;
        unsafe {
            let mut result__: ProtectionPolicyAuditAction = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ProtectionPolicyAuditAction>(result__)
        }
    }
    pub fn SetDataDescription<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DataDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetSourceDescription<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SourceDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetTargetDescription<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TargetDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Create<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        action: ProtectionPolicyAuditAction,
        datadescription: Param1,
        sourcedescription: Param2,
        targetdescription: Param3,
    ) -> ::windows::runtime::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                action,
                datadescription.into_param().abi(),
                sourcedescription.into_param().abi(),
                targetdescription.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    pub fn CreateWithActionAndDataDescription<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        action: ProtectionPolicyAuditAction,
        datadescription: Param1,
    ) -> ::windows::runtime::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                action,
                datadescription.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ProtectionPolicyAuditInfo>(result__)
        })
    }
    pub fn IProtectionPolicyAuditInfoFactory<
        R,
        F: FnOnce(&IProtectionPolicyAuditInfoFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ProtectionPolicyAuditInfo,
            IProtectionPolicyAuditInfoFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyAuditInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo;{425ab7e4-feb7-44fc-b3bb-c3c4d7ecbebb})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1113241572,
        65207,
        17660,
        [179, 187, 195, 196, 215, 236, 190, 187],
    );
}
impl ::windows::runtime::RuntimeName for ProtectionPolicyAuditInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo";
}
impl ::std::convert::From<ProtectionPolicyAuditInfo> for ::windows::runtime::IUnknown {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectionPolicyAuditInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectionPolicyAuditInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectionPolicyAuditInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectionPolicyAuditInfo> for ::windows::runtime::IInspectable {
    fn from(value: ProtectionPolicyAuditInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectionPolicyAuditInfo> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectionPolicyAuditInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectionPolicyAuditInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectionPolicyAuditInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectionPolicyAuditInfo {}
unsafe impl ::std::marker::Sync for ProtectionPolicyAuditInfo {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(0i32);
    pub const Blocked: ProtectionPolicyEvaluationResult = ProtectionPolicyEvaluationResult(1i32);
    pub const ConsentRequired: ProtectionPolicyEvaluationResult =
        ProtectionPolicyEvaluationResult(2i32);
}
impl ::std::convert::From<i32> for ProtectionPolicyEvaluationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectionPolicyEvaluationResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyEvaluationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProtectionPolicyManager(::windows::runtime::IInspectable);
impl ProtectionPolicyManager {
    pub fn SetIdentity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Identity(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn IsIdentityManaged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn TryApplyProcessUIPolicy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn ClearProcessUIPolicy() -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        })
    }
    pub fn CreateCurrentThreadNetworkContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<ThreadNetworkContext> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ThreadNetworkContext>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub fn GetPrimaryManagedIdentityForNetworkEndpointAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Networking::HostName>,
    >(
        endpointhost: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>,
    > {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                endpointhost.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(
                result__,
            )
        })
    }
    pub fn RevokeContent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn GetForCurrentView() -> ::windows::runtime::Result<ProtectionPolicyManager> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ProtectionPolicyManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ProtectedAccessSuspending<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedAccessSuspending<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ProtectedAccessResumed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedAccessResumed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ProtectedContentRevoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedContentRevoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn CheckAccess<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        targetidentity: Param1,
    ) -> ::windows::runtime::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ProtectionPolicyEvaluationResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                sourceidentity.into_param().abi(),
                targetidentity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        targetidentity: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .20 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , targetidentity . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn HasContentBeenRevokedSince<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        identity: Param0,
        since: Param1,
    ) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                since.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn CheckAccessForApp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        apppackagefamilyname: Param1,
    ) -> ::windows::runtime::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: ProtectionPolicyEvaluationResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                sourceidentity.into_param().abi(),
                apppackagefamilyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ProtectionPolicyEvaluationResult>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        apppackagefamilyname: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , apppackagefamilyname . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    pub fn GetEnforcementLevel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<EnforcementLevel> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: EnforcementLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<EnforcementLevel>(result__)
        })
    }
    pub fn IsUserDecryptionAllowed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IsProtectionUnderLockRequired<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn PolicyChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePolicyChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IsProtectionEnabled() -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetShowEnterpriseIndicator(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShowEnterpriseIndicator(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithAuditingInfoAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
    >(
        sourceidentity: Param0,
        targetidentity: Param1,
        auditinfo: Param2,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , targetidentity . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithMessageAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        targetidentity: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .7 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , targetidentity . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , messagefromapp . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithAuditingInfoAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
    >(
        sourceidentity: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , apppackagefamilyname . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithMessageAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .9 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , apppackagefamilyname . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , messagefromapp . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    pub fn LogAuditEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
    >(
        sourceidentity: Param0,
        targetidentity: Param1,
        auditinfo: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                sourceidentity.into_param().abi(),
                targetidentity.into_param().abi(),
                auditinfo.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IsRoamableProtectionEnabled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithBehaviorAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        targetidentity: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .7 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , targetidentity . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , messagefromapp . into_param ( ) . abi ( ) , behavior , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithBehaviorAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceidentity: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , sourceidentity . into_param ( ) . abi ( ) , apppackagefamilyname . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , messagefromapp . into_param ( ) . abi ( ) , behavior , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub fn RequestAccessToFilesForAppAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>,
        >,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
    >(
        sourceitemlist: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .9 ) ( :: std :: mem :: transmute_copy ( this ) , sourceitemlist . into_param ( ) . abi ( ) , apppackagefamilyname . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>,
        >,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceitemlist: Param0,
        apppackagefamilyname: Param1,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .10 ) ( :: std :: mem :: transmute_copy ( this ) , sourceitemlist . into_param ( ) . abi ( ) , apppackagefamilyname . into_param ( ) . abi ( ) , auditinfo . into_param ( ) . abi ( ) , messagefromapp . into_param ( ) . abi ( ) , behavior , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub fn RequestAccessToFilesForProcessAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>,
        >,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
    >(
        sourceitemlist: Param0,
        processid: u32,
        auditinfo: Param2,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .11 ) ( :: std :: mem :: transmute_copy ( this ) , sourceitemlist . into_param ( ) . abi ( ) , processid , auditinfo . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>,
        >,
        Param2: ::windows::runtime::IntoParam<'a, ProtectionPolicyAuditInfo>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sourceitemlist: Param0,
        processid: u32,
        auditinfo: Param2,
        messagefromapp: Param3,
        behavior: ProtectionPolicyRequestAccessBehavior,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>,
    > {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .12 ) ( :: std :: mem :: transmute_copy ( this ) , sourceitemlist . into_param ( ) . abi ( ) , processid , auditinfo . into_param ( ) . abi ( ) , messagefromapp . into_param ( ) . abi ( ) , behavior , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < ProtectionPolicyEvaluationResult > > ( result__ )
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsFileProtectionRequiredAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageItem>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        target: Param0,
        identity: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                target.into_param().abi(),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsFileProtectionRequiredForNewFileAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        parentfolder: Param0,
        identity: Param1,
        desiredname: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                parentfolder.into_param().abi(),
                identity.into_param().abi(),
                desiredname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn PrimaryManagedIdentity() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn GetPrimaryManagedIdentityForIdentity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        identity: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                identity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IProtectionPolicyManagerStatics<
        R,
        F: FnOnce(&IProtectionPolicyManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ProtectionPolicyManager,
            IProtectionPolicyManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics2<
        R,
        F: FnOnce(&IProtectionPolicyManagerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ProtectionPolicyManager,
            IProtectionPolicyManagerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics3<
        R,
        F: FnOnce(&IProtectionPolicyManagerStatics3) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ProtectionPolicyManager,
            IProtectionPolicyManagerStatics3,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProtectionPolicyManagerStatics4<
        R,
        F: FnOnce(&IProtectionPolicyManagerStatics4) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ProtectionPolicyManager,
            IProtectionPolicyManagerStatics4,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyManager {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ProtectionPolicyManager;{d5703e18-a08d-47e6-a240-9934d7165eb5})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3580902936,
        41101,
        18406,
        [162, 64, 153, 52, 215, 22, 94, 181],
    );
}
impl ::windows::runtime::RuntimeName for ProtectionPolicyManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyManager";
}
impl ::std::convert::From<ProtectionPolicyManager> for ::windows::runtime::IUnknown {
    fn from(value: ProtectionPolicyManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectionPolicyManager> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectionPolicyManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProtectionPolicyManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProtectionPolicyManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProtectionPolicyManager> for ::windows::runtime::IInspectable {
    fn from(value: ProtectionPolicyManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectionPolicyManager> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectionPolicyManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProtectionPolicyManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProtectionPolicyManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectionPolicyManager {}
unsafe impl ::std::marker::Sync for ProtectionPolicyManager {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: ProtectionPolicyRequestAccessBehavior =
        ProtectionPolicyRequestAccessBehavior(0i32);
    pub const TreatOverridePolicyAsBlock: ProtectionPolicyRequestAccessBehavior =
        ProtectionPolicyRequestAccessBehavior(1i32);
}
impl ::std::convert::From<i32> for ProtectionPolicyRequestAccessBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectionPolicyRequestAccessBehavior {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionPolicyRequestAccessBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Security.EnterpriseData.ProtectionPolicyRequestAccessBehavior;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ThreadNetworkContext(::windows::runtime::IInspectable);
impl ThreadNetworkContext {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ThreadNetworkContext {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Security.EnterpriseData.ThreadNetworkContext;{fa4ea8e9-ef13-405a-b12c-d7348c6f41fc})" ) ;
}
unsafe impl ::windows::runtime::Interface for ThreadNetworkContext {
    type Vtable = IThreadNetworkContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4199459049,
        61203,
        16474,
        [177, 44, 215, 52, 140, 111, 65, 252],
    );
}
impl ::windows::runtime::RuntimeName for ThreadNetworkContext {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ThreadNetworkContext";
}
impl ::std::convert::From<ThreadNetworkContext> for ::windows::runtime::IUnknown {
    fn from(value: ThreadNetworkContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ThreadNetworkContext> for ::windows::runtime::IUnknown {
    fn from(value: &ThreadNetworkContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ThreadNetworkContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ThreadNetworkContext> for ::windows::runtime::IInspectable {
    fn from(value: ThreadNetworkContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ThreadNetworkContext> for ::windows::runtime::IInspectable {
    fn from(value: &ThreadNetworkContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ThreadNetworkContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ThreadNetworkContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ThreadNetworkContext> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ThreadNetworkContext) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ThreadNetworkContext> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ThreadNetworkContext) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for ThreadNetworkContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &ThreadNetworkContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ThreadNetworkContext {}
unsafe impl ::std::marker::Sync for ThreadNetworkContext {}
