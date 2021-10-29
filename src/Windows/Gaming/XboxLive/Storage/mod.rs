#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveBlobGetResult(::windows::runtime::IInspectable);
impl GameSaveBlobGetResult {
    pub fn Status(&self) -> ::windows::runtime::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Value(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveBlobGetResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult;{917281e0-7201-4953-aa2c-4008f03aef45})");
}
unsafe impl ::windows::runtime::Interface for GameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2440200672, 29185, 18771, [170, 44, 64, 8, 240, 58, 239, 69]);
}
impl ::windows::runtime::RuntimeName for GameSaveBlobGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult";
}
impl ::std::convert::From<GameSaveBlobGetResult> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveBlobGetResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveBlobGetResult> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveBlobGetResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveBlobGetResult> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveBlobGetResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveBlobGetResult> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveBlobGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveBlobGetResult {}
unsafe impl ::std::marker::Sync for GameSaveBlobGetResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveBlobInfo(::windows::runtime::IInspectable);
impl GameSaveBlobInfo {
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveBlobInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo;{add38034-baf0-4645-b6d0-46edaffb3c2b})");
}
unsafe impl ::windows::runtime::Interface for GameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2916319284, 47856, 17989, [182, 208, 70, 237, 175, 251, 60, 43]);
}
impl ::windows::runtime::RuntimeName for GameSaveBlobInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo";
}
impl ::std::convert::From<GameSaveBlobInfo> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveBlobInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveBlobInfo> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveBlobInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveBlobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveBlobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveBlobInfo> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveBlobInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveBlobInfo> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveBlobInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveBlobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveBlobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveBlobInfo {}
unsafe impl ::std::marker::Sync for GameSaveBlobInfo {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveBlobInfoGetResult(::windows::runtime::IInspectable);
impl GameSaveBlobInfoGetResult {
    pub fn Status(&self) -> ::windows::runtime::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveBlobInfoGetResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult;{c7578582-3697-42bf-989c-665d923b5231})");
}
unsafe impl ::windows::runtime::Interface for GameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3344401794, 13975, 17087, [152, 156, 102, 93, 146, 59, 82, 49]);
}
impl ::windows::runtime::RuntimeName for GameSaveBlobInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult";
}
impl ::std::convert::From<GameSaveBlobInfoGetResult> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveBlobInfoGetResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveBlobInfoGetResult> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveBlobInfoGetResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveBlobInfoGetResult> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveBlobInfoGetResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveBlobInfoGetResult> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveBlobInfoGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveBlobInfoGetResult {}
unsafe impl ::std::marker::Sync for GameSaveBlobInfoGetResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveBlobInfoQuery(::windows::runtime::IInspectable);
impl GameSaveBlobInfoQuery {
    #[cfg(feature = "Foundation")]
    pub fn GetBlobInfoAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetBlobInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), startindex, maxnumberofitems, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveBlobInfoQuery {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery;{9fdd74b2-eeee-447b-a9d2-7f96c0f83208})");
}
unsafe impl ::windows::runtime::Interface for GameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2682090674, 61166, 17531, [169, 210, 127, 150, 192, 248, 50, 8]);
}
impl ::windows::runtime::RuntimeName for GameSaveBlobInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery";
}
impl ::std::convert::From<GameSaveBlobInfoQuery> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveBlobInfoQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveBlobInfoQuery> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveBlobInfoQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveBlobInfoQuery> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveBlobInfoQuery) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveBlobInfoQuery> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveBlobInfoQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveBlobInfoQuery {}
unsafe impl ::std::marker::Sync for GameSaveBlobInfoQuery {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveContainer(::windows::runtime::IInspectable);
impl GameSaveContainer {
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Provider(&self) -> ::windows::runtime::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveProvider>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SubmitUpdatesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::super::Storage::Streams::IBuffer>>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        blobstowrite: Param0,
        blobstodelete: Param1,
        displayname: Param2,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), blobstowrite.into_param().abi(), blobstodelete.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::super::Storage::Streams::IBuffer>>>(&self, blobstoread: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), blobstoread.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, blobstoread: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), blobstoread.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SubmitPropertySetUpdatesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IPropertySet>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        blobstowrite: Param0,
        blobstodelete: Param1,
        displayname: Param2,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), blobstowrite.into_param().abi(), blobstodelete.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    pub fn CreateBlobInfoQuery<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, blobnameprefix: Param0) -> ::windows::runtime::Result<GameSaveBlobInfoQuery> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), blobnameprefix.into_param().abi(), &mut result__).from_abi::<GameSaveBlobInfoQuery>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainer;{c3c08f89-563f-4ecd-9c6f-33fd0e323d10})");
}
unsafe impl ::windows::runtime::Interface for GameSaveContainer {
    type Vtable = IGameSaveContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3284176777, 22079, 20173, [156, 111, 51, 253, 14, 50, 61, 16]);
}
impl ::windows::runtime::RuntimeName for GameSaveContainer {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainer";
}
impl ::std::convert::From<GameSaveContainer> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveContainer> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveContainer> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveContainer> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveContainer {}
unsafe impl ::std::marker::Sync for GameSaveContainer {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveContainerInfo(::windows::runtime::IInspectable);
impl GameSaveContainerInfo {
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn TotalSize(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LastModifiedTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn NeedsSync(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveContainerInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo;{b7e27300-155d-4bb4-b2ba-930306f391b5})");
}
unsafe impl ::windows::runtime::Interface for GameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085071104, 5469, 19380, [178, 186, 147, 3, 6, 243, 145, 181]);
}
impl ::windows::runtime::RuntimeName for GameSaveContainerInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo";
}
impl ::std::convert::From<GameSaveContainerInfo> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveContainerInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveContainerInfo> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveContainerInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveContainerInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveContainerInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveContainerInfo> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveContainerInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveContainerInfo> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveContainerInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveContainerInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveContainerInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveContainerInfo {}
unsafe impl ::std::marker::Sync for GameSaveContainerInfo {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveContainerInfoGetResult(::windows::runtime::IInspectable);
impl GameSaveContainerInfoGetResult {
    pub fn Status(&self) -> ::windows::runtime::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveContainerInfoGetResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult;{ffc50d74-c581-4f9d-9e39-30a10c1e4c50})");
}
unsafe impl ::windows::runtime::Interface for GameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4291104116, 50561, 20381, [158, 57, 48, 161, 12, 30, 76, 80]);
}
impl ::windows::runtime::RuntimeName for GameSaveContainerInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult";
}
impl ::std::convert::From<GameSaveContainerInfoGetResult> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveContainerInfoGetResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveContainerInfoGetResult> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveContainerInfoGetResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveContainerInfoGetResult> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveContainerInfoGetResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveContainerInfoGetResult> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveContainerInfoGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveContainerInfoGetResult {}
unsafe impl ::std::marker::Sync for GameSaveContainerInfoGetResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveContainerInfoQuery(::windows::runtime::IInspectable);
impl GameSaveContainerInfoQuery {
    #[cfg(feature = "Foundation")]
    pub fn GetContainerInfoAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetContainerInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), startindex, maxnumberofitems, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveContainerInfoQuery {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery;{3c94e863-6f80-4327-9327-ffc11afd42b3})");
}
unsafe impl ::windows::runtime::Interface for GameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1016391779, 28544, 17191, [147, 39, 255, 193, 26, 253, 66, 179]);
}
impl ::windows::runtime::RuntimeName for GameSaveContainerInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery";
}
impl ::std::convert::From<GameSaveContainerInfoQuery> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveContainerInfoQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveContainerInfoQuery> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveContainerInfoQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveContainerInfoQuery> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveContainerInfoQuery) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveContainerInfoQuery> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveContainerInfoQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveContainerInfoQuery {}
unsafe impl ::std::marker::Sync for GameSaveContainerInfoQuery {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameSaveErrorStatus(pub i32);
impl GameSaveErrorStatus {
    pub const Ok: GameSaveErrorStatus = GameSaveErrorStatus(0i32);
    pub const Abort: GameSaveErrorStatus = GameSaveErrorStatus(-2147467260i32);
    pub const InvalidContainerName: GameSaveErrorStatus = GameSaveErrorStatus(-2138898431i32);
    pub const NoAccess: GameSaveErrorStatus = GameSaveErrorStatus(-2138898430i32);
    pub const OutOfLocalStorage: GameSaveErrorStatus = GameSaveErrorStatus(-2138898429i32);
    pub const UserCanceled: GameSaveErrorStatus = GameSaveErrorStatus(-2138898428i32);
    pub const UpdateTooBig: GameSaveErrorStatus = GameSaveErrorStatus(-2138898427i32);
    pub const QuotaExceeded: GameSaveErrorStatus = GameSaveErrorStatus(-2138898426i32);
    pub const ProvidedBufferTooSmall: GameSaveErrorStatus = GameSaveErrorStatus(-2138898425i32);
    pub const BlobNotFound: GameSaveErrorStatus = GameSaveErrorStatus(-2138898424i32);
    pub const NoXboxLiveInfo: GameSaveErrorStatus = GameSaveErrorStatus(-2138898423i32);
    pub const ContainerNotInSync: GameSaveErrorStatus = GameSaveErrorStatus(-2138898422i32);
    pub const ContainerSyncFailed: GameSaveErrorStatus = GameSaveErrorStatus(-2138898421i32);
    pub const UserHasNoXboxLiveInfo: GameSaveErrorStatus = GameSaveErrorStatus(-2138898420i32);
    pub const ObjectExpired: GameSaveErrorStatus = GameSaveErrorStatus(-2138898419i32);
}
impl ::std::convert::From<i32> for GameSaveErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameSaveErrorStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveErrorStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.XboxLive.Storage.GameSaveErrorStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveOperationResult(::windows::runtime::IInspectable);
impl GameSaveOperationResult {
    pub fn Status(&self) -> ::windows::runtime::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveOperationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveOperationResult;{cf0f1a05-24a0-4582-9a55-b1bbbb9388d8})");
}
unsafe impl ::windows::runtime::Interface for GameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3473873413, 9376, 17794, [154, 85, 177, 187, 187, 147, 136, 216]);
}
impl ::windows::runtime::RuntimeName for GameSaveOperationResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveOperationResult";
}
impl ::std::convert::From<GameSaveOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveOperationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveOperationResult {}
unsafe impl ::std::marker::Sync for GameSaveOperationResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveProvider(::windows::runtime::IInspectable);
impl GameSaveProvider {
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    pub fn CreateContainer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<GameSaveContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<GameSaveContainer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteContainerAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    pub fn CreateContainerInfoQuery(&self) -> ::windows::runtime::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveContainerInfoQuery>(result__)
        }
    }
    pub fn CreateContainerInfoQueryWithName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, containernameprefix: Param0) -> ::windows::runtime::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), containernameprefix.into_param().abi(), &mut result__).from_abi::<GameSaveContainerInfoQuery>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRemainingBytesInQuotaAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContainersChangedSinceLastSync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, serviceconfigid: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), serviceconfigid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetSyncOnDemandForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, serviceconfigid: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), serviceconfigid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>(result__)
        })
    }
    pub fn IGameSaveProviderStatics<R, F: FnOnce(&IGameSaveProviderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameSaveProvider, IGameSaveProviderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProvider;{90a60394-80fe-4211-97f8-a5de14dd95d2})");
}
unsafe impl ::windows::runtime::Interface for GameSaveProvider {
    type Vtable = IGameSaveProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2426798996, 33022, 16913, [151, 248, 165, 222, 20, 221, 149, 210]);
}
impl ::windows::runtime::RuntimeName for GameSaveProvider {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProvider";
}
impl ::std::convert::From<GameSaveProvider> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveProvider> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveProvider> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveProvider> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveProvider {}
unsafe impl ::std::marker::Sync for GameSaveProvider {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GameSaveProviderGetResult(::windows::runtime::IInspectable);
impl GameSaveProviderGetResult {
    pub fn Status(&self) -> ::windows::runtime::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::runtime::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameSaveProviderGetResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult;{3ab90816-d393-4d65-ac16-41c3e67ab945})");
}
unsafe impl ::windows::runtime::Interface for GameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(985204758, 54163, 19813, [172, 22, 65, 195, 230, 122, 185, 69]);
}
impl ::windows::runtime::RuntimeName for GameSaveProviderGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult";
}
impl ::std::convert::From<GameSaveProviderGetResult> for ::windows::runtime::IUnknown {
    fn from(value: GameSaveProviderGetResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GameSaveProviderGetResult> for ::windows::runtime::IUnknown {
    fn from(value: &GameSaveProviderGetResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GameSaveProviderGetResult> for ::windows::runtime::IInspectable {
    fn from(value: GameSaveProviderGetResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GameSaveProviderGetResult> for ::windows::runtime::IInspectable {
    fn from(value: &GameSaveProviderGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GameSaveProviderGetResult {}
unsafe impl ::std::marker::Sync for GameSaveProviderGetResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveBlobGetResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2440200672, 29185, 18771, [170, 44, 64, 8, 240, 58, 239, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveBlobInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2916319284, 47856, 17989, [182, 208, 70, 237, 175, 251, 60, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoGetResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3344401794, 13975, 17087, [152, 156, 102, 93, 146, 59, 82, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoQuery(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2682090674, 61166, 17531, [169, 210, 127, 150, 192, 248, 50, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveContainer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveContainer {
    type Vtable = IGameSaveContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3284176777, 22079, 20173, [156, 111, 51, 253, 14, 50, 61, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blobstowrite: ::windows::runtime::RawPtr, blobstodelete: ::windows::runtime::RawPtr, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blobstoread: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blobstoread: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blobstowrite: ::windows::runtime::RawPtr, blobstodelete: ::windows::runtime::RawPtr, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blobnameprefix: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveContainerInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085071104, 5469, 19380, [178, 186, 147, 3, 6, 243, 145, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoGetResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4291104116, 50561, 20381, [158, 57, 48, 161, 12, 30, 76, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoQuery(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1016391779, 28544, 17191, [147, 39, 255, 193, 26, 253, 66, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveOperationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3473873413, 9376, 17794, [154, 85, 177, 187, 187, 147, 136, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveProvider {
    type Vtable = IGameSaveProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2426798996, 33022, 16913, [151, 248, 165, 222, 20, 221, 149, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, containernameprefix: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveProviderGetResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(985204758, 54163, 19813, [172, 22, 65, 195, 230, 122, 185, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameSaveProviderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameSaveProviderStatics {
    type Vtable = IGameSaveProviderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3491577552, 31491, 17565, [140, 189, 52, 2, 132, 42, 16, 72]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, serviceconfigid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, serviceconfigid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
