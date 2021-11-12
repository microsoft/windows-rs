#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Buffer(pub ::windows::core::IInspectable);
impl Buffer {
    pub fn Capacity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Create(capacity: u32) -> ::windows::core::Result<Buffer> {
        Self::IBufferFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capacity, &mut result__).from_abi::<Buffer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateCopyFromMemoryBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer>>(input: Param0) -> ::windows::core::Result<Buffer> {
        Self::IBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<Buffer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateMemoryBufferOverIBuffer<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(input: Param0) -> ::windows::core::Result<super::super::Foundation::MemoryBuffer> {
        Self::IBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::MemoryBuffer>(result__)
        })
    }
    pub fn IBufferFactory<R, F: FnOnce(&IBufferFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Buffer, IBufferFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBufferStatics<R, F: FnOnce(&IBufferStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Buffer, IBufferStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Buffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.Buffer;{905a0fe0-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for Buffer {
    type Vtable = IBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe0_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for Buffer {
    const NAME: &'static str = "Windows.Storage.Streams.Buffer";
}
impl ::core::convert::From<Buffer> for ::windows::core::IUnknown {
    fn from(value: Buffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Buffer> for ::windows::core::IUnknown {
    fn from(value: &Buffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Buffer> for ::windows::core::IInspectable {
    fn from(value: Buffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Buffer> for ::windows::core::IInspectable {
    fn from(value: &Buffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Buffer> for IBuffer {
    fn from(value: Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Buffer> for IBuffer {
    fn from(value: &Buffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBuffer> for Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, IBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBuffer> for &Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, IBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Buffer {}
unsafe impl ::core::marker::Sync for Buffer {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: ByteOrder = ByteOrder(0i32);
    pub const BigEndian: ByteOrder = ByteOrder(1i32);
}
impl ::core::convert::From<i32> for ByteOrder {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ByteOrder {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ByteOrder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.ByteOrder;i4)");
}
impl ::windows::core::DefaultType for ByteOrder {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DataReader(pub ::windows::core::IInspectable);
impl DataReader {
    pub fn UnconsumedBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> ::windows::core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__: InputStreamOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InputStreamOptions>(result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReadByte(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute_copy(&value)).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    pub fn ReadBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ReadGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ReadInt16(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    pub fn ReadInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn ReadInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn ReadUInt16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ReadUInt32(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ReadUInt64(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn ReadSingle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn ReadDouble(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), codeunitcount, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadDateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadTimeSpan(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoadAsync(&self, count: u32) -> ::windows::core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<DataReaderLoadOperation>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn CreateDataReader<'a, Param0: ::windows::core::IntoParam<'a, IInputStream>>(inputstream: Param0) -> ::windows::core::Result<DataReader> {
        Self::IDataReaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<DataReader>(result__)
        })
    }
    pub fn FromBuffer<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(buffer: Param0) -> ::windows::core::Result<DataReader> {
        Self::IDataReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<DataReader>(result__)
        })
    }
    pub fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataReader, IDataReaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataReader, IDataReaderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DataReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReader;{e2b50029-b4c1-4314-a4b8-fb813a2f275e})");
}
unsafe impl ::windows::core::Interface for DataReader {
    type Vtable = IDataReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b50029_b4c1_4314_a4b8_fb813a2f275e);
}
impl ::windows::core::RuntimeName for DataReader {
    const NAME: &'static str = "Windows.Storage.Streams.DataReader";
}
impl ::core::convert::From<DataReader> for ::windows::core::IUnknown {
    fn from(value: DataReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DataReader> for ::windows::core::IUnknown {
    fn from(value: &DataReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DataReader> for ::windows::core::IInspectable {
    fn from(value: DataReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DataReader> for ::windows::core::IInspectable {
    fn from(value: &DataReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DataReader> for IDataReader {
    fn from(value: DataReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataReader> for IDataReader {
    fn from(value: &DataReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataReader> for DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, IDataReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataReader> for &DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, IDataReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DataReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &DataReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DataReader {}
unsafe impl ::core::marker::Sync for DataReader {}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DataReaderLoadOperation(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl DataReaderLoadOperation {
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::AsyncOperationCompletedHandler<u32>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn get(&self) -> ::windows::core::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for DataReaderLoadOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReaderLoadOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for DataReaderLoadOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_abi<u32>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::IAsyncOperation<u32> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for DataReaderLoadOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataReaderLoadOperation";
}
#[cfg(feature = "Foundation")]
#[cfg(feature = "std")]
impl ::std::future::Future for DataReaderLoadOperation {
    type Output = ::windows::core::Result<u32>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<DataReaderLoadOperation> for ::windows::core::IUnknown {
    fn from(value: DataReaderLoadOperation) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&DataReaderLoadOperation> for ::windows::core::IUnknown {
    fn from(value: &DataReaderLoadOperation) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<DataReaderLoadOperation> for ::windows::core::IInspectable {
    fn from(value: DataReaderLoadOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&DataReaderLoadOperation> for ::windows::core::IInspectable {
    fn from(value: &DataReaderLoadOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<DataReaderLoadOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: DataReaderLoadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&DataReaderLoadOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: &DataReaderLoadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for &DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataReaderLoadOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataReaderLoadOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncInfo> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &DataReaderLoadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::core::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for DataReaderLoadOperation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DataWriter(pub ::windows::core::IInspectable);
impl DataWriter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataWriter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn UnstoredBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn WriteBuffer<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    pub fn WriteBufferRange<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, start: u32, count: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), buffer.into_param().abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteDateTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MeasureString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StoreAsync(&self) -> ::windows::core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataWriterStoreOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn CreateDataWriter<'a, Param0: ::windows::core::IntoParam<'a, IOutputStream>>(outputstream: Param0) -> ::windows::core::Result<DataWriter> {
        Self::IDataWriterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<DataWriter>(result__)
        })
    }
    pub fn IDataWriterFactory<R, F: FnOnce(&IDataWriterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataWriter, IDataWriterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DataWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriter;{64b89265-d341-4922-b38a-dd4af8808c4e})");
}
unsafe impl ::windows::core::Interface for DataWriter {
    type Vtable = IDataWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64b89265_d341_4922_b38a_dd4af8808c4e);
}
impl ::windows::core::RuntimeName for DataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriter";
}
impl ::core::convert::From<DataWriter> for ::windows::core::IUnknown {
    fn from(value: DataWriter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DataWriter> for ::windows::core::IUnknown {
    fn from(value: &DataWriter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DataWriter> for ::windows::core::IInspectable {
    fn from(value: DataWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DataWriter> for ::windows::core::IInspectable {
    fn from(value: &DataWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DataWriter> for IDataWriter {
    fn from(value: DataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataWriter> for IDataWriter {
    fn from(value: &DataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataWriter> for DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IDataWriter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataWriter> for &DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IDataWriter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DataWriter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &DataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DataWriter {}
unsafe impl ::core::marker::Sync for DataWriter {}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DataWriterStoreOperation(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl DataWriterStoreOperation {
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::AsyncOperationCompletedHandler<u32>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn get(&self) -> ::windows::core::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for DataWriterStoreOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriterStoreOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for DataWriterStoreOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_abi<u32>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::IAsyncOperation<u32> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for DataWriterStoreOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriterStoreOperation";
}
#[cfg(feature = "Foundation")]
#[cfg(feature = "std")]
impl ::std::future::Future for DataWriterStoreOperation {
    type Output = ::windows::core::Result<u32>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<DataWriterStoreOperation> for ::windows::core::IUnknown {
    fn from(value: DataWriterStoreOperation) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&DataWriterStoreOperation> for ::windows::core::IUnknown {
    fn from(value: &DataWriterStoreOperation) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<DataWriterStoreOperation> for ::windows::core::IInspectable {
    fn from(value: DataWriterStoreOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&DataWriterStoreOperation> for ::windows::core::IInspectable {
    fn from(value: &DataWriterStoreOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<DataWriterStoreOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: DataWriterStoreOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&DataWriterStoreOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: &DataWriterStoreOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for &DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataWriterStoreOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataWriterStoreOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncInfo> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &DataWriterStoreOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::core::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for DataWriterStoreOperation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileInputStream(pub ::windows::core::IInspectable);
impl FileInputStream {
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FileInputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileInputStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for FileInputStream {
    type Vtable = IInputStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe2_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for FileInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileInputStream";
}
impl ::core::convert::From<FileInputStream> for ::windows::core::IUnknown {
    fn from(value: FileInputStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileInputStream> for ::windows::core::IUnknown {
    fn from(value: &FileInputStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileInputStream> for ::windows::core::IInspectable {
    fn from(value: FileInputStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileInputStream> for ::windows::core::IInspectable {
    fn from(value: &FileInputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileInputStream> for IInputStream {
    fn from(value: FileInputStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInputStream> for IInputStream {
    fn from(value: &FileInputStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for &FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<FileInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&FileInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &FileInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileInputStream {}
unsafe impl ::core::marker::Sync for FileInputStream {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: FileOpenDisposition = FileOpenDisposition(0i32);
    pub const OpenAlways: FileOpenDisposition = FileOpenDisposition(1i32);
    pub const CreateNew: FileOpenDisposition = FileOpenDisposition(2i32);
    pub const CreateAlways: FileOpenDisposition = FileOpenDisposition(3i32);
    pub const TruncateExisting: FileOpenDisposition = FileOpenDisposition(4i32);
}
impl ::core::convert::From<i32> for FileOpenDisposition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FileOpenDisposition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FileOpenDisposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.FileOpenDisposition;i4)");
}
impl ::windows::core::DefaultType for FileOpenDisposition {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileOutputStream(pub ::windows::core::IInspectable);
impl FileOutputStream {
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FileOutputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileOutputStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for FileOutputStream {
    type Vtable = IOutputStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe6_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for FileOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileOutputStream";
}
impl ::core::convert::From<FileOutputStream> for ::windows::core::IUnknown {
    fn from(value: FileOutputStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileOutputStream> for ::windows::core::IUnknown {
    fn from(value: &FileOutputStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileOutputStream> for ::windows::core::IInspectable {
    fn from(value: FileOutputStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileOutputStream> for ::windows::core::IInspectable {
    fn from(value: &FileOutputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileOutputStream> for IOutputStream {
    fn from(value: FileOutputStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOutputStream> for IOutputStream {
    fn from(value: &FileOutputStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for &FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<FileOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOutputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&FileOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOutputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &FileOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOutputStream {}
unsafe impl ::core::marker::Sync for FileOutputStream {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileRandomAccessStream(pub ::windows::core::IInspectable);
impl FileRandomAccessStream {
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filepath: Param0, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), filepath.into_param().abi(), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filepath: Param0, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), filepath.into_param().abi(), accessmode, sharingoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filepath: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filepath: Param0, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), filepath.into_param().abi(), openoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, filepath: Param1, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenForUserWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, filepath: Param1, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), accessmode, sharingoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenTransactedWriteForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, filepath: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenTransactedWriteForUserWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, filepath: Param1, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), openoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    pub fn IFileRandomAccessStreamStatics<R, F: FnOnce(&IFileRandomAccessStreamStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileRandomAccessStream, IFileRandomAccessStreamStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FileRandomAccessStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for FileRandomAccessStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe1_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for FileRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileRandomAccessStream";
}
impl ::core::convert::From<FileRandomAccessStream> for ::windows::core::IUnknown {
    fn from(value: FileRandomAccessStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileRandomAccessStream> for ::windows::core::IUnknown {
    fn from(value: &FileRandomAccessStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileRandomAccessStream> for ::windows::core::IInspectable {
    fn from(value: FileRandomAccessStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileRandomAccessStream> for ::windows::core::IInspectable {
    fn from(value: &FileRandomAccessStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileRandomAccessStream> for IRandomAccessStream {
    fn from(value: FileRandomAccessStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileRandomAccessStream> for IRandomAccessStream {
    fn from(value: &FileRandomAccessStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<FileRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&FileRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::core::convert::TryInto::<IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::core::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileRandomAccessStream {}
unsafe impl ::core::marker::Sync for FileRandomAccessStream {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBuffer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBuffer {
    type Vtable = IBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe0_bc53_11df_8c49_001e4fc686da);
}
impl IBuffer {
    pub fn Capacity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe0-bc53-11df-8c49-001e4fc686da}");
}
impl ::core::convert::From<IBuffer> for ::windows::core::IUnknown {
    fn from(value: IBuffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBuffer> for ::windows::core::IUnknown {
    fn from(value: &IBuffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBuffer> for ::windows::core::IInspectable {
    fn from(value: IBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBuffer> for ::windows::core::IInspectable {
    fn from(value: &IBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBufferFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBufferFactory {
    type Vtable = IBufferFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71af914d_c10f_484b_bc50_14bc623b3a27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBufferStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBufferStatics {
    type Vtable = IBufferStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe901e65b_d716_475a_a90a_af7229b1e741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContentTypeProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContentTypeProvider {
    type Vtable = IContentTypeProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d098a5_3b99_4de9_88a5_e11d2f50c795);
}
impl IContentTypeProvider {
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContentTypeProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{97d098a5-3b99-4de9-88a5-e11d2f50c795}");
}
impl ::core::convert::From<IContentTypeProvider> for ::windows::core::IUnknown {
    fn from(value: IContentTypeProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContentTypeProvider> for ::windows::core::IUnknown {
    fn from(value: &IContentTypeProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContentTypeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContentTypeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContentTypeProvider> for ::windows::core::IInspectable {
    fn from(value: IContentTypeProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContentTypeProvider> for ::windows::core::IInspectable {
    fn from(value: &IContentTypeProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContentTypeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContentTypeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentTypeProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDataReader {
    type Vtable = IDataReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b50029_b4c1_4314_a4b8_fb813a2f275e);
}
impl IDataReader {
    pub fn UnconsumedBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> ::windows::core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__: InputStreamOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InputStreamOptions>(result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReadByte(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute_copy(&value)).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    pub fn ReadBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ReadGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ReadInt16(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    pub fn ReadInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn ReadInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn ReadUInt16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ReadUInt32(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ReadUInt64(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn ReadSingle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn ReadDouble(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), codeunitcount, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadDateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadTimeSpan(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoadAsync(&self, count: u32) -> ::windows::core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<DataReaderLoadOperation>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInputStream>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IDataReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e2b50029-b4c1-4314-a4b8-fb813a2f275e}");
}
impl ::core::convert::From<IDataReader> for ::windows::core::IUnknown {
    fn from(value: IDataReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDataReader> for ::windows::core::IUnknown {
    fn from(value: &IDataReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDataReader> for ::windows::core::IInspectable {
    fn from(value: IDataReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataReader> for ::windows::core::IInspectable {
    fn from(value: &IDataReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UnicodeEncoding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ByteOrder) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ByteOrder) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InputStreamOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InputStreamOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codeunitcount: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataReaderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDataReaderFactory {
    type Vtable = IDataReaderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7527847_57da_4e15_914c_06806699a098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataReaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDataReaderStatics {
    type Vtable = IDataReaderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11fcbfc8_f93a_471b_b121_f379e349313c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataWriter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDataWriter {
    type Vtable = IDataWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64b89265_d341_4922_b38a_dd4af8808c4e);
}
impl IDataWriter {
    pub fn UnstoredBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn WriteBuffer<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    pub fn WriteBufferRange<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, start: u32, count: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), buffer.into_param().abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteDateTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn WriteString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MeasureString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StoreAsync(&self) -> ::windows::core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DataWriterStoreOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IDataWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{64b89265-d341-4922-b38a-dd4af8808c4e}");
}
impl ::core::convert::From<IDataWriter> for ::windows::core::IUnknown {
    fn from(value: IDataWriter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDataWriter> for ::windows::core::IUnknown {
    fn from(value: &IDataWriter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDataWriter> for ::windows::core::IInspectable {
    fn from(value: IDataWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataWriter> for ::windows::core::IInspectable {
    fn from(value: &IDataWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UnicodeEncoding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ByteOrder) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ByteOrder) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, start: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataWriterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDataWriterFactory {
    type Vtable = IDataWriterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338c67c2_8b84_4c2b_9c50_7b8767847a1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileRandomAccessStreamStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileRandomAccessStreamStatics {
    type Vtable = IFileRandomAccessStreamStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73550107_3b57_4b5d_8345_554d2fc621f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRandomAccessStreamStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInputStream(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputStream {
    type Vtable = IInputStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe2_bc53_11df_8c49_001e4fc686da);
}
impl IInputStream {
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IInputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe2-bc53-11df-8c49-001e4fc686da}");
}
impl ::core::convert::From<IInputStream> for ::windows::core::IUnknown {
    fn from(value: IInputStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInputStream> for ::windows::core::IUnknown {
    fn from(value: &IInputStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInputStream> for ::windows::core::IInspectable {
    fn from(value: IInputStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInputStream> for ::windows::core::IInspectable {
    fn from(value: &IInputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IInputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IInputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IInputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, count: u32, options: InputStreamOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInputStreamReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputStreamReference {
    type Vtable = IInputStreamReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43929d18_5ec9_4b5a_919c_4205b0c804b6);
}
impl IInputStreamReference {
    #[cfg(feature = "Foundation")]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IInputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IInputStream>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInputStreamReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{43929d18-5ec9-4b5a-919c-4205b0c804b6}");
}
impl ::core::convert::From<IInputStreamReference> for ::windows::core::IUnknown {
    fn from(value: IInputStreamReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInputStreamReference> for ::windows::core::IUnknown {
    fn from(value: &IInputStreamReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInputStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInputStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInputStreamReference> for ::windows::core::IInspectable {
    fn from(value: IInputStreamReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInputStreamReference> for ::windows::core::IInspectable {
    fn from(value: &IInputStreamReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInputStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInputStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStreamReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOutputStream(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOutputStream {
    type Vtable = IOutputStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe6_bc53_11df_8c49_001e4fc686da);
}
impl IOutputStream {
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IOutputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe6-bc53-11df-8c49-001e4fc686da}");
}
impl ::core::convert::From<IOutputStream> for ::windows::core::IUnknown {
    fn from(value: IOutputStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IOutputStream> for ::windows::core::IUnknown {
    fn from(value: &IOutputStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IOutputStream> for ::windows::core::IInspectable {
    fn from(value: IOutputStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOutputStream> for ::windows::core::IInspectable {
    fn from(value: &IOutputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IOutputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IOutputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IOutputStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOutputStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertySetSerializer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPropertySetSerializer {
    type Vtable = IPropertySetSerializer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e8ebf1c_ef3d_4376_b20e_5be638aeac77);
}
impl IPropertySetSerializer {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Serialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, propertyset: Param0) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyset.into_param().abi(), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Deserialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>, Param1: ::windows::core::IntoParam<'a, IBuffer>>(&self, propertyset: Param0, buffer: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertyset.into_param().abi(), buffer.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IPropertySetSerializer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6e8ebf1c-ef3d-4376-b20e-5be638aeac77}");
}
impl ::core::convert::From<IPropertySetSerializer> for ::windows::core::IUnknown {
    fn from(value: IPropertySetSerializer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPropertySetSerializer> for ::windows::core::IUnknown {
    fn from(value: &IPropertySetSerializer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertySetSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertySetSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPropertySetSerializer> for ::windows::core::IInspectable {
    fn from(value: IPropertySetSerializer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertySetSerializer> for ::windows::core::IInspectable {
    fn from(value: &IPropertySetSerializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPropertySetSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPropertySetSerializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetSerializer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyset: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRandomAccessStream(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRandomAccessStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe1_bc53_11df_8c49_001e4fc686da);
}
impl IRandomAccessStream {
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IRandomAccessStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe1-bc53-11df-8c49-001e4fc686da}");
}
impl ::core::convert::From<IRandomAccessStream> for ::windows::core::IUnknown {
    fn from(value: IRandomAccessStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IRandomAccessStream> for ::windows::core::IUnknown {
    fn from(value: &IRandomAccessStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IRandomAccessStream> for ::windows::core::IInspectable {
    fn from(value: IRandomAccessStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRandomAccessStream> for ::windows::core::IInspectable {
    fn from(value: &IRandomAccessStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for &IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::core::convert::TryInto::<IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for &IRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::core::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRandomAccessStreamReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
}
impl IRandomAccessStreamReference {
    #[cfg(feature = "Foundation")]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IRandomAccessStreamReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{33ee3134-1dd6-4e3a-8067-d1c162e8642b}");
}
impl ::core::convert::From<IRandomAccessStreamReference> for ::windows::core::IUnknown {
    fn from(value: IRandomAccessStreamReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IRandomAccessStreamReference> for ::windows::core::IUnknown {
    fn from(value: &IRandomAccessStreamReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IRandomAccessStreamReference> for ::windows::core::IInspectable {
    fn from(value: IRandomAccessStreamReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRandomAccessStreamReference> for ::windows::core::IInspectable {
    fn from(value: &IRandomAccessStreamReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRandomAccessStreamReferenceStatics {
    type Vtable = IRandomAccessStreamReferenceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x857309dc_3fbf_4e7d_986f_ef3b1a07a964);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRandomAccessStreamStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRandomAccessStreamStatics {
    type Vtable = IRandomAccessStreamStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x524cedcf_6e29_4ce5_9573_6b753db66c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, bytestocopy: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRandomAccessStreamWithContentType(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRandomAccessStreamWithContentType {
    type Vtable = IRandomAccessStreamWithContentType_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc254827_4b3d_438f_9232_10c76bc7e038);
}
impl IRandomAccessStreamWithContentType {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IRandomAccessStreamWithContentType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cc254827-4b3d-438f-9232-10c76bc7e038}");
}
impl ::core::convert::From<IRandomAccessStreamWithContentType> for ::windows::core::IUnknown {
    fn from(value: IRandomAccessStreamWithContentType) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IRandomAccessStreamWithContentType> for ::windows::core::IUnknown {
    fn from(value: &IRandomAccessStreamWithContentType) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IRandomAccessStreamWithContentType> for ::windows::core::IInspectable {
    fn from(value: IRandomAccessStreamWithContentType) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRandomAccessStreamWithContentType> for ::windows::core::IInspectable {
    fn from(value: &IRandomAccessStreamWithContentType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentTypeProvider> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IContentTypeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentTypeProvider> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IContentTypeProvider> {
        ::core::convert::TryInto::<IContentTypeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::core::convert::TryInto::<IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::core::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::core::convert::TryInto::<IRandomAccessStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamWithContentType_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InMemoryRandomAccessStream(pub ::windows::core::IInspectable);
impl InMemoryRandomAccessStream {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InMemoryRandomAccessStream, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InMemoryRandomAccessStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InMemoryRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for InMemoryRandomAccessStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe1_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for InMemoryRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.InMemoryRandomAccessStream";
}
impl ::core::convert::From<InMemoryRandomAccessStream> for ::windows::core::IUnknown {
    fn from(value: InMemoryRandomAccessStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InMemoryRandomAccessStream> for ::windows::core::IUnknown {
    fn from(value: &InMemoryRandomAccessStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InMemoryRandomAccessStream> for ::windows::core::IInspectable {
    fn from(value: InMemoryRandomAccessStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InMemoryRandomAccessStream> for ::windows::core::IInspectable {
    fn from(value: &InMemoryRandomAccessStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InMemoryRandomAccessStream> for IRandomAccessStream {
    fn from(value: InMemoryRandomAccessStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InMemoryRandomAccessStream> for IRandomAccessStream {
    fn from(value: &InMemoryRandomAccessStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InMemoryRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InMemoryRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InMemoryRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InMemoryRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::core::convert::TryInto::<IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InMemoryRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InMemoryRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::core::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InMemoryRandomAccessStream {}
unsafe impl ::core::marker::Sync for InMemoryRandomAccessStream {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: InputStreamOptions = InputStreamOptions(0u32);
    pub const Partial: InputStreamOptions = InputStreamOptions(1u32);
    pub const ReadAhead: InputStreamOptions = InputStreamOptions(2u32);
}
impl ::core::convert::From<u32> for InputStreamOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InputStreamOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InputStreamOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.InputStreamOptions;u4)");
}
impl ::windows::core::DefaultType for InputStreamOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for InputStreamOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for InputStreamOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for InputStreamOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for InputStreamOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for InputStreamOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InputStreamOverStream(pub ::windows::core::IInspectable);
impl InputStreamOverStream {
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InputStreamOverStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InputStreamOverStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for InputStreamOverStream {
    type Vtable = IInputStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe2_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for InputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.InputStreamOverStream";
}
impl ::core::convert::From<InputStreamOverStream> for ::windows::core::IUnknown {
    fn from(value: InputStreamOverStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputStreamOverStream> for ::windows::core::IUnknown {
    fn from(value: &InputStreamOverStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputStreamOverStream> for ::windows::core::IInspectable {
    fn from(value: InputStreamOverStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputStreamOverStream> for ::windows::core::IInspectable {
    fn from(value: &InputStreamOverStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InputStreamOverStream> for IInputStream {
    fn from(value: InputStreamOverStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputStreamOverStream> for IInputStream {
    fn from(value: &InputStreamOverStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for &InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InputStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &InputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InputStreamOverStream {}
unsafe impl ::core::marker::Sync for InputStreamOverStream {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OutputStreamOverStream(pub ::windows::core::IInspectable);
impl OutputStreamOverStream {
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for OutputStreamOverStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.OutputStreamOverStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for OutputStreamOverStream {
    type Vtable = IOutputStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe6_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for OutputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.OutputStreamOverStream";
}
impl ::core::convert::From<OutputStreamOverStream> for ::windows::core::IUnknown {
    fn from(value: OutputStreamOverStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OutputStreamOverStream> for ::windows::core::IUnknown {
    fn from(value: &OutputStreamOverStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OutputStreamOverStream> for ::windows::core::IInspectable {
    fn from(value: OutputStreamOverStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OutputStreamOverStream> for ::windows::core::IInspectable {
    fn from(value: &OutputStreamOverStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<OutputStreamOverStream> for IOutputStream {
    fn from(value: OutputStreamOverStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OutputStreamOverStream> for IOutputStream {
    fn from(value: &OutputStreamOverStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for &OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<OutputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: OutputStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&OutputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &OutputStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &OutputStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for OutputStreamOverStream {}
unsafe impl ::core::marker::Sync for OutputStreamOverStream {}
pub struct RandomAccessStream {}
impl RandomAccessStream {
    #[cfg(feature = "Foundation")]
    pub fn CopyAsync<'a, Param0: ::windows::core::IntoParam<'a, IInputStream>, Param1: ::windows::core::IntoParam<'a, IOutputStream>>(source: Param0, destination: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CopySizeAsync<'a, Param0: ::windows::core::IntoParam<'a, IInputStream>, Param1: ::windows::core::IntoParam<'a, IOutputStream>>(source: Param0, destination: Param1, bytestocopy: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), bytestocopy, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CopyAndCloseAsync<'a, Param0: ::windows::core::IntoParam<'a, IInputStream>, Param1: ::windows::core::IntoParam<'a, IOutputStream>>(source: Param0, destination: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    pub fn IRandomAccessStreamStatics<R, F: FnOnce(&IRandomAccessStreamStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RandomAccessStream, IRandomAccessStreamStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for RandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStream";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RandomAccessStreamOverStream(pub ::windows::core::IInspectable);
impl RandomAccessStreamOverStream {
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RandomAccessStreamOverStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamOverStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::core::Interface for RandomAccessStreamOverStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe1_bc53_11df_8c49_001e4fc686da);
}
impl ::windows::core::RuntimeName for RandomAccessStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamOverStream";
}
impl ::core::convert::From<RandomAccessStreamOverStream> for ::windows::core::IUnknown {
    fn from(value: RandomAccessStreamOverStream) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RandomAccessStreamOverStream> for ::windows::core::IUnknown {
    fn from(value: &RandomAccessStreamOverStream) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RandomAccessStreamOverStream> for ::windows::core::IInspectable {
    fn from(value: RandomAccessStreamOverStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RandomAccessStreamOverStream> for ::windows::core::IInspectable {
    fn from(value: &RandomAccessStreamOverStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RandomAccessStreamOverStream> for IRandomAccessStream {
    fn from(value: RandomAccessStreamOverStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RandomAccessStreamOverStream> for IRandomAccessStream {
    fn from(value: &RandomAccessStreamOverStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStream> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<RandomAccessStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RandomAccessStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<RandomAccessStreamOverStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RandomAccessStreamOverStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInputStream> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IInputStream> {
        ::core::convert::TryInto::<IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<RandomAccessStreamOverStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RandomAccessStreamOverStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOutputStream> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::core::Param<'a, IOutputStream> {
        ::core::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RandomAccessStreamOverStream {}
unsafe impl ::core::marker::Sync for RandomAccessStreamOverStream {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RandomAccessStreamReference(pub ::windows::core::IInspectable);
impl RandomAccessStreamReference {
    #[cfg(feature = "Foundation")]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>(result__)
        }
    }
    pub fn CreateFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>>(file: Param0) -> ::windows::core::Result<RandomAccessStreamReference> {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<RandomAccessStreamReference> {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    pub fn CreateFromStream<'a, Param0: ::windows::core::IntoParam<'a, IRandomAccessStream>>(stream: Param0) -> ::windows::core::Result<RandomAccessStreamReference> {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    pub fn IRandomAccessStreamReferenceStatics<R, F: FnOnce(&IRandomAccessStreamReferenceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RandomAccessStreamReference, IRandomAccessStreamReferenceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RandomAccessStreamReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamReference;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
}
unsafe impl ::windows::core::Interface for RandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
}
impl ::windows::core::RuntimeName for RandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
impl ::core::convert::From<RandomAccessStreamReference> for ::windows::core::IUnknown {
    fn from(value: RandomAccessStreamReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RandomAccessStreamReference> for ::windows::core::IUnknown {
    fn from(value: &RandomAccessStreamReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RandomAccessStreamReference> for ::windows::core::IInspectable {
    fn from(value: RandomAccessStreamReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RandomAccessStreamReference> for ::windows::core::IInspectable {
    fn from(value: &RandomAccessStreamReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RandomAccessStreamReference> for IRandomAccessStreamReference {
    fn from(value: RandomAccessStreamReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RandomAccessStreamReference> for IRandomAccessStreamReference {
    fn from(value: &RandomAccessStreamReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStreamReference> for RandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStreamReference> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRandomAccessStreamReference> for &RandomAccessStreamReference {
    fn into_param(self) -> ::windows::core::Param<'a, IRandomAccessStreamReference> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RandomAccessStreamReference {}
unsafe impl ::core::marker::Sync for RandomAccessStreamReference {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: UnicodeEncoding = UnicodeEncoding(0i32);
    pub const Utf16LE: UnicodeEncoding = UnicodeEncoding(1i32);
    pub const Utf16BE: UnicodeEncoding = UnicodeEncoding(2i32);
}
impl ::core::convert::From<i32> for UnicodeEncoding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnicodeEncoding {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnicodeEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.UnicodeEncoding;i4)");
}
impl ::windows::core::DefaultType for UnicodeEncoding {
    type DefaultType = Self;
}
