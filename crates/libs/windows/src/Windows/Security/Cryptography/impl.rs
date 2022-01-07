#[cfg(feature = "implement_exclusive")]
pub trait ICryptographicBufferStaticsImpl: Sized {
    fn Compare(&self, object1: &::core::option::Option<super::super::Storage::Streams::IBuffer>, object2: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<bool>;
    fn GenerateRandom(&self, length: u32) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GenerateRandomNumber(&self) -> ::windows::core::Result<u32>;
    fn CreateFromByteArray(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CopyToByteArray(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn DecodeFromHexString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn EncodeToHexString(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DecodeFromBase64String(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn EncodeToBase64String(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConvertStringToBinary(&self, value: &::windows::core::HSTRING, encoding: BinaryStringEncoding) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ConvertBinaryToString(&self, encoding: BinaryStringEncoding, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICryptographicBufferStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.ICryptographicBufferStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICryptographicBufferStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>() -> ICryptographicBufferStaticsVtbl {
        unsafe extern "system" fn Compare<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object1: ::windows::core::RawPtr, object2: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&object1 as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&object2 as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateRandom<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateRandom(length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateRandomNumber<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateRandomNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromByteArray<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromByteArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToByteArray<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyToByteArray(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), ::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn DecodeFromHexString<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodeFromHexString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodeToHexString<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeToHexString(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecodeFromBase64String<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodeFromBase64String(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodeToBase64String<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeToBase64String(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringToBinary<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: BinaryStringEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringToBinary(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertBinaryToString<Impl: ICryptographicBufferStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: BinaryStringEncoding, buffer: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertBinaryToString(encoding, &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICryptographicBufferStatics>,
            ::windows::core::GetTrustLevel,
            Compare::<Impl, OFFSET>,
            GenerateRandom::<Impl, OFFSET>,
            GenerateRandomNumber::<Impl, OFFSET>,
            CreateFromByteArray::<Impl, OFFSET>,
            CopyToByteArray::<Impl, OFFSET>,
            DecodeFromHexString::<Impl, OFFSET>,
            EncodeToHexString::<Impl, OFFSET>,
            DecodeFromBase64String::<Impl, OFFSET>,
            EncodeToBase64String::<Impl, OFFSET>,
            ConvertStringToBinary::<Impl, OFFSET>,
            ConvertBinaryToString::<Impl, OFFSET>,
        )
    }
}
