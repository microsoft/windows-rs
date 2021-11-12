#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveBlobGetResult(pub ::windows::core::IInspectable);
impl GameSaveBlobGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult;{917281e0-7201-4953-aa2c-4008f03aef45})");
}
unsafe impl ::windows::core::Interface for GameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917281e0_7201_4953_aa2c_4008f03aef45);
}
impl ::windows::core::RuntimeName for GameSaveBlobGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult";
}
impl ::core::convert::From<GameSaveBlobGetResult> for ::windows::core::IUnknown {
    fn from(value: GameSaveBlobGetResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveBlobGetResult> for ::windows::core::IUnknown {
    fn from(value: &GameSaveBlobGetResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveBlobGetResult> for ::windows::core::IInspectable {
    fn from(value: GameSaveBlobGetResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveBlobGetResult> for ::windows::core::IInspectable {
    fn from(value: &GameSaveBlobGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveBlobGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobGetResult {}
unsafe impl ::core::marker::Sync for GameSaveBlobGetResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveBlobInfo(pub ::windows::core::IInspectable);
impl GameSaveBlobInfo {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo;{add38034-baf0-4645-b6d0-46edaffb3c2b})");
}
unsafe impl ::windows::core::Interface for GameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadd38034_baf0_4645_b6d0_46edaffb3c2b);
}
impl ::windows::core::RuntimeName for GameSaveBlobInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo";
}
impl ::core::convert::From<GameSaveBlobInfo> for ::windows::core::IUnknown {
    fn from(value: GameSaveBlobInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveBlobInfo> for ::windows::core::IUnknown {
    fn from(value: &GameSaveBlobInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveBlobInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveBlobInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveBlobInfo> for ::windows::core::IInspectable {
    fn from(value: GameSaveBlobInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveBlobInfo> for ::windows::core::IInspectable {
    fn from(value: &GameSaveBlobInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveBlobInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveBlobInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobInfo {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveBlobInfoGetResult(pub ::windows::core::IInspectable);
impl GameSaveBlobInfoGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobInfoGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult;{c7578582-3697-42bf-989c-665d923b5231})");
}
unsafe impl ::windows::core::Interface for GameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7578582_3697_42bf_989c_665d923b5231);
}
impl ::windows::core::RuntimeName for GameSaveBlobInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult";
}
impl ::core::convert::From<GameSaveBlobInfoGetResult> for ::windows::core::IUnknown {
    fn from(value: GameSaveBlobInfoGetResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveBlobInfoGetResult> for ::windows::core::IUnknown {
    fn from(value: &GameSaveBlobInfoGetResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveBlobInfoGetResult> for ::windows::core::IInspectable {
    fn from(value: GameSaveBlobInfoGetResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveBlobInfoGetResult> for ::windows::core::IInspectable {
    fn from(value: &GameSaveBlobInfoGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveBlobInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobInfoGetResult {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfoGetResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveBlobInfoQuery(pub ::windows::core::IInspectable);
impl GameSaveBlobInfoQuery {
    #[cfg(feature = "Foundation")]
    pub fn GetBlobInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetBlobInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), startindex, maxnumberofitems, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveBlobInfoQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery;{9fdd74b2-eeee-447b-a9d2-7f96c0f83208})");
}
unsafe impl ::windows::core::Interface for GameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fdd74b2_eeee_447b_a9d2_7f96c0f83208);
}
impl ::windows::core::RuntimeName for GameSaveBlobInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery";
}
impl ::core::convert::From<GameSaveBlobInfoQuery> for ::windows::core::IUnknown {
    fn from(value: GameSaveBlobInfoQuery) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveBlobInfoQuery> for ::windows::core::IUnknown {
    fn from(value: &GameSaveBlobInfoQuery) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveBlobInfoQuery> for ::windows::core::IInspectable {
    fn from(value: GameSaveBlobInfoQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveBlobInfoQuery> for ::windows::core::IInspectable {
    fn from(value: &GameSaveBlobInfoQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveBlobInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveBlobInfoQuery {}
unsafe impl ::core::marker::Sync for GameSaveBlobInfoQuery {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveContainer(pub ::windows::core::IInspectable);
impl GameSaveContainer {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Provider(&self) -> ::windows::core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveProvider>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SubmitUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        blobstowrite: Param0,
        blobstodelete: Param1,
        displayname: Param2,
    ) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), blobstowrite.into_param().abi(), blobstodelete.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>>(&self, blobstoread: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), blobstoread.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, blobstoread: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), blobstoread.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SubmitPropertySetUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IPropertySet>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        blobstowrite: Param0,
        blobstodelete: Param1,
        displayname: Param2,
    ) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), blobstowrite.into_param().abi(), blobstodelete.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    pub fn CreateBlobInfoQuery<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, blobnameprefix: Param0) -> ::windows::core::Result<GameSaveBlobInfoQuery> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), blobnameprefix.into_param().abi(), &mut result__).from_abi::<GameSaveBlobInfoQuery>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainer;{c3c08f89-563f-4ecd-9c6f-33fd0e323d10})");
}
unsafe impl ::windows::core::Interface for GameSaveContainer {
    type Vtable = IGameSaveContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c08f89_563f_4ecd_9c6f_33fd0e323d10);
}
impl ::windows::core::RuntimeName for GameSaveContainer {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainer";
}
impl ::core::convert::From<GameSaveContainer> for ::windows::core::IUnknown {
    fn from(value: GameSaveContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveContainer> for ::windows::core::IUnknown {
    fn from(value: &GameSaveContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveContainer> for ::windows::core::IInspectable {
    fn from(value: GameSaveContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveContainer> for ::windows::core::IInspectable {
    fn from(value: &GameSaveContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveContainer {}
unsafe impl ::core::marker::Sync for GameSaveContainer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveContainerInfo(pub ::windows::core::IInspectable);
impl GameSaveContainerInfo {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TotalSize(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LastModifiedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn NeedsSync(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo;{b7e27300-155d-4bb4-b2ba-930306f391b5})");
}
unsafe impl ::windows::core::Interface for GameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7e27300_155d_4bb4_b2ba_930306f391b5);
}
impl ::windows::core::RuntimeName for GameSaveContainerInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo";
}
impl ::core::convert::From<GameSaveContainerInfo> for ::windows::core::IUnknown {
    fn from(value: GameSaveContainerInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveContainerInfo> for ::windows::core::IUnknown {
    fn from(value: &GameSaveContainerInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveContainerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveContainerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveContainerInfo> for ::windows::core::IInspectable {
    fn from(value: GameSaveContainerInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveContainerInfo> for ::windows::core::IInspectable {
    fn from(value: &GameSaveContainerInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveContainerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveContainerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveContainerInfo {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveContainerInfoGetResult(pub ::windows::core::IInspectable);
impl GameSaveContainerInfoGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainerInfoGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult;{ffc50d74-c581-4f9d-9e39-30a10c1e4c50})");
}
unsafe impl ::windows::core::Interface for GameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffc50d74_c581_4f9d_9e39_30a10c1e4c50);
}
impl ::windows::core::RuntimeName for GameSaveContainerInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult";
}
impl ::core::convert::From<GameSaveContainerInfoGetResult> for ::windows::core::IUnknown {
    fn from(value: GameSaveContainerInfoGetResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveContainerInfoGetResult> for ::windows::core::IUnknown {
    fn from(value: &GameSaveContainerInfoGetResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveContainerInfoGetResult> for ::windows::core::IInspectable {
    fn from(value: GameSaveContainerInfoGetResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveContainerInfoGetResult> for ::windows::core::IInspectable {
    fn from(value: &GameSaveContainerInfoGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveContainerInfoGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveContainerInfoGetResult {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfoGetResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveContainerInfoQuery(pub ::windows::core::IInspectable);
impl GameSaveContainerInfoQuery {
    #[cfg(feature = "Foundation")]
    pub fn GetContainerInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetContainerInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), startindex, maxnumberofitems, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveContainerInfoQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery;{3c94e863-6f80-4327-9327-ffc11afd42b3})");
}
unsafe impl ::windows::core::Interface for GameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c94e863_6f80_4327_9327_ffc11afd42b3);
}
impl ::windows::core::RuntimeName for GameSaveContainerInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery";
}
impl ::core::convert::From<GameSaveContainerInfoQuery> for ::windows::core::IUnknown {
    fn from(value: GameSaveContainerInfoQuery) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveContainerInfoQuery> for ::windows::core::IUnknown {
    fn from(value: &GameSaveContainerInfoQuery) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveContainerInfoQuery> for ::windows::core::IInspectable {
    fn from(value: GameSaveContainerInfoQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveContainerInfoQuery> for ::windows::core::IInspectable {
    fn from(value: &GameSaveContainerInfoQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveContainerInfoQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveContainerInfoQuery {}
unsafe impl ::core::marker::Sync for GameSaveContainerInfoQuery {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for GameSaveErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameSaveErrorStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameSaveErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.XboxLive.Storage.GameSaveErrorStatus;i4)");
}
impl ::windows::core::DefaultType for GameSaveErrorStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveOperationResult(pub ::windows::core::IInspectable);
impl GameSaveOperationResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveOperationResult;{cf0f1a05-24a0-4582-9a55-b1bbbb9388d8})");
}
unsafe impl ::windows::core::Interface for GameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf0f1a05_24a0_4582_9a55_b1bbbb9388d8);
}
impl ::windows::core::RuntimeName for GameSaveOperationResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveOperationResult";
}
impl ::core::convert::From<GameSaveOperationResult> for ::windows::core::IUnknown {
    fn from(value: GameSaveOperationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveOperationResult> for ::windows::core::IUnknown {
    fn from(value: &GameSaveOperationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveOperationResult> for ::windows::core::IInspectable {
    fn from(value: GameSaveOperationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveOperationResult> for ::windows::core::IInspectable {
    fn from(value: &GameSaveOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveOperationResult {}
unsafe impl ::core::marker::Sync for GameSaveOperationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveProvider(pub ::windows::core::IInspectable);
impl GameSaveProvider {
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    pub fn CreateContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<GameSaveContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<GameSaveContainer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteContainerAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>(result__)
        }
    }
    pub fn CreateContainerInfoQuery(&self) -> ::windows::core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveContainerInfoQuery>(result__)
        }
    }
    pub fn CreateContainerInfoQueryWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, containernameprefix: Param0) -> ::windows::core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), containernameprefix.into_param().abi(), &mut result__).from_abi::<GameSaveContainerInfoQuery>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRemainingBytesInQuotaAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContainersChangedSinceLastSync(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, serviceconfigid: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), serviceconfigid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetSyncOnDemandForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, serviceconfigid: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>> {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), serviceconfigid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>(result__)
        })
    }
    pub fn IGameSaveProviderStatics<R, F: FnOnce(&IGameSaveProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameSaveProvider, IGameSaveProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProvider;{90a60394-80fe-4211-97f8-a5de14dd95d2})");
}
unsafe impl ::windows::core::Interface for GameSaveProvider {
    type Vtable = IGameSaveProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90a60394_80fe_4211_97f8_a5de14dd95d2);
}
impl ::windows::core::RuntimeName for GameSaveProvider {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProvider";
}
impl ::core::convert::From<GameSaveProvider> for ::windows::core::IUnknown {
    fn from(value: GameSaveProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveProvider> for ::windows::core::IUnknown {
    fn from(value: &GameSaveProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveProvider> for ::windows::core::IInspectable {
    fn from(value: GameSaveProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveProvider> for ::windows::core::IInspectable {
    fn from(value: &GameSaveProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveProvider {}
unsafe impl ::core::marker::Sync for GameSaveProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameSaveProviderGetResult(pub ::windows::core::IInspectable);
impl GameSaveProviderGetResult {
    pub fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: GameSaveErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveErrorStatus>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameSaveProvider>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameSaveProviderGetResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult;{3ab90816-d393-4d65-ac16-41c3e67ab945})");
}
unsafe impl ::windows::core::Interface for GameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ab90816_d393_4d65_ac16_41c3e67ab945);
}
impl ::windows::core::RuntimeName for GameSaveProviderGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult";
}
impl ::core::convert::From<GameSaveProviderGetResult> for ::windows::core::IUnknown {
    fn from(value: GameSaveProviderGetResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameSaveProviderGetResult> for ::windows::core::IUnknown {
    fn from(value: &GameSaveProviderGetResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameSaveProviderGetResult> for ::windows::core::IInspectable {
    fn from(value: GameSaveProviderGetResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameSaveProviderGetResult> for ::windows::core::IInspectable {
    fn from(value: &GameSaveProviderGetResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameSaveProviderGetResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameSaveProviderGetResult {}
unsafe impl ::core::marker::Sync for GameSaveProviderGetResult {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveBlobGetResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917281e0_7201_4953_aa2c_4008f03aef45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveBlobInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadd38034_baf0_4645_b6d0_46edaffb3c2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoGetResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7578582_3697_42bf_989c_665d923b5231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoQuery(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fdd74b2_eeee_447b_a9d2_7f96c0f83208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveBlobInfoQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveContainer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveContainer {
    type Vtable = IGameSaveContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c08f89_563f_4ecd_9c6f_33fd0e323d10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blobstowrite: ::windows::core::RawPtr, blobstodelete: ::windows::core::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blobstoread: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blobstoread: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blobstowrite: ::windows::core::RawPtr, blobstodelete: ::windows::core::RawPtr, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blobnameprefix: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveContainerInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7e27300_155d_4bb4_b2ba_930306f391b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoGetResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffc50d74_c581_4f9d_9e39_30a10c1e4c50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoQuery(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c94e863_6f80_4327_9327_ffc11afd42b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveContainerInfoQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveOperationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf0f1a05_24a0_4582_9a55_b1bbbb9388d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveProvider {
    type Vtable = IGameSaveProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90a60394_80fe_4211_97f8_a5de14dd95d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, containernameprefix: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveProviderGetResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ab90816_d393_4d65_ac16_41c3e67ab945);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderGetResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameSaveErrorStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameSaveProviderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameSaveProviderStatics {
    type Vtable = IGameSaveProviderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd01d3ed0_7b03_449d_8cbd_3402842a1048);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameSaveProviderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, serviceconfigid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, serviceconfigid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
