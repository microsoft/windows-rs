pub trait IBuffer_Impl: Sized {
    fn Capacity(&mut self) -> ::windows::core::Result<u32>;
    fn Length(&mut self) -> ::windows::core::Result<u32>;
    fn SetLength(&mut self, value: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBuffer {
    const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
}
impl IBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBuffer_Vtbl {
        unsafe extern "system" fn Capacity<Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBuffer, BASE_OFFSET>(),
            Capacity: Capacity::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBufferFactory_Impl: Sized {
    fn Create(&mut self, capacity: u32) -> ::windows::core::Result<Buffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBufferFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IBufferFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBufferFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBufferFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBufferFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IBufferFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(capacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBufferFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBufferFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBufferStatics_Impl: Sized {
    fn CreateCopyFromMemoryBuffer(&mut self, input: &::core::option::Option<super::super::Foundation::IMemoryBuffer>) -> ::windows::core::Result<Buffer>;
    fn CreateMemoryBufferOverIBuffer(&mut self, input: &::core::option::Option<IBuffer>) -> ::windows::core::Result<super::super::Foundation::MemoryBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBufferStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IBufferStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBufferStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBufferStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBufferStatics_Vtbl {
        unsafe extern "system" fn CreateCopyFromMemoryBuffer<Impl: IBufferStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCopyFromMemoryBuffer(&*(&input as *const <super::super::Foundation::IMemoryBuffer as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IMemoryBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMemoryBufferOverIBuffer<Impl: IBufferStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMemoryBufferOverIBuffer(&*(&input as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBufferStatics, BASE_OFFSET>(),
            CreateCopyFromMemoryBuffer: CreateCopyFromMemoryBuffer::<Impl, IMPL_OFFSET>,
            CreateMemoryBufferOverIBuffer: CreateMemoryBufferOverIBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBufferStatics as ::windows::core::Interface>::IID
    }
}
pub trait IContentTypeProvider_Impl: Sized {
    fn ContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContentTypeProvider {
    const NAME: &'static str = "Windows.Storage.Streams.IContentTypeProvider";
}
impl IContentTypeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentTypeProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentTypeProvider_Vtbl {
        unsafe extern "system" fn ContentType<Impl: IContentTypeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContentTypeProvider, BASE_OFFSET>(), ContentType: ContentType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentTypeProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IDataReader_Impl: Sized {
    fn UnconsumedBufferLength(&mut self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&mut self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&mut self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&mut self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&mut self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn InputStreamOptions(&mut self) -> ::windows::core::Result<InputStreamOptions>;
    fn SetInputStreamOptions(&mut self, value: InputStreamOptions) -> ::windows::core::Result<()>;
    fn ReadByte(&mut self) -> ::windows::core::Result<u8>;
    fn ReadBytes(&mut self, value: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ReadBuffer(&mut self, length: u32) -> ::windows::core::Result<IBuffer>;
    fn ReadBoolean(&mut self) -> ::windows::core::Result<bool>;
    fn ReadGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReadInt16(&mut self) -> ::windows::core::Result<i16>;
    fn ReadInt32(&mut self) -> ::windows::core::Result<i32>;
    fn ReadInt64(&mut self) -> ::windows::core::Result<i64>;
    fn ReadUInt16(&mut self) -> ::windows::core::Result<u16>;
    fn ReadUInt32(&mut self) -> ::windows::core::Result<u32>;
    fn ReadUInt64(&mut self) -> ::windows::core::Result<u64>;
    fn ReadSingle(&mut self) -> ::windows::core::Result<f32>;
    fn ReadDouble(&mut self) -> ::windows::core::Result<f64>;
    fn ReadString(&mut self, codeunitcount: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadDateTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ReadTimeSpan(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn LoadAsync(&mut self, count: u32) -> ::windows::core::Result<DataReaderLoadOperation>;
    fn DetachBuffer(&mut self) -> ::windows::core::Result<IBuffer>;
    fn DetachStream(&mut self) -> ::windows::core::Result<IInputStream>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDataReader {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReader";
}
#[cfg(feature = "Foundation")]
impl IDataReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataReader_Vtbl {
        unsafe extern "system" fn UnconsumedBufferLength<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnconsumedBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicodeEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnicodeEncoding(value).into()
        }
        unsafe extern "system" fn ByteOrder<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ByteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetByteOrder(value).into()
        }
        unsafe extern "system" fn InputStreamOptions<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InputStreamOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputStreamOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputStreamOptions<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InputStreamOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputStreamOptions(value).into()
        }
        unsafe extern "system" fn ReadByte<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadByte() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBytes<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn ReadBuffer<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBuffer(length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBoolean<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBoolean() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadGuid<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt16<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt32<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt64<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt16<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadUInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt32<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadUInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt64<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadUInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSingle<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadSingle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDouble<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadDouble() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadString<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codeunitcount: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadString(codeunitcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDateTime<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTimeSpan<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadTimeSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetachBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataReader, BASE_OFFSET>(),
            UnconsumedBufferLength: UnconsumedBufferLength::<Impl, IMPL_OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Impl, IMPL_OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Impl, IMPL_OFFSET>,
            ByteOrder: ByteOrder::<Impl, IMPL_OFFSET>,
            SetByteOrder: SetByteOrder::<Impl, IMPL_OFFSET>,
            InputStreamOptions: InputStreamOptions::<Impl, IMPL_OFFSET>,
            SetInputStreamOptions: SetInputStreamOptions::<Impl, IMPL_OFFSET>,
            ReadByte: ReadByte::<Impl, IMPL_OFFSET>,
            ReadBytes: ReadBytes::<Impl, IMPL_OFFSET>,
            ReadBuffer: ReadBuffer::<Impl, IMPL_OFFSET>,
            ReadBoolean: ReadBoolean::<Impl, IMPL_OFFSET>,
            ReadGuid: ReadGuid::<Impl, IMPL_OFFSET>,
            ReadInt16: ReadInt16::<Impl, IMPL_OFFSET>,
            ReadInt32: ReadInt32::<Impl, IMPL_OFFSET>,
            ReadInt64: ReadInt64::<Impl, IMPL_OFFSET>,
            ReadUInt16: ReadUInt16::<Impl, IMPL_OFFSET>,
            ReadUInt32: ReadUInt32::<Impl, IMPL_OFFSET>,
            ReadUInt64: ReadUInt64::<Impl, IMPL_OFFSET>,
            ReadSingle: ReadSingle::<Impl, IMPL_OFFSET>,
            ReadDouble: ReadDouble::<Impl, IMPL_OFFSET>,
            ReadString: ReadString::<Impl, IMPL_OFFSET>,
            ReadDateTime: ReadDateTime::<Impl, IMPL_OFFSET>,
            ReadTimeSpan: ReadTimeSpan::<Impl, IMPL_OFFSET>,
            LoadAsync: LoadAsync::<Impl, IMPL_OFFSET>,
            DetachBuffer: DetachBuffer::<Impl, IMPL_OFFSET>,
            DetachStream: DetachStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataReaderFactory_Impl: Sized {
    fn CreateDataReader(&mut self, inputstream: &::core::option::Option<IInputStream>) -> ::windows::core::Result<DataReader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataReaderFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataReaderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataReaderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataReaderFactory_Vtbl {
        unsafe extern "system" fn CreateDataReader<Impl: IDataReaderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDataReader(&*(&inputstream as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataReaderFactory, BASE_OFFSET>(),
            CreateDataReader: CreateDataReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataReaderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataReaderStatics_Impl: Sized {
    fn FromBuffer(&mut self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<DataReader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataReaderStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDataReaderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataReaderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataReaderStatics_Vtbl {
        unsafe extern "system" fn FromBuffer<Impl: IDataReaderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromBuffer(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDataReaderStatics, BASE_OFFSET>(), FromBuffer: FromBuffer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataReaderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IDataWriter_Impl: Sized {
    fn UnstoredBufferLength(&mut self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&mut self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&mut self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&mut self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&mut self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn WriteByte(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn WriteBytes(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WriteBuffer(&mut self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<()>;
    fn WriteBufferRange(&mut self, buffer: &::core::option::Option<IBuffer>, start: u32, count: u32) -> ::windows::core::Result<()>;
    fn WriteBoolean(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn WriteGuid(&mut self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn WriteInt16(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn WriteInt32(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn WriteInt64(&mut self, value: i64) -> ::windows::core::Result<()>;
    fn WriteUInt16(&mut self, value: u16) -> ::windows::core::Result<()>;
    fn WriteUInt32(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn WriteUInt64(&mut self, value: u64) -> ::windows::core::Result<()>;
    fn WriteSingle(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn WriteDouble(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn WriteDateTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn WriteTimeSpan(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn WriteString(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32>;
    fn MeasureString(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32>;
    fn StoreAsync(&mut self) -> ::windows::core::Result<DataWriterStoreOperation>;
    fn FlushAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachBuffer(&mut self) -> ::windows::core::Result<IBuffer>;
    fn DetachStream(&mut self) -> ::windows::core::Result<IOutputStream>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriter";
}
#[cfg(feature = "Foundation")]
impl IDataWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataWriter_Vtbl {
        unsafe extern "system" fn UnstoredBufferLength<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnstoredBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicodeEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnicodeEncoding(value).into()
        }
        unsafe extern "system" fn ByteOrder<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ByteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetByteOrder(value).into()
        }
        unsafe extern "system" fn WriteByte<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteByte(value).into()
        }
        unsafe extern "system" fn WriteBytes<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBytes(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn WriteBuffer<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBuffer(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteBufferRange<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, start: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBufferRange(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType), start, count).into()
        }
        unsafe extern "system" fn WriteBoolean<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBoolean(value).into()
        }
        unsafe extern "system" fn WriteGuid<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteGuid(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteInt16<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteInt16(value).into()
        }
        unsafe extern "system" fn WriteInt32<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteInt32(value).into()
        }
        unsafe extern "system" fn WriteInt64<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteInt64(value).into()
        }
        unsafe extern "system" fn WriteUInt16<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteUInt16(value).into()
        }
        unsafe extern "system" fn WriteUInt32<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteUInt32(value).into()
        }
        unsafe extern "system" fn WriteUInt64<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteUInt64(value).into()
        }
        unsafe extern "system" fn WriteSingle<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteSingle(value).into()
        }
        unsafe extern "system" fn WriteDouble<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteDouble(value).into()
        }
        unsafe extern "system" fn WriteDateTime<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteDateTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteTimeSpan<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteTimeSpan(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteString<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureString<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasureString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreAsync<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetachBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataWriter, BASE_OFFSET>(),
            UnstoredBufferLength: UnstoredBufferLength::<Impl, IMPL_OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Impl, IMPL_OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Impl, IMPL_OFFSET>,
            ByteOrder: ByteOrder::<Impl, IMPL_OFFSET>,
            SetByteOrder: SetByteOrder::<Impl, IMPL_OFFSET>,
            WriteByte: WriteByte::<Impl, IMPL_OFFSET>,
            WriteBytes: WriteBytes::<Impl, IMPL_OFFSET>,
            WriteBuffer: WriteBuffer::<Impl, IMPL_OFFSET>,
            WriteBufferRange: WriteBufferRange::<Impl, IMPL_OFFSET>,
            WriteBoolean: WriteBoolean::<Impl, IMPL_OFFSET>,
            WriteGuid: WriteGuid::<Impl, IMPL_OFFSET>,
            WriteInt16: WriteInt16::<Impl, IMPL_OFFSET>,
            WriteInt32: WriteInt32::<Impl, IMPL_OFFSET>,
            WriteInt64: WriteInt64::<Impl, IMPL_OFFSET>,
            WriteUInt16: WriteUInt16::<Impl, IMPL_OFFSET>,
            WriteUInt32: WriteUInt32::<Impl, IMPL_OFFSET>,
            WriteUInt64: WriteUInt64::<Impl, IMPL_OFFSET>,
            WriteSingle: WriteSingle::<Impl, IMPL_OFFSET>,
            WriteDouble: WriteDouble::<Impl, IMPL_OFFSET>,
            WriteDateTime: WriteDateTime::<Impl, IMPL_OFFSET>,
            WriteTimeSpan: WriteTimeSpan::<Impl, IMPL_OFFSET>,
            WriteString: WriteString::<Impl, IMPL_OFFSET>,
            MeasureString: MeasureString::<Impl, IMPL_OFFSET>,
            StoreAsync: StoreAsync::<Impl, IMPL_OFFSET>,
            FlushAsync: FlushAsync::<Impl, IMPL_OFFSET>,
            DetachBuffer: DetachBuffer::<Impl, IMPL_OFFSET>,
            DetachStream: DetachStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataWriterFactory_Impl: Sized {
    fn CreateDataWriter(&mut self, outputstream: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<DataWriter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataWriterFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataWriterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataWriterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataWriterFactory_Vtbl {
        unsafe extern "system" fn CreateDataWriter<Impl: IDataWriterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDataWriter(&*(&outputstream as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataWriterFactory, BASE_OFFSET>(),
            CreateDataWriter: CreateDataWriter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataWriterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IFileRandomAccessStreamStatics_Impl: Sized {
    fn OpenAsync(&mut self, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenWithOptionsAsync(&mut self, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&mut self, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenTransactedWriteWithOptionsAsync(&mut self, filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenForUserAsync(&mut self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenForUserWithOptionsAsync(&mut self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenTransactedWriteForUserAsync(&mut self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenTransactedWriteForUserWithOptionsAsync(&mut self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileRandomAccessStreamStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IFileRandomAccessStreamStatics";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IFileRandomAccessStreamStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileRandomAccessStreamStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileRandomAccessStreamStatics_Vtbl {
        unsafe extern "system" fn OpenAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenWithOptionsAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenWithOptionsAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode, sharingoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteWithOptionsAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteWithOptionsAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), openoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenForUserAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenForUserWithOptionsAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenForUserWithOptionsAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode, sharingoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteForUserAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteForUserWithOptionsAsync<Impl: IFileRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteForUserWithOptionsAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), openoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileRandomAccessStreamStatics, BASE_OFFSET>(),
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
            OpenWithOptionsAsync: OpenWithOptionsAsync::<Impl, IMPL_OFFSET>,
            OpenTransactedWriteAsync: OpenTransactedWriteAsync::<Impl, IMPL_OFFSET>,
            OpenTransactedWriteWithOptionsAsync: OpenTransactedWriteWithOptionsAsync::<Impl, IMPL_OFFSET>,
            OpenForUserAsync: OpenForUserAsync::<Impl, IMPL_OFFSET>,
            OpenForUserWithOptionsAsync: OpenForUserWithOptionsAsync::<Impl, IMPL_OFFSET>,
            OpenTransactedWriteForUserAsync: OpenTransactedWriteForUserAsync::<Impl, IMPL_OFFSET>,
            OpenTransactedWriteForUserWithOptionsAsync: OpenTransactedWriteForUserWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileRandomAccessStreamStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IInputStream_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn ReadAsync(&mut self, buffer: &::core::option::Option<IBuffer>, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStream";
}
#[cfg(feature = "Foundation")]
impl IInputStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputStream_Vtbl {
        unsafe extern "system" fn ReadAsync<Impl: IInputStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, count: u32, options: InputStreamOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadAsync(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType), count, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInputStream, BASE_OFFSET>(), ReadAsync: ReadAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IInputStreamReference_Impl: Sized {
    fn OpenSequentialReadAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IInputStream>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInputStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStreamReference";
}
#[cfg(feature = "Foundation")]
impl IInputStreamReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputStreamReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputStreamReference_Vtbl {
        unsafe extern "system" fn OpenSequentialReadAsync<Impl: IInputStreamReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSequentialReadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputStreamReference, BASE_OFFSET>(),
            OpenSequentialReadAsync: OpenSequentialReadAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputStreamReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IOutputStream_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn WriteAsync(&mut self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
}
#[cfg(feature = "Foundation")]
impl IOutputStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOutputStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOutputStream_Vtbl {
        unsafe extern "system" fn WriteAsync<Impl: IOutputStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteAsync(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Impl: IOutputStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOutputStream, BASE_OFFSET>(),
            WriteAsync: WriteAsync::<Impl, IMPL_OFFSET>,
            FlushAsync: FlushAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOutputStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPropertySetSerializer_Impl: Sized {
    fn Serialize(&mut self, propertyset: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<IBuffer>;
    fn Deserialize(&mut self, propertyset: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPropertySetSerializer {
    const NAME: &'static str = "Windows.Storage.Streams.IPropertySetSerializer";
}
#[cfg(feature = "Foundation_Collections")]
impl IPropertySetSerializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetSerializer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySetSerializer_Vtbl {
        unsafe extern "system" fn Serialize<Impl: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&propertyset as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deserialize<Impl: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyset: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deserialize(&*(&propertyset as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType), &*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertySetSerializer, BASE_OFFSET>(),
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
            Deserialize: Deserialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySetSerializer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStream_Impl: Sized + super::super::Foundation::IClosable_Impl + IInputStream_Impl + IOutputStream_Impl {
    fn Size(&mut self) -> ::windows::core::Result<u64>;
    fn SetSize(&mut self, value: u64) -> ::windows::core::Result<()>;
    fn GetInputStreamAt(&mut self, position: u64) -> ::windows::core::Result<IInputStream>;
    fn GetOutputStreamAt(&mut self, position: u64) -> ::windows::core::Result<IOutputStream>;
    fn Position(&mut self) -> ::windows::core::Result<u64>;
    fn Seek(&mut self, position: u64) -> ::windows::core::Result<()>;
    fn CloneStream(&mut self) -> ::windows::core::Result<IRandomAccessStream>;
    fn CanRead(&mut self) -> ::windows::core::Result<bool>;
    fn CanWrite(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStream";
}
#[cfg(feature = "Foundation")]
impl IRandomAccessStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStream_Vtbl {
        unsafe extern "system" fn Size<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(value).into()
        }
        unsafe extern "system" fn GetInputStreamAt<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAt<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(position).into()
        }
        unsafe extern "system" fn CloneStream<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloneStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanWrite<Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRandomAccessStream, BASE_OFFSET>(),
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            GetInputStreamAt: GetInputStreamAt::<Impl, IMPL_OFFSET>,
            GetOutputStreamAt: GetOutputStreamAt::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            CloneStream: CloneStream::<Impl, IMPL_OFFSET>,
            CanRead: CanRead::<Impl, IMPL_OFFSET>,
            CanWrite: CanWrite::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamReference_Impl: Sized {
    fn OpenReadAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IRandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
}
#[cfg(feature = "Foundation")]
impl IRandomAccessStreamReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStreamReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStreamReference_Vtbl {
        unsafe extern "system" fn OpenReadAsync<Impl: IRandomAccessStreamReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenReadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRandomAccessStreamReference, BASE_OFFSET>(),
            OpenReadAsync: OpenReadAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamReference as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRandomAccessStreamReferenceStatics_Impl: Sized {
    fn CreateFromFile(&mut self, file: &::core::option::Option<super::IStorageFile>) -> ::windows::core::Result<RandomAccessStreamReference>;
    fn CreateFromUri(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<RandomAccessStreamReference>;
    fn CreateFromStream(&mut self, stream: &::core::option::Option<IRandomAccessStream>) -> ::windows::core::Result<RandomAccessStreamReference>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRandomAccessStreamReferenceStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReferenceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRandomAccessStreamReferenceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStreamReferenceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStreamReferenceStatics_Vtbl {
        unsafe extern "system" fn CreateFromFile<Impl: IRandomAccessStreamReferenceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromFile(&*(&file as *const <super::IStorageFile as ::windows::core::Abi>::Abi as *const <super::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUri<Impl: IRandomAccessStreamReferenceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStream<Impl: IRandomAccessStreamReferenceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStream(&*(&stream as *const <IRandomAccessStream as ::windows::core::Abi>::Abi as *const <IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRandomAccessStreamReferenceStatics, BASE_OFFSET>(),
            CreateFromFile: CreateFromFile::<Impl, IMPL_OFFSET>,
            CreateFromUri: CreateFromUri::<Impl, IMPL_OFFSET>,
            CreateFromStream: CreateFromStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamReferenceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRandomAccessStreamStatics_Impl: Sized {
    fn CopyAsync(&mut self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn CopySizeAsync(&mut self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>, bytestocopy: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn CopyAndCloseAsync(&mut self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRandomAccessStreamStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRandomAccessStreamStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStreamStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStreamStatics_Vtbl {
        unsafe extern "system" fn CopyAsync<Impl: IRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyAsync(&*(&source as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopySizeAsync<Impl: IRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, bytestocopy: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopySizeAsync(&*(&source as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType), bytestocopy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndCloseAsync<Impl: IRandomAccessStreamStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyAndCloseAsync(&*(&source as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRandomAccessStreamStatics, BASE_OFFSET>(),
            CopyAsync: CopyAsync::<Impl, IMPL_OFFSET>,
            CopySizeAsync: CopySizeAsync::<Impl, IMPL_OFFSET>,
            CopyAndCloseAsync: CopyAndCloseAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamWithContentType_Impl: Sized + super::super::Foundation::IClosable_Impl + IContentTypeProvider_Impl + IInputStream_Impl + IOutputStream_Impl + IRandomAccessStream_Impl {}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IRandomAccessStreamWithContentType {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamWithContentType";
}
#[cfg(feature = "Foundation")]
impl IRandomAccessStreamWithContentType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStreamWithContentType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStreamWithContentType_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRandomAccessStreamWithContentType, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamWithContentType as ::windows::core::Interface>::IID
    }
}
