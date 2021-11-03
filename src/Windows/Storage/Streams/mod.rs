#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Buffer(::windows::runtime::IInspectable);
impl Buffer {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Capacity(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetLength(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Create(capacity: u32) -> ::windows::runtime::Result<Buffer> {
        Self::IBufferFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), capacity, &mut result__).from_abi::<Buffer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn CreateCopyFromMemoryBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IMemoryBuffer>>(input: Param0) -> ::windows::runtime::Result<Buffer> {
        Self::IBufferStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<Buffer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn CreateMemoryBufferOverIBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(input: Param0) -> ::windows::runtime::Result<super::super::Foundation::MemoryBuffer> {
        Self::IBufferStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::MemoryBuffer>(result__)
        })
    }
    pub fn IBufferFactory<R, F: FnOnce(&IBufferFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Buffer, IBufferFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBufferStatics<R, F: FnOnce(&IBufferStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Buffer, IBufferStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Buffer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.Buffer;{905a0fe0-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for Buffer {
    type Vtable = IBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821408, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for Buffer {
    const NAME: &'static str = "Windows.Storage.Streams.Buffer";
}
impl ::std::convert::From<Buffer> for ::windows::runtime::IUnknown {
    fn from(value: Buffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Buffer> for ::windows::runtime::IUnknown {
    fn from(value: &Buffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Buffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Buffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Buffer> for ::windows::runtime::IInspectable {
    fn from(value: Buffer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Buffer> for ::windows::runtime::IInspectable {
    fn from(value: &Buffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Buffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Buffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Buffer> for IBuffer {
    fn from(value: Buffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Buffer> for IBuffer {
    fn from(value: &Buffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBuffer> for Buffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBuffer> for &Buffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBuffer>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for Buffer {}
unsafe impl ::std::marker::Sync for Buffer {}
#[doc = "*Required features: `Storage_Streams`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: ByteOrder = ByteOrder(0i32);
    pub const BigEndian: ByteOrder = ByteOrder(1i32);
}
impl ::std::convert::From<i32> for ByteOrder {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ByteOrder {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ByteOrder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.ByteOrder;i4)");
}
impl ::windows::runtime::DefaultType for ByteOrder {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataReader(::windows::runtime::IInspectable);
impl DataReader {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnconsumedBufferLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnicodeEncoding(&self) -> ::windows::runtime::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ByteOrder(&self) -> ::windows::runtime::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn InputStreamOptions(&self) -> ::windows::runtime::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__: InputStreamOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InputStreamOptions>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadByte(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadBytes(&self, value: &mut [<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute_copy(&value)).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadBuffer(&self, length: u32) -> ::windows::runtime::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), length, &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadBoolean(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadInt16(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadInt32(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadInt64(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadUInt16(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadUInt32(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadUInt64(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadSingle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadDouble(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), codeunitcount, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadDateTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadTimeSpan(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn LoadAsync(&self, count: u32) -> ::windows::runtime::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), count, &mut result__).from_abi::<DataReaderLoadOperation>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachBuffer(&self) -> ::windows::runtime::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachStream(&self) -> ::windows::runtime::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CreateDataReader<'a, Param0: ::windows::runtime::IntoParam<'a, IInputStream>>(inputstream: Param0) -> ::windows::runtime::Result<DataReader> {
        Self::IDataReaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<DataReader>(result__)
        })
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn FromBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(buffer: Param0) -> ::windows::runtime::Result<DataReader> {
        Self::IDataReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<DataReader>(result__)
        })
    }
    pub fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataReader, IDataReaderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataReader, IDataReaderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReader;{e2b50029-b4c1-4314-a4b8-fb813a2f275e})");
}
unsafe impl ::windows::runtime::Interface for DataReader {
    type Vtable = IDataReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3803512873, 46273, 17172, [164, 184, 251, 129, 58, 47, 39, 94]);
}
impl ::windows::runtime::RuntimeName for DataReader {
    const NAME: &'static str = "Windows.Storage.Streams.DataReader";
}
impl ::std::convert::From<DataReader> for ::windows::runtime::IUnknown {
    fn from(value: DataReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataReader> for ::windows::runtime::IUnknown {
    fn from(value: &DataReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataReader> for ::windows::runtime::IInspectable {
    fn from(value: DataReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataReader> for ::windows::runtime::IInspectable {
    fn from(value: &DataReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<DataReader> for IDataReader {
    fn from(value: DataReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataReader> for IDataReader {
    fn from(value: &DataReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDataReader> for DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDataReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDataReader>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDataReader> for &DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDataReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDataReader>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DataReader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataReader) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DataReader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataReader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &DataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DataReader {}
unsafe impl ::std::marker::Sync for DataReader {}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataReaderLoadOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl DataReaderLoadOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncOperationCompletedHandler<u32>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
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
unsafe impl ::windows::runtime::RuntimeType for DataReaderLoadOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReaderLoadOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for DataReaderLoadOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_abi<u32>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::Foundation::IAsyncOperation<u32> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for DataReaderLoadOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataReaderLoadOperation";
}
impl ::std::future::Future for DataReaderLoadOperation {
    type Output = ::windows::runtime::Result<u32>;
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
impl ::std::convert::From<DataReaderLoadOperation> for ::windows::runtime::IUnknown {
    fn from(value: DataReaderLoadOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DataReaderLoadOperation> for ::windows::runtime::IUnknown {
    fn from(value: &DataReaderLoadOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DataReaderLoadOperation> for ::windows::runtime::IInspectable {
    fn from(value: DataReaderLoadOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DataReaderLoadOperation> for ::windows::runtime::IInspectable {
    fn from(value: &DataReaderLoadOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DataReaderLoadOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: DataReaderLoadOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DataReaderLoadOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: &DataReaderLoadOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<u32>>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for &DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<u32>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DataReaderLoadOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataReaderLoadOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DataReaderLoadOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataReaderLoadOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &DataReaderLoadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Send for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Sync for DataReaderLoadOperation {}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataWriter(::windows::runtime::IInspectable);
impl DataWriter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataWriter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnstoredBufferLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnicodeEncoding(&self) -> ::windows::runtime::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ByteOrder(&self) -> ::windows::runtime::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteByte(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBytes(&self, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBufferRange<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, start: u32, count: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), buffer.into_param().abi(), start, count).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBoolean(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteGuid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteInt16(&self, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteInt32(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteInt64(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteUInt16(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteUInt32(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteUInt64(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteSingle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteDouble(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn MeasureString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn StoreAsync(&self) -> ::windows::runtime::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataWriterStoreOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachBuffer(&self) -> ::windows::runtime::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachStream(&self) -> ::windows::runtime::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CreateDataWriter<'a, Param0: ::windows::runtime::IntoParam<'a, IOutputStream>>(outputstream: Param0) -> ::windows::runtime::Result<DataWriter> {
        Self::IDataWriterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<DataWriter>(result__)
        })
    }
    pub fn IDataWriterFactory<R, F: FnOnce(&IDataWriterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataWriter, IDataWriterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataWriter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriter;{64b89265-d341-4922-b38a-dd4af8808c4e})");
}
unsafe impl ::windows::runtime::Interface for DataWriter {
    type Vtable = IDataWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1689817701, 54081, 18722, [179, 138, 221, 74, 248, 128, 140, 78]);
}
impl ::windows::runtime::RuntimeName for DataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriter";
}
impl ::std::convert::From<DataWriter> for ::windows::runtime::IUnknown {
    fn from(value: DataWriter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataWriter> for ::windows::runtime::IUnknown {
    fn from(value: &DataWriter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DataWriter> for ::windows::runtime::IInspectable {
    fn from(value: DataWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataWriter> for ::windows::runtime::IInspectable {
    fn from(value: &DataWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<DataWriter> for IDataWriter {
    fn from(value: DataWriter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DataWriter> for IDataWriter {
    fn from(value: &DataWriter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDataWriter> for DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDataWriter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDataWriter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDataWriter> for &DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDataWriter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDataWriter>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataWriter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataWriter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &DataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DataWriter {}
unsafe impl ::std::marker::Sync for DataWriter {}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataWriterStoreOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl DataWriterStoreOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncOperationCompletedHandler<u32>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
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
unsafe impl ::windows::runtime::RuntimeType for DataWriterStoreOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriterStoreOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for DataWriterStoreOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_abi<u32>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::Foundation::IAsyncOperation<u32> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for DataWriterStoreOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriterStoreOperation";
}
impl ::std::future::Future for DataWriterStoreOperation {
    type Output = ::windows::runtime::Result<u32>;
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
impl ::std::convert::From<DataWriterStoreOperation> for ::windows::runtime::IUnknown {
    fn from(value: DataWriterStoreOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DataWriterStoreOperation> for ::windows::runtime::IUnknown {
    fn from(value: &DataWriterStoreOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DataWriterStoreOperation> for ::windows::runtime::IInspectable {
    fn from(value: DataWriterStoreOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DataWriterStoreOperation> for ::windows::runtime::IInspectable {
    fn from(value: &DataWriterStoreOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DataWriterStoreOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: DataWriterStoreOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DataWriterStoreOperation> for super::super::Foundation::IAsyncOperation<u32> {
    fn from(value: &DataWriterStoreOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<u32>>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<u32>> for &DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<u32>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<u32>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DataWriterStoreOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DataWriterStoreOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DataWriterStoreOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DataWriterStoreOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &DataWriterStoreOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Send for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Sync for DataWriterStoreOperation {}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FileInputStream(::windows::runtime::IInspectable);
impl FileInputStream {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileInputStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileInputStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for FileInputStream {
    type Vtable = IInputStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821410, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for FileInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileInputStream";
}
impl ::std::convert::From<FileInputStream> for ::windows::runtime::IUnknown {
    fn from(value: FileInputStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileInputStream> for ::windows::runtime::IUnknown {
    fn from(value: &FileInputStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FileInputStream> for ::windows::runtime::IInspectable {
    fn from(value: FileInputStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileInputStream> for ::windows::runtime::IInspectable {
    fn from(value: &FileInputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<FileInputStream> for IInputStream {
    fn from(value: FileInputStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileInputStream> for IInputStream {
    fn from(value: &FileInputStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInputStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for &FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInputStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<FileInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileInputStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&FileInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileInputStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &FileInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for FileInputStream {}
unsafe impl ::std::marker::Sync for FileInputStream {}
#[doc = "*Required features: `Storage_Streams`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: FileOpenDisposition = FileOpenDisposition(0i32);
    pub const OpenAlways: FileOpenDisposition = FileOpenDisposition(1i32);
    pub const CreateNew: FileOpenDisposition = FileOpenDisposition(2i32);
    pub const CreateAlways: FileOpenDisposition = FileOpenDisposition(3i32);
    pub const TruncateExisting: FileOpenDisposition = FileOpenDisposition(4i32);
}
impl ::std::convert::From<i32> for FileOpenDisposition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FileOpenDisposition {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FileOpenDisposition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.FileOpenDisposition;i4)");
}
impl ::windows::runtime::DefaultType for FileOpenDisposition {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FileOutputStream(::windows::runtime::IInspectable);
impl FileOutputStream {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileOutputStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileOutputStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for FileOutputStream {
    type Vtable = IOutputStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821414, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for FileOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileOutputStream";
}
impl ::std::convert::From<FileOutputStream> for ::windows::runtime::IUnknown {
    fn from(value: FileOutputStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileOutputStream> for ::windows::runtime::IUnknown {
    fn from(value: &FileOutputStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FileOutputStream> for ::windows::runtime::IInspectable {
    fn from(value: FileOutputStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileOutputStream> for ::windows::runtime::IInspectable {
    fn from(value: &FileOutputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<FileOutputStream> for IOutputStream {
    fn from(value: FileOutputStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileOutputStream> for IOutputStream {
    fn from(value: &FileOutputStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IOutputStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for &FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IOutputStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<FileOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileOutputStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&FileOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileOutputStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &FileOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for FileOutputStream {}
unsafe impl ::std::marker::Sync for FileOutputStream {}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FileRandomAccessStream(::windows::runtime::IInspectable);
impl FileRandomAccessStream {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn OpenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(filepath: Param0, accessmode: super::FileAccessMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), filepath.into_param().abi(), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn OpenWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(filepath: Param0, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), filepath.into_param().abi(), accessmode, sharingoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn OpenTransactedWriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(filepath: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn OpenTransactedWriteWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(filepath: Param0, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), filepath.into_param().abi(), openoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`, `System`*"]
    pub fn OpenForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, filepath: Param1, accessmode: super::FileAccessMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`, `System`*"]
    pub fn OpenForUserWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, filepath: Param1, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), accessmode, sharingoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`, `System`*"]
    pub fn OpenTransactedWriteForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, filepath: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`, `System`*"]
    pub fn OpenTransactedWriteForUserWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, filepath: Param1, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), user.into_param().abi(), filepath.into_param().abi(), openoptions, opendisposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    pub fn IFileRandomAccessStreamStatics<R, F: FnOnce(&IFileRandomAccessStreamStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileRandomAccessStream, IFileRandomAccessStreamStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileRandomAccessStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for FileRandomAccessStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821409, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for FileRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileRandomAccessStream";
}
impl ::std::convert::From<FileRandomAccessStream> for ::windows::runtime::IUnknown {
    fn from(value: FileRandomAccessStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileRandomAccessStream> for ::windows::runtime::IUnknown {
    fn from(value: &FileRandomAccessStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FileRandomAccessStream> for ::windows::runtime::IInspectable {
    fn from(value: FileRandomAccessStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FileRandomAccessStream> for ::windows::runtime::IInspectable {
    fn from(value: &FileRandomAccessStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<FileRandomAccessStream> for IRandomAccessStream {
    fn from(value: FileRandomAccessStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileRandomAccessStream> for IRandomAccessStream {
    fn from(value: &FileRandomAccessStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<FileRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&FileRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileRandomAccessStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileRandomAccessStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::std::convert::TryInto::<IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FileRandomAccessStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileRandomAccessStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for &FileRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::std::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for FileRandomAccessStream {}
unsafe impl ::std::marker::Sync for FileRandomAccessStream {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IBuffer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBuffer {
    type Vtable = IBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821408, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl IBuffer {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Capacity(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetLength(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBuffer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{905a0fe0-bc53-11df-8c49-001e4fc686da}");
}
impl ::std::convert::From<IBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBuffer> for ::windows::runtime::IInspectable {
    fn from(value: IBuffer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBuffer> for ::windows::runtime::IInspectable {
    fn from(value: &IBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBufferFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBufferFactory {
    type Vtable = IBufferFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1907331405, 49423, 18507, [188, 80, 20, 188, 98, 59, 58, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capacity: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBufferStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBufferStatics {
    type Vtable = IBufferStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3909215835, 55062, 18266, [169, 10, 175, 114, 41, 177, 231, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IContentTypeProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContentTypeProvider {
    type Vtable = IContentTypeProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2547030181, 15257, 19945, [136, 165, 225, 29, 47, 80, 199, 149]);
}
impl IContentTypeProvider {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContentTypeProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{97d098a5-3b99-4de9-88a5-e11d2f50c795}");
}
impl ::std::convert::From<IContentTypeProvider> for ::windows::runtime::IUnknown {
    fn from(value: IContentTypeProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContentTypeProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IContentTypeProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContentTypeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContentTypeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IContentTypeProvider> for ::windows::runtime::IInspectable {
    fn from(value: IContentTypeProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IContentTypeProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IContentTypeProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContentTypeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContentTypeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentTypeProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IDataReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataReader {
    type Vtable = IDataReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3803512873, 46273, 17172, [164, 184, 251, 129, 58, 47, 39, 94]);
}
impl IDataReader {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnconsumedBufferLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnicodeEncoding(&self) -> ::windows::runtime::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ByteOrder(&self) -> ::windows::runtime::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn InputStreamOptions(&self) -> ::windows::runtime::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__: InputStreamOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InputStreamOptions>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadByte(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadBytes(&self, value: &mut [<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute_copy(&value)).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadBuffer(&self, length: u32) -> ::windows::runtime::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), length, &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadBoolean(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadInt16(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadInt32(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadInt64(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadUInt16(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadUInt32(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadUInt64(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadSingle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadDouble(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), codeunitcount, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadDateTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadTimeSpan(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn LoadAsync(&self, count: u32) -> ::windows::runtime::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), count, &mut result__).from_abi::<DataReaderLoadOperation>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachBuffer(&self) -> ::windows::runtime::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachStream(&self) -> ::windows::runtime::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInputStream>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDataReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e2b50029-b4c1-4314-a4b8-fb813a2f275e}");
}
impl ::std::convert::From<IDataReader> for ::windows::runtime::IUnknown {
    fn from(value: IDataReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDataReader> for ::windows::runtime::IUnknown {
    fn from(value: &IDataReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDataReader> for ::windows::runtime::IInspectable {
    fn from(value: IDataReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDataReader> for ::windows::runtime::IInspectable {
    fn from(value: &IDataReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnicodeEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UnicodeEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ByteOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ByteOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InputStreamOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InputStreamOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, length: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codeunitcount: u32, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataReaderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataReaderFactory {
    type Vtable = IDataReaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3612506183, 22490, 19989, [145, 76, 6, 128, 102, 153, 160, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataReaderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataReaderStatics {
    type Vtable = IDataReaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(301776840, 63802, 18203, [177, 33, 243, 121, 227, 73, 49, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IDataWriter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataWriter {
    type Vtable = IDataWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1689817701, 54081, 18722, [179, 138, 221, 74, 248, 128, 140, 78]);
}
impl IDataWriter {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnstoredBufferLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn UnicodeEncoding(&self) -> ::windows::runtime::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__: UnicodeEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnicodeEncoding>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ByteOrder(&self) -> ::windows::runtime::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__: ByteOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ByteOrder>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteByte(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBytes(&self, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBufferRange<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, start: u32, count: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), buffer.into_param().abi(), start, count).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteBoolean(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteGuid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteInt16(&self, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteInt32(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteInt64(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteUInt16(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteUInt32(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteUInt64(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteSingle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteDouble(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn WriteString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn MeasureString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn StoreAsync(&self) -> ::windows::runtime::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DataWriterStoreOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachBuffer(&self) -> ::windows::runtime::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn DetachStream(&self) -> ::windows::runtime::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDataWriter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{64b89265-d341-4922-b38a-dd4af8808c4e}");
}
impl ::std::convert::From<IDataWriter> for ::windows::runtime::IUnknown {
    fn from(value: IDataWriter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDataWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IDataWriter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDataWriter> for ::windows::runtime::IInspectable {
    fn from(value: IDataWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDataWriter> for ::windows::runtime::IInspectable {
    fn from(value: &IDataWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnicodeEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UnicodeEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ByteOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ByteOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr, start: u32, count: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataWriterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataWriterFactory {
    type Vtable = IDataWriterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(864839618, 35716, 19499, [156, 80, 123, 135, 103, 132, 122, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileRandomAccessStreamStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileRandomAccessStreamStatics {
    type Vtable = IFileRandomAccessStreamStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1934950663, 15191, 19293, [131, 69, 85, 77, 47, 198, 33, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRandomAccessStreamStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IInputStream(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputStream {
    type Vtable = IInputStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821410, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl IInputStream {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInputStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{905a0fe2-bc53-11df-8c49-001e4fc686da}");
}
impl ::std::convert::From<IInputStream> for ::windows::runtime::IUnknown {
    fn from(value: IInputStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInputStream> for ::windows::runtime::IUnknown {
    fn from(value: &IInputStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInputStream> for ::windows::runtime::IInspectable {
    fn from(value: IInputStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInputStream> for ::windows::runtime::IInspectable {
    fn from(value: &IInputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IInputStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IInputStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for IInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &IInputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr, count: u32, options: InputStreamOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IInputStreamReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputStreamReference {
    type Vtable = IInputStreamReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1133681944, 24265, 19290, [145, 156, 66, 5, 176, 200, 4, 182]);
}
impl IInputStreamReference {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IInputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IInputStream>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInputStreamReference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{43929d18-5ec9-4b5a-919c-4205b0c804b6}");
}
impl ::std::convert::From<IInputStreamReference> for ::windows::runtime::IUnknown {
    fn from(value: IInputStreamReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInputStreamReference> for ::windows::runtime::IUnknown {
    fn from(value: &IInputStreamReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInputStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInputStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInputStreamReference> for ::windows::runtime::IInspectable {
    fn from(value: IInputStreamReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInputStreamReference> for ::windows::runtime::IInspectable {
    fn from(value: &IInputStreamReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInputStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInputStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStreamReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IOutputStream(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOutputStream {
    type Vtable = IOutputStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821414, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl IOutputStream {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IOutputStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{905a0fe6-bc53-11df-8c49-001e4fc686da}");
}
impl ::std::convert::From<IOutputStream> for ::windows::runtime::IUnknown {
    fn from(value: IOutputStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOutputStream> for ::windows::runtime::IUnknown {
    fn from(value: &IOutputStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IOutputStream> for ::windows::runtime::IInspectable {
    fn from(value: IOutputStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOutputStream> for ::windows::runtime::IInspectable {
    fn from(value: &IOutputStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IOutputStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IOutputStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for IOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &IOutputStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOutputStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IPropertySetSerializer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPropertySetSerializer {
    type Vtable = IPropertySetSerializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1854848796, 61245, 17270, [178, 14, 91, 230, 56, 174, 172, 119]);
}
impl IPropertySetSerializer {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation_Collections`*"]
    pub fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, propertyset: Param0) -> ::windows::runtime::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), propertyset.into_param().abi(), &mut result__).from_abi::<IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation_Collections`*"]
    pub fn Deserialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>, Param1: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, propertyset: Param0, buffer: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), propertyset.into_param().abi(), buffer.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPropertySetSerializer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6e8ebf1c-ef3d-4376-b20e-5be638aeac77}");
}
impl ::std::convert::From<IPropertySetSerializer> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySetSerializer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertySetSerializer> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySetSerializer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertySetSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertySetSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPropertySetSerializer> for ::windows::runtime::IInspectable {
    fn from(value: IPropertySetSerializer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertySetSerializer> for ::windows::runtime::IInspectable {
    fn from(value: &IPropertySetSerializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPropertySetSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPropertySetSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetSerializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyset: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyset: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IRandomAccessStream(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRandomAccessStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821409, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl IRandomAccessStream {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IRandomAccessStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{905a0fe1-bc53-11df-8c49-001e4fc686da}");
}
impl ::std::convert::From<IRandomAccessStream> for ::windows::runtime::IUnknown {
    fn from(value: IRandomAccessStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRandomAccessStream> for ::windows::runtime::IUnknown {
    fn from(value: &IRandomAccessStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IRandomAccessStream> for ::windows::runtime::IInspectable {
    fn from(value: IRandomAccessStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRandomAccessStream> for ::windows::runtime::IInspectable {
    fn from(value: &IRandomAccessStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IRandomAccessStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IRandomAccessStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for &IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::std::convert::TryInto::<IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IRandomAccessStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IRandomAccessStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for &IRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::std::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IRandomAccessStreamReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(871248180, 7638, 20026, [128, 103, 209, 193, 98, 232, 100, 43]);
}
impl IRandomAccessStreamReference {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn OpenReadAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IRandomAccessStreamReference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{33ee3134-1dd6-4e3a-8067-d1c162e8642b}");
}
impl ::std::convert::From<IRandomAccessStreamReference> for ::windows::runtime::IUnknown {
    fn from(value: IRandomAccessStreamReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRandomAccessStreamReference> for ::windows::runtime::IUnknown {
    fn from(value: &IRandomAccessStreamReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IRandomAccessStreamReference> for ::windows::runtime::IInspectable {
    fn from(value: IRandomAccessStreamReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRandomAccessStreamReference> for ::windows::runtime::IInspectable {
    fn from(value: &IRandomAccessStreamReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IRandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRandomAccessStreamReferenceStatics {
    type Vtable = IRandomAccessStreamReferenceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2238908892, 16319, 20093, [152, 111, 239, 59, 26, 7, 169, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRandomAccessStreamStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRandomAccessStreamStatics {
    type Vtable = IRandomAccessStreamStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1380773327, 28201, 19685, [149, 115, 107, 117, 61, 182, 108, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, bytestocopy: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Storage_Streams`*"]
pub struct IRandomAccessStreamWithContentType(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRandomAccessStreamWithContentType {
    type Vtable = IRandomAccessStreamWithContentType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3424995367, 19261, 17295, [146, 50, 16, 199, 107, 199, 224, 56]);
}
impl IRandomAccessStreamWithContentType {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IInputStream> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IOutputStream> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<IRandomAccessStream> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IRandomAccessStreamWithContentType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{cc254827-4b3d-438f-9232-10c76bc7e038}");
}
impl ::std::convert::From<IRandomAccessStreamWithContentType> for ::windows::runtime::IUnknown {
    fn from(value: IRandomAccessStreamWithContentType) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRandomAccessStreamWithContentType> for ::windows::runtime::IUnknown {
    fn from(value: &IRandomAccessStreamWithContentType) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IRandomAccessStreamWithContentType> for ::windows::runtime::IInspectable {
    fn from(value: IRandomAccessStreamWithContentType) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRandomAccessStreamWithContentType> for ::windows::runtime::IInspectable {
    fn from(value: &IRandomAccessStreamWithContentType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IRandomAccessStreamWithContentType> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IRandomAccessStreamWithContentType> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IRandomAccessStreamWithContentType> for IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IRandomAccessStreamWithContentType> for IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContentTypeProvider> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContentTypeProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContentTypeProvider> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContentTypeProvider> {
        ::std::convert::TryInto::<IContentTypeProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IRandomAccessStreamWithContentType> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IRandomAccessStreamWithContentType> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::std::convert::TryInto::<IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IRandomAccessStreamWithContentType> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IRandomAccessStreamWithContentType> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::std::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IRandomAccessStreamWithContentType> for IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IRandomAccessStreamWithContentType> for IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for &IRandomAccessStreamWithContentType {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::std::convert::TryInto::<IRandomAccessStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamWithContentType_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InMemoryRandomAccessStream(::windows::runtime::IInspectable);
impl InMemoryRandomAccessStream {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InMemoryRandomAccessStream, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InMemoryRandomAccessStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InMemoryRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for InMemoryRandomAccessStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821409, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for InMemoryRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.InMemoryRandomAccessStream";
}
impl ::std::convert::From<InMemoryRandomAccessStream> for ::windows::runtime::IUnknown {
    fn from(value: InMemoryRandomAccessStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InMemoryRandomAccessStream> for ::windows::runtime::IUnknown {
    fn from(value: &InMemoryRandomAccessStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InMemoryRandomAccessStream> for ::windows::runtime::IInspectable {
    fn from(value: InMemoryRandomAccessStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InMemoryRandomAccessStream> for ::windows::runtime::IInspectable {
    fn from(value: &InMemoryRandomAccessStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InMemoryRandomAccessStream> for IRandomAccessStream {
    fn from(value: InMemoryRandomAccessStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InMemoryRandomAccessStream> for IRandomAccessStream {
    fn from(value: &InMemoryRandomAccessStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InMemoryRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InMemoryRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InMemoryRandomAccessStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InMemoryRandomAccessStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::std::convert::TryInto::<IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InMemoryRandomAccessStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InMemoryRandomAccessStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for &InMemoryRandomAccessStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::std::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InMemoryRandomAccessStream {}
unsafe impl ::std::marker::Sync for InMemoryRandomAccessStream {}
#[doc = "*Required features: `Storage_Streams`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: InputStreamOptions = InputStreamOptions(0u32);
    pub const Partial: InputStreamOptions = InputStreamOptions(1u32);
    pub const ReadAhead: InputStreamOptions = InputStreamOptions(2u32);
}
impl ::std::convert::From<u32> for InputStreamOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InputStreamOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InputStreamOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.InputStreamOptions;u4)");
}
impl ::windows::runtime::DefaultType for InputStreamOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for InputStreamOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InputStreamOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InputStreamOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InputStreamOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InputStreamOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InputStreamOverStream(::windows::runtime::IInspectable);
impl InputStreamOverStream {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputStreamOverStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InputStreamOverStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for InputStreamOverStream {
    type Vtable = IInputStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821410, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for InputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.InputStreamOverStream";
}
impl ::std::convert::From<InputStreamOverStream> for ::windows::runtime::IUnknown {
    fn from(value: InputStreamOverStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputStreamOverStream> for ::windows::runtime::IUnknown {
    fn from(value: &InputStreamOverStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InputStreamOverStream> for ::windows::runtime::IInspectable {
    fn from(value: InputStreamOverStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputStreamOverStream> for ::windows::runtime::IInspectable {
    fn from(value: &InputStreamOverStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InputStreamOverStream> for IInputStream {
    fn from(value: InputStreamOverStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputStreamOverStream> for IInputStream {
    fn from(value: &InputStreamOverStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInputStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for &InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInputStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InputStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InputStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &InputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InputStreamOverStream {}
unsafe impl ::std::marker::Sync for InputStreamOverStream {}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OutputStreamOverStream(::windows::runtime::IInspectable);
impl OutputStreamOverStream {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OutputStreamOverStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.OutputStreamOverStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for OutputStreamOverStream {
    type Vtable = IOutputStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821414, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for OutputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.OutputStreamOverStream";
}
impl ::std::convert::From<OutputStreamOverStream> for ::windows::runtime::IUnknown {
    fn from(value: OutputStreamOverStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OutputStreamOverStream> for ::windows::runtime::IUnknown {
    fn from(value: &OutputStreamOverStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OutputStreamOverStream> for ::windows::runtime::IInspectable {
    fn from(value: OutputStreamOverStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OutputStreamOverStream> for ::windows::runtime::IInspectable {
    fn from(value: &OutputStreamOverStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<OutputStreamOverStream> for IOutputStream {
    fn from(value: OutputStreamOverStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OutputStreamOverStream> for IOutputStream {
    fn from(value: &OutputStreamOverStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IOutputStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for &OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IOutputStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<OutputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: OutputStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&OutputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &OutputStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &OutputStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for OutputStreamOverStream {}
unsafe impl ::std::marker::Sync for OutputStreamOverStream {}
#[doc = "*Required features: `Storage_Streams`*"]
pub struct RandomAccessStream {}
impl RandomAccessStream {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn CopyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IInputStream>, Param1: ::windows::runtime::IntoParam<'a, IOutputStream>>(source: Param0, destination: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn CopySizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IInputStream>, Param1: ::windows::runtime::IntoParam<'a, IOutputStream>>(source: Param0, destination: Param1, bytestocopy: u64) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), bytestocopy, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn CopyAndCloseAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IInputStream>, Param1: ::windows::runtime::IntoParam<'a, IOutputStream>>(source: Param0, destination: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    pub fn IRandomAccessStreamStatics<R, F: FnOnce(&IRandomAccessStreamStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RandomAccessStream, IRandomAccessStreamStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for RandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStream";
}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RandomAccessStreamOverStream(::windows::runtime::IInspectable);
impl RandomAccessStreamOverStream {
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0, count: u32, options: InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RandomAccessStreamOverStream {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamOverStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
unsafe impl ::windows::runtime::Interface for RandomAccessStreamOverStream {
    type Vtable = IRandomAccessStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821409, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
impl ::windows::runtime::RuntimeName for RandomAccessStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamOverStream";
}
impl ::std::convert::From<RandomAccessStreamOverStream> for ::windows::runtime::IUnknown {
    fn from(value: RandomAccessStreamOverStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RandomAccessStreamOverStream> for ::windows::runtime::IUnknown {
    fn from(value: &RandomAccessStreamOverStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RandomAccessStreamOverStream> for ::windows::runtime::IInspectable {
    fn from(value: RandomAccessStreamOverStream) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RandomAccessStreamOverStream> for ::windows::runtime::IInspectable {
    fn from(value: &RandomAccessStreamOverStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<RandomAccessStreamOverStream> for IRandomAccessStream {
    fn from(value: RandomAccessStreamOverStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RandomAccessStreamOverStream> for IRandomAccessStream {
    fn from(value: &RandomAccessStreamOverStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStream> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<RandomAccessStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&RandomAccessStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<RandomAccessStreamOverStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RandomAccessStreamOverStream> for IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInputStream> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInputStream> {
        ::std::convert::TryInto::<IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<RandomAccessStreamOverStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RandomAccessStreamOverStream> for IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOutputStream> for &RandomAccessStreamOverStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOutputStream> {
        ::std::convert::TryInto::<IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RandomAccessStreamOverStream {}
unsafe impl ::std::marker::Sync for RandomAccessStreamOverStream {}
#[doc = "*Required features: `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RandomAccessStreamReference(::windows::runtime::IInspectable);
impl RandomAccessStreamReference {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn OpenReadAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CreateFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<RandomAccessStreamReference> {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Streams`, `Foundation`*"]
    pub fn CreateFromUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<RandomAccessStreamReference> {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    #[doc = "*Required features: `Storage_Streams`*"]
    pub fn CreateFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, IRandomAccessStream>>(stream: Param0) -> ::windows::runtime::Result<RandomAccessStreamReference> {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    pub fn IRandomAccessStreamReferenceStatics<R, F: FnOnce(&IRandomAccessStreamReferenceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RandomAccessStreamReference, IRandomAccessStreamReferenceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RandomAccessStreamReference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamReference;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
}
unsafe impl ::windows::runtime::Interface for RandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(871248180, 7638, 20026, [128, 103, 209, 193, 98, 232, 100, 43]);
}
impl ::windows::runtime::RuntimeName for RandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
impl ::std::convert::From<RandomAccessStreamReference> for ::windows::runtime::IUnknown {
    fn from(value: RandomAccessStreamReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RandomAccessStreamReference> for ::windows::runtime::IUnknown {
    fn from(value: &RandomAccessStreamReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RandomAccessStreamReference> for ::windows::runtime::IInspectable {
    fn from(value: RandomAccessStreamReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RandomAccessStreamReference> for ::windows::runtime::IInspectable {
    fn from(value: &RandomAccessStreamReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<RandomAccessStreamReference> for IRandomAccessStreamReference {
    fn from(value: RandomAccessStreamReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RandomAccessStreamReference> for IRandomAccessStreamReference {
    fn from(value: &RandomAccessStreamReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStreamReference> for RandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStreamReference> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStreamReference>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRandomAccessStreamReference> for &RandomAccessStreamReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRandomAccessStreamReference> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRandomAccessStreamReference>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for RandomAccessStreamReference {}
unsafe impl ::std::marker::Sync for RandomAccessStreamReference {}
#[doc = "*Required features: `Storage_Streams`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: UnicodeEncoding = UnicodeEncoding(0i32);
    pub const Utf16LE: UnicodeEncoding = UnicodeEncoding(1i32);
    pub const Utf16BE: UnicodeEncoding = UnicodeEncoding(2i32);
}
impl ::std::convert::From<i32> for UnicodeEncoding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnicodeEncoding {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnicodeEncoding {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.UnicodeEncoding;i4)");
}
impl ::windows::runtime::DefaultType for UnicodeEncoding {
    type DefaultType = Self;
}
