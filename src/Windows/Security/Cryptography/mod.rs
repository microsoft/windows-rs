#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Security_Cryptography_Core")]
pub mod Core;
#[cfg(feature = "Security_Cryptography_DataProtection")]
pub mod DataProtection;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BinaryStringEncoding(pub i32);
impl BinaryStringEncoding {
    pub const Utf8: BinaryStringEncoding = BinaryStringEncoding(0i32);
    pub const Utf16LE: BinaryStringEncoding = BinaryStringEncoding(1i32);
    pub const Utf16BE: BinaryStringEncoding = BinaryStringEncoding(2i32);
}
impl ::core::convert::From<i32> for BinaryStringEncoding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BinaryStringEncoding {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BinaryStringEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.BinaryStringEncoding;i4)");
}
impl ::windows::core::DefaultType for BinaryStringEncoding {
    type DefaultType = Self;
}
pub struct CryptographicBuffer {}
impl CryptographicBuffer {
    #[cfg(feature = "Storage_Streams")]
    pub fn Compare<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(object1: Param0, object2: Param1) -> ::windows::core::Result<bool> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), object1.into_param().abi(), object2.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GenerateRandom(length: u32) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    pub fn GenerateRandomNumber() -> ::windows::core::Result<u32> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromByteArray(value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyToByteArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        Self::ICryptographicBufferStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), buffer.into_param().abi(), value.set_abi_len(), value as *mut _ as _).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DecodeFromHexString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn EncodeToHexString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DecodeFromBase64String<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn EncodeToBase64String<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ConvertStringToBinary<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0, encoding: BinaryStringEncoding) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi(), encoding, &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ConvertBinaryToString<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(encoding: BinaryStringEncoding, buffer: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), encoding, buffer.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ICryptographicBufferStatics<R, F: FnOnce(&ICryptographicBufferStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CryptographicBuffer, ICryptographicBufferStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CryptographicBuffer {
    const NAME: &'static str = "Windows.Security.Cryptography.CryptographicBuffer";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICryptographicBufferStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICryptographicBufferStatics {
    type Vtable = ICryptographicBufferStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320b7e22_3cb0_4cdf_8663_1d28910065eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicBufferStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, object1: ::windows::core::RawPtr, object2: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: BinaryStringEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encoding: BinaryStringEncoding, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
