#[cfg(feature = "Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Security_Cryptography_Core")]
pub mod Core;
#[cfg(feature = "Security_Cryptography_DataProtection")]
pub mod DataProtection;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicBufferStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICryptographicBufferStatics {
    type Vtable = ICryptographicBufferStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICryptographicBufferStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320b7e22_3cb0_4cdf_8663_1d28910065eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicBufferStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object1: *mut ::core::ffi::c_void, object2: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Compare: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GenerateRandom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GenerateRandom: usize,
    pub GenerateRandomNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromByteArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromByteArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CopyToByteArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyToByteArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecodeFromHexString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecodeFromHexString: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncodeToHexString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncodeToHexString: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecodeFromBase64String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecodeFromBase64String: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncodeToBase64String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncodeToBase64String: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ConvertStringToBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, encoding: BinaryStringEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ConvertStringToBinary: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ConvertBinaryToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoding: BinaryStringEncoding, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ConvertBinaryToString: usize,
}
#[doc = "*Required features: `\"Security_Cryptography\"`*"]
pub struct CryptographicBuffer;
impl CryptographicBuffer {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Compare<P0, E0, P1, E1>(object1: P0, object2: P1) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compare)(::windows::core::Vtable::as_raw(this), object1.try_into().map_err(|e| e.into())?.abi(), object2.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GenerateRandom(length: u32) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateRandom)(::windows::core::Vtable::as_raw(this), length, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GenerateRandomNumber() -> ::windows::core::Result<u32> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateRandomNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromByteArray(value: &[u8]) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromByteArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyToByteArray<P0, E0>(buffer: P0, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicBufferStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).CopyToByteArray)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), value.set_abi_len(), value as *mut _ as _).ok() })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DecodeFromHexString(value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DecodeFromHexString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncodeToHexString<P0, E0>(buffer: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EncodeToHexString)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DecodeFromBase64String(value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DecodeFromBase64String)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncodeToBase64String<P0, E0>(buffer: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EncodeToBase64String)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ConvertStringToBinary(value: &::windows::core::HSTRING, encoding: BinaryStringEncoding) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertStringToBinary)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), encoding, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ConvertBinaryToString<P0, E0>(encoding: BinaryStringEncoding, buffer: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertBinaryToString)(::windows::core::Vtable::as_raw(this), encoding, buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICryptographicBufferStatics<R, F: FnOnce(&ICryptographicBufferStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CryptographicBuffer, ICryptographicBufferStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CryptographicBuffer {
    const NAME: &'static str = "Windows.Security.Cryptography.CryptographicBuffer";
}
#[doc = "*Required features: `\"Security_Cryptography\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BinaryStringEncoding(pub i32);
impl BinaryStringEncoding {
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}
impl ::core::marker::Copy for BinaryStringEncoding {}
impl ::core::clone::Clone for BinaryStringEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BinaryStringEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BinaryStringEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for BinaryStringEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BinaryStringEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BinaryStringEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.BinaryStringEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
