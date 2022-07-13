#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FinishAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DetachStream)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCompressor<'a, P0, E0>(underlyingstream: P0) -> ::windows::core::Result<Compressor>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateCompressor)(::windows::core::Interface::as_raw(this), underlyingstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<Compressor>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCompressorEx<'a, P0, E0>(underlyingstream: P0, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::core::Result<Compressor>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Streams::IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateCompressorEx)(::windows::core::Interface::as_raw(this), underlyingstream.try_into().map_err(|e| e.into())?.abi(), algorithm, blocksize, result__.as_mut_ptr()).from_abi::<Compressor>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WriteAsync)(::windows::core::Interface::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FlushAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICompressorFactory<R, F: FnOnce(&ICompressorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Compressor, ICompressorFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
impl ::core::convert::From<&Compressor> for &::windows::core::IUnknown {
    fn from(value: &Compressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl ::core::convert::From<&Compressor> for &::windows::core::IInspectable {
    fn from(value: &Compressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&Compressor> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Compressor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&Compressor> for ::windows::core::InParam<'a, super::Streams::IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Compressor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Compressor {}
unsafe impl ::core::marker::Sync for Compressor {}
#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
pub struct Decompressor(::windows::core::IUnknown);
impl Decompressor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DetachStream)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateDecompressor<'a, P0, E0>(underlyingstream: P0) -> ::windows::core::Result<Decompressor>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IDecompressorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateDecompressor)(::windows::core::Interface::as_raw(this), underlyingstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<Decompressor>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: super::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReadAsync)(::windows::core::Interface::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDecompressorFactory<R, F: FnOnce(&IDecompressorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Decompressor, IDecompressorFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
impl ::core::convert::From<&Decompressor> for &::windows::core::IUnknown {
    fn from(value: &Decompressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl ::core::convert::From<&Decompressor> for &::windows::core::IInspectable {
    fn from(value: &Decompressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&Decompressor> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Decompressor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&Decompressor> for ::windows::core::InParam<'a, super::Streams::IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Decompressor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underlyingstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCompressor: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressorEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underlyingstream: *mut ::core::ffi::c_void, algorithm: CompressAlgorithm, blocksize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateDecompressor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underlyingstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateDecompressor: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
