#[doc(hidden)]
#[repr(transparent)]
pub struct ICompressor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompressor {
    type Vtable = ICompressor_Vtbl;
}
impl ::core::clone::Clone for ICompressor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompressor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ac3645a_57ac_4ee1_b702_84d39d5424e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompressor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for ICompressorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompressorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f3d96a4_2cfb_442c_a8ba_d7d11b039da0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompressorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IDecompressor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDecompressor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb883fe46_d68a_4c8b_ada0_4ee813fc5283);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IDecompressorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDecompressorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5337e252_1da2_42e1_8834_0379d28d742f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateDecompressor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underlyingstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateDecompressor: usize,
}
#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
pub struct Compressor(::windows::core::IUnknown);
impl Compressor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).FinishAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).DetachStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCompressor<P0>(underlyingstream: P0) -> ::windows::core::Result<Compressor>
    where
        P0: ::windows::core::TryIntoParam<super::Streams::IOutputStream>,
    {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Compressor>();
            (::windows::core::Interface::vtable(this).CreateCompressor)(::windows::core::Interface::as_raw(this), underlyingstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCompressorEx<P0>(underlyingstream: P0, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::core::Result<Compressor>
    where
        P0: ::windows::core::TryIntoParam<super::Streams::IOutputStream>,
    {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Compressor>();
            (::windows::core::Interface::vtable(this).CreateCompressorEx)(::windows::core::Interface::as_raw(this), underlyingstream.try_into_param()?.abi(), algorithm, blocksize, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows::core::TryIntoParam<super::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>();
            (::windows::core::Interface::vtable(this).WriteAsync)(::windows::core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).FlushAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICompressorFactory<R, F: FnOnce(&ICompressorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Compressor, ICompressorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for Compressor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Compression.Compressor;{0ac3645a-57ac-4ee1-b702-84d39d5424e0})");
}
impl ::core::clone::Clone for Compressor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Compressor {
    type Vtable = ICompressor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Compressor {
    const IID: ::windows::core::GUID = <ICompressor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Compressor {
    const NAME: &'static str = "Windows.Storage.Compression.Compressor";
}
::windows::imp::interface_hierarchy!(Compressor, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for Compressor {}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::CanTryInto<super::Streams::IOutputStream> for Compressor {}
unsafe impl ::core::marker::Send for Compressor {}
unsafe impl ::core::marker::Sync for Compressor {}
#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
pub struct Decompressor(::windows::core::IUnknown);
impl Decompressor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).DetachStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateDecompressor<P0>(underlyingstream: P0) -> ::windows::core::Result<Decompressor>
    where
        P0: ::windows::core::TryIntoParam<super::Streams::IInputStream>,
    {
        Self::IDecompressorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Decompressor>();
            (::windows::core::Interface::vtable(this).CreateDecompressor)(::windows::core::Interface::as_raw(this), underlyingstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>
    where
        P0: ::windows::core::TryIntoParam<super::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>();
            (::windows::core::Interface::vtable(this).ReadAsync)(::windows::core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDecompressorFactory<R, F: FnOnce(&IDecompressorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Decompressor, IDecompressorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for Decompressor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Compression.Decompressor;{b883fe46-d68a-4c8b-ada0-4ee813fc5283})");
}
impl ::core::clone::Clone for Decompressor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Decompressor {
    type Vtable = IDecompressor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Decompressor {
    const IID: ::windows::core::GUID = <IDecompressor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Decompressor {
    const NAME: &'static str = "Windows.Storage.Compression.Decompressor";
}
::windows::imp::interface_hierarchy!(Decompressor, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for Decompressor {}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::CanTryInto<super::Streams::IInputStream> for Decompressor {}
unsafe impl ::core::marker::Send for Decompressor {}
unsafe impl ::core::marker::Sync for Decompressor {}
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
impl ::windows::core::TypeKind for CompressAlgorithm {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CompressAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompressAlgorithm").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CompressAlgorithm {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Compression.CompressAlgorithm;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
