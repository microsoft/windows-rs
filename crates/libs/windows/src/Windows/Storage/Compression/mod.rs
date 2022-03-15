#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CompressAlgorithm(pub i32);
impl CompressAlgorithm {
    pub const InvalidAlgorithm: Self = Self(0i32);
    pub const NullAlgorithm: Self = Self(1i32);
    pub const Mszip: Self = Self(2i32);
    pub const Xpress: Self = Self(3i32);
    pub const XpressHuff: Self = Self(4i32);
    pub const Lzms: Self = Self(5i32);
}
impl ::core::marker::Copy for CompressAlgorithm {}
impl ::core::clone::Clone for CompressAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompressAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CompressAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for CompressAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompressAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompressAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Compression.CompressAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
pub struct Compressor(::windows::core::IUnknown);
impl Compressor {
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinishAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetachStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCompressor<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IOutputStream>>(underlyingstream: Param0) -> ::windows::core::Result<Compressor> {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCompressor)(::core::mem::transmute_copy(this), underlyingstream.into_param().abi(), &mut result__).from_abi::<Compressor>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCompressorEx<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IOutputStream>>(underlyingstream: Param0, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::core::Result<Compressor> {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCompressorEx)(::core::mem::transmute_copy(this), underlyingstream.into_param().abi(), algorithm, blocksize, &mut result__).from_abi::<Compressor>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteAsync)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlushAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICompressorFactory<R, F: FnOnce(&ICompressorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Compressor, ICompressorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Compressor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Compressor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Compressor {}
impl ::core::fmt::Debug for Compressor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Compressor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Compressor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Compression.Compressor;{0ac3645a-57ac-4ee1-b702-84d39d5424e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Compressor {
    type Vtable = ICompressor_Vtbl;
    const IID: ::windows::core::GUID = <ICompressor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Compressor {
    const NAME: &'static str = "Windows.Storage.Compression.Compressor";
}
impl ::core::convert::From<Compressor> for ::windows::core::IUnknown {
    fn from(value: Compressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Compressor> for ::windows::core::IUnknown {
    fn from(value: &Compressor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Compressor> for ::windows::core::IInspectable {
    fn from(value: Compressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Compressor> for ::windows::core::IInspectable {
    fn from(value: &Compressor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Compressor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Compressor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Compressor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Compressor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<Compressor> for super::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: Compressor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&Compressor> for super::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &Compressor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IOutputStream> for Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IOutputStream> for &Compressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Compressor {}
unsafe impl ::core::marker::Sync for Compressor {}
#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
pub struct Decompressor(::windows::core::IUnknown);
impl Decompressor {
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetachStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateDecompressor<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IInputStream>>(underlyingstream: Param0) -> ::windows::core::Result<Decompressor> {
        Self::IDecompressorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDecompressor)(::core::mem::transmute_copy(this), underlyingstream.into_param().abi(), &mut result__).from_abi::<Decompressor>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Compression\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadAsync)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDecompressorFactory<R, F: FnOnce(&IDecompressorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Decompressor, IDecompressorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Decompressor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Decompressor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Decompressor {}
impl ::core::fmt::Debug for Decompressor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Decompressor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Decompressor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Compression.Decompressor;{b883fe46-d68a-4c8b-ada0-4ee813fc5283})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Decompressor {
    type Vtable = IDecompressor_Vtbl;
    const IID: ::windows::core::GUID = <IDecompressor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Decompressor {
    const NAME: &'static str = "Windows.Storage.Compression.Decompressor";
}
impl ::core::convert::From<Decompressor> for ::windows::core::IUnknown {
    fn from(value: Decompressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Decompressor> for ::windows::core::IUnknown {
    fn from(value: &Decompressor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Decompressor> for ::windows::core::IInspectable {
    fn from(value: Decompressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Decompressor> for ::windows::core::IInspectable {
    fn from(value: &Decompressor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Decompressor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Decompressor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Decompressor> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Decompressor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<Decompressor> for super::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: Decompressor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&Decompressor> for super::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &Decompressor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IInputStream> for Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IInputStream> for &Decompressor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IInputStream> {
        ::core::convert::TryInto::<super::Streams::IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Decompressor {}
unsafe impl ::core::marker::Sync for Decompressor {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompressor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompressor {
    type Vtable = ICompressor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ac3645a_57ac_4ee1_b702_84d39d5424e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompressor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DetachStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompressorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompressorFactory {
    type Vtable = ICompressorFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f3d96a4_2cfb_442c_a8ba_d7d11b039da0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompressorFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCompressor: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressorEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, algorithm: CompressAlgorithm, blocksize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCompressorEx: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDecompressor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDecompressor {
    type Vtable = IDecompressor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb883fe46_d68a_4c8b_ada0_4ee813fc5283);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DetachStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDecompressorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDecompressorFactory {
    type Vtable = IDecompressorFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5337e252_1da2_42e1_8834_0379d28d742f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressorFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateDecompressor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateDecompressor: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
