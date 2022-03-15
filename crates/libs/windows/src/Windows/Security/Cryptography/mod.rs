#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Security_Cryptography_Core")]
pub mod Core;
#[cfg(feature = "Security_Cryptography_DataProtection")]
pub mod DataProtection;
#[doc = "*Required features: `\"Security_Cryptography\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[doc = "*Required features: `\"Security_Cryptography\"`*"]
pub struct CryptographicBuffer {}
impl CryptographicBuffer {
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Compare<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(object1: Param0, object2: Param1) -> ::windows::core::Result<bool> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compare)(::core::mem::transmute_copy(this), object1.into_param().abi(), object2.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GenerateRandom(length: u32) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GenerateRandom)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`*"]
    pub fn GenerateRandomNumber() -> ::windows::core::Result<u32> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GenerateRandomNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromByteArray(value: &[u8]) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromByteArray)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyToByteArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        Self::ICryptographicBufferStatics(|this| unsafe { (::windows::core::Interface::vtable(this).CopyToByteArray)(::core::mem::transmute_copy(this), buffer.into_param().abi(), value.set_abi_len(), value as *mut _ as _).ok() })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DecodeFromHexString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DecodeFromHexString)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncodeToHexString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodeToHexString)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DecodeFromBase64String<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DecodeFromBase64String)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncodeToBase64String<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodeToBase64String)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ConvertStringToBinary<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0, encoding: BinaryStringEncoding) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConvertStringToBinary)(::core::mem::transmute_copy(this), value.into_param().abi(), encoding, &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ConvertBinaryToString<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(encoding: BinaryStringEncoding, buffer: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConvertBinaryToString)(::core::mem::transmute_copy(this), encoding, buffer.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICryptographicBufferStatics<R, F: FnOnce(&ICryptographicBufferStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CryptographicBuffer, ICryptographicBufferStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CryptographicBuffer {
    const NAME: &'static str = "Windows.Security.Cryptography.CryptographicBuffer";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicBufferStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICryptographicBufferStatics {
    type Vtable = ICryptographicBufferStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320b7e22_3cb0_4cdf_8663_1d28910065eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicBufferStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object1: ::windows::core::RawPtr, object2: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Compare: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GenerateRandom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GenerateRandom: usize,
    pub GenerateRandomNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromByteArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromByteArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CopyToByteArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyToByteArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecodeFromHexString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecodeFromHexString: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncodeToHexString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncodeToHexString: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecodeFromBase64String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecodeFromBase64String: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncodeToBase64String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncodeToBase64String: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ConvertStringToBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: BinaryStringEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ConvertStringToBinary: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ConvertBinaryToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoding: BinaryStringEncoding, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ConvertBinaryToString: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
