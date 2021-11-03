#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Storage_Compression`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CompressAlgorithm(pub i32);
impl CompressAlgorithm {
    pub const InvalidAlgorithm: CompressAlgorithm = CompressAlgorithm(0i32);
    pub const NullAlgorithm: CompressAlgorithm = CompressAlgorithm(1i32);
    pub const Mszip: CompressAlgorithm = CompressAlgorithm(2i32);
    pub const Xpress: CompressAlgorithm = CompressAlgorithm(3i32);
    pub const XpressHuff: CompressAlgorithm = CompressAlgorithm(4i32);
    pub const Lzms: CompressAlgorithm = CompressAlgorithm(5i32);
}
impl ::std::convert::From<i32> for CompressAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CompressAlgorithm {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CompressAlgorithm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Storage.Compression.CompressAlgorithm;i4)");
}
impl ::windows::runtime::DefaultType for CompressAlgorithm {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage_Compression`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Compressor(::windows::runtime::IInspectable);
impl Compressor {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Compression`, `Foundation`*"]
    pub fn FinishAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_Compression`, `Storage_Streams`*"]
    pub fn DetachStream(&self) -> ::windows::runtime::Result<super::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Compression`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_Compression`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_Compression`, `Foundation`, `Storage_Streams`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_Compression`, `Storage_Streams`*"]
    pub fn CreateCompressor<'a, Param0: ::windows::runtime::IntoParam<'a, super::Streams::IOutputStream>>(underlyingstream: Param0) -> ::windows::runtime::Result<Compressor> {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), underlyingstream.into_param().abi(), &mut result__).from_abi::<Compressor>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_Compression`, `Storage_Streams`*"]
    pub fn CreateCompressorEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Streams::IOutputStream>>(underlyingstream: Param0, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::runtime::Result<Compressor> {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), underlyingstream.into_param().abi(), algorithm, blocksize, &mut result__).from_abi::<Compressor>(result__)
        })
    }
    pub fn ICompressorFactory<R, F: FnOnce(&ICompressorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Compressor, ICompressorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Compressor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Compression.Compressor;{0ac3645a-57ac-4ee1-b702-84d39d5424e0})");
}
unsafe impl ::windows::runtime::Interface for Compressor {
    type Vtable = ICompressor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(180577370, 22444, 20193, [183, 2, 132, 211, 157, 84, 36, 224]);
}
impl ::windows::runtime::RuntimeName for Compressor {
    const NAME: &'static str = "Windows.Storage.Compression.Compressor";
}
impl ::std::convert::From<Compressor> for ::windows::runtime::IUnknown {
    fn from(value: Compressor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Compressor> for ::windows::runtime::IUnknown {
    fn from(value: &Compressor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Compressor> for ::windows::runtime::IInspectable {
    fn from(value: Compressor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Compressor> for ::windows::runtime::IInspectable {
    fn from(value: &Compressor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<Compressor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Compressor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&Compressor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Compressor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<Compressor> for super::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Compressor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&Compressor> for super::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Compressor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IOutputStream> for Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IOutputStream> for &Compressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IOutputStream> {
        ::std::convert::TryInto::<super::Streams::IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Compressor {}
unsafe impl ::std::marker::Sync for Compressor {}
#[doc = "*Required features: `Storage_Compression`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Decompressor(::windows::runtime::IInspectable);
impl Decompressor {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_Compression`, `Storage_Streams`*"]
    pub fn DetachStream(&self) -> ::windows::runtime::Result<super::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage_Compression`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage_Compression`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::Streams::InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage_Compression`, `Storage_Streams`*"]
    pub fn CreateDecompressor<'a, Param0: ::windows::runtime::IntoParam<'a, super::Streams::IInputStream>>(underlyingstream: Param0) -> ::windows::runtime::Result<Decompressor> {
        Self::IDecompressorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), underlyingstream.into_param().abi(), &mut result__).from_abi::<Decompressor>(result__)
        })
    }
    pub fn IDecompressorFactory<R, F: FnOnce(&IDecompressorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Decompressor, IDecompressorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Decompressor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Storage.Compression.Decompressor;{b883fe46-d68a-4c8b-ada0-4ee813fc5283})");
}
unsafe impl ::windows::runtime::Interface for Decompressor {
    type Vtable = IDecompressor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3095658054, 54922, 19595, [173, 160, 78, 232, 19, 252, 82, 131]);
}
impl ::windows::runtime::RuntimeName for Decompressor {
    const NAME: &'static str = "Windows.Storage.Compression.Decompressor";
}
impl ::std::convert::From<Decompressor> for ::windows::runtime::IUnknown {
    fn from(value: Decompressor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Decompressor> for ::windows::runtime::IUnknown {
    fn from(value: &Decompressor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Decompressor> for ::windows::runtime::IInspectable {
    fn from(value: Decompressor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Decompressor> for ::windows::runtime::IInspectable {
    fn from(value: &Decompressor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<Decompressor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Decompressor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&Decompressor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Decompressor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<Decompressor> for super::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Decompressor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&Decompressor> for super::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Decompressor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IInputStream> for Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Streams::IInputStream> for &Decompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Streams::IInputStream> {
        ::std::convert::TryInto::<super::Streams::IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Decompressor {}
unsafe impl ::std::marker::Sync for Decompressor {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompressor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompressor {
    type Vtable = ICompressor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(180577370, 22444, 20193, [183, 2, 132, 211, 157, 84, 36, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompressor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompressorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompressorFactory {
    type Vtable = ICompressorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1597871780, 11515, 17452, [168, 186, 215, 209, 27, 3, 157, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompressorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, underlyingstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, underlyingstream: ::windows::runtime::RawPtr, algorithm: CompressAlgorithm, blocksize: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDecompressor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDecompressor {
    type Vtable = IDecompressor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3095658054, 54922, 19595, [173, 160, 78, 232, 19, 252, 82, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDecompressorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDecompressorFactory {
    type Vtable = IDecompressorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1396171346, 7586, 17121, [136, 52, 3, 121, 210, 141, 116, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, underlyingstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
