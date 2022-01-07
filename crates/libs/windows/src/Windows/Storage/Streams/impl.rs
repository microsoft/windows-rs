pub trait IBufferImpl: Sized {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, value: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBuffer {
    const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
}
impl IBufferVtbl {
    pub const fn new<Impl: IBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBufferVtbl {
        unsafe extern "system" fn Capacity<Impl: IBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: IBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBuffer>, base.5, Capacity::<Impl, OFFSET>, Length::<Impl, OFFSET>, SetLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBufferFactoryImpl: Sized {
    fn Create(&self, capacity: u32) -> ::windows::core::Result<Buffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBufferFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IBufferFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBufferFactoryVtbl {
    pub const fn new<Impl: IBufferFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBufferFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBufferFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(capacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBufferFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBufferStaticsImpl: Sized {
    fn CreateCopyFromMemoryBuffer(&self, input: &::core::option::Option<super::super::Foundation::IMemoryBuffer>) -> ::windows::core::Result<Buffer>;
    fn CreateMemoryBufferOverIBuffer(&self, input: &::core::option::Option<IBuffer>) -> ::windows::core::Result<super::super::Foundation::MemoryBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBufferStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IBufferStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBufferStaticsVtbl {
    pub const fn new<Impl: IBufferStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBufferStaticsVtbl {
        unsafe extern "system" fn CreateCopyFromMemoryBuffer<Impl: IBufferStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCopyFromMemoryBuffer(&*(&input as *const <super::super::Foundation::IMemoryBuffer as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IMemoryBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMemoryBufferOverIBuffer<Impl: IBufferStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, input: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMemoryBufferOverIBuffer(&*(&input as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBufferStatics>, base.5, CreateCopyFromMemoryBuffer::<Impl, OFFSET>, CreateMemoryBufferOverIBuffer::<Impl, OFFSET>)
    }
}
pub trait IContentTypeProviderImpl: Sized {
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContentTypeProvider {
    const NAME: &'static str = "Windows.Storage.Streams.IContentTypeProvider";
}
impl IContentTypeProviderVtbl {
    pub const fn new<Impl: IContentTypeProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContentTypeProviderVtbl {
        unsafe extern "system" fn ContentType<Impl: IContentTypeProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContentTypeProvider>, base.5, ContentType::<Impl, OFFSET>)
    }
}
pub trait IDataReaderImpl: Sized {
    fn UnconsumedBufferLength(&self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn InputStreamOptions(&self) -> ::windows::core::Result<InputStreamOptions>;
    fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::core::Result<()>;
    fn ReadByte(&self) -> ::windows::core::Result<u8>;
    fn ReadBytes(&self, value: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ReadBuffer(&self, length: u32) -> ::windows::core::Result<IBuffer>;
    fn ReadBoolean(&self) -> ::windows::core::Result<bool>;
    fn ReadGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReadInt16(&self) -> ::windows::core::Result<i16>;
    fn ReadInt32(&self) -> ::windows::core::Result<i32>;
    fn ReadInt64(&self) -> ::windows::core::Result<i64>;
    fn ReadUInt16(&self) -> ::windows::core::Result<u16>;
    fn ReadUInt32(&self) -> ::windows::core::Result<u32>;
    fn ReadUInt64(&self) -> ::windows::core::Result<u64>;
    fn ReadSingle(&self) -> ::windows::core::Result<f32>;
    fn ReadDouble(&self) -> ::windows::core::Result<f64>;
    fn ReadString(&self, codeunitcount: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadDateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ReadTimeSpan(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn LoadAsync(&self, count: u32) -> ::windows::core::Result<DataReaderLoadOperation>;
    fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer>;
    fn DetachStream(&self) -> ::windows::core::Result<IInputStream>;
}
impl ::windows::core::RuntimeName for IDataReader {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReader";
}
impl IDataReaderVtbl {
    pub const fn new<Impl: IDataReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataReaderVtbl {
        unsafe extern "system" fn UnconsumedBufferLength<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnconsumedBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnicodeEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUnicodeEncoding(value).into()
        }
        unsafe extern "system" fn ByteOrder<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ByteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetByteOrder(value).into()
        }
        unsafe extern "system" fn InputStreamOptions<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InputStreamOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputStreamOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputStreamOptions<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InputStreamOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInputStreamOptions(value).into()
        }
        unsafe extern "system" fn ReadByte<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadByte() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBytes<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReadBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn ReadBuffer<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadBuffer(length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBoolean<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadBoolean() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadGuid<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt16<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt32<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt64<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt16<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadUInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt32<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadUInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt64<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadUInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSingle<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadSingle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDouble<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadDouble() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadString<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, codeunitcount: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadString(codeunitcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDateTime<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTimeSpan<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadTimeSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetachBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Impl: IDataReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDataReader>,
            base.5,
            UnconsumedBufferLength::<Impl, OFFSET>,
            UnicodeEncoding::<Impl, OFFSET>,
            SetUnicodeEncoding::<Impl, OFFSET>,
            ByteOrder::<Impl, OFFSET>,
            SetByteOrder::<Impl, OFFSET>,
            InputStreamOptions::<Impl, OFFSET>,
            SetInputStreamOptions::<Impl, OFFSET>,
            ReadByte::<Impl, OFFSET>,
            ReadBytes::<Impl, OFFSET>,
            ReadBuffer::<Impl, OFFSET>,
            ReadBoolean::<Impl, OFFSET>,
            ReadGuid::<Impl, OFFSET>,
            ReadInt16::<Impl, OFFSET>,
            ReadInt32::<Impl, OFFSET>,
            ReadInt64::<Impl, OFFSET>,
            ReadUInt16::<Impl, OFFSET>,
            ReadUInt32::<Impl, OFFSET>,
            ReadUInt64::<Impl, OFFSET>,
            ReadSingle::<Impl, OFFSET>,
            ReadDouble::<Impl, OFFSET>,
            ReadString::<Impl, OFFSET>,
            ReadDateTime::<Impl, OFFSET>,
            ReadTimeSpan::<Impl, OFFSET>,
            LoadAsync::<Impl, OFFSET>,
            DetachBuffer::<Impl, OFFSET>,
            DetachStream::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataReaderFactoryImpl: Sized {
    fn CreateDataReader(&self, inputstream: &::core::option::Option<IInputStream>) -> ::windows::core::Result<DataReader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataReaderFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReaderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataReaderFactoryVtbl {
    pub const fn new<Impl: IDataReaderFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataReaderFactoryVtbl {
        unsafe extern "system" fn CreateDataReader<Impl: IDataReaderFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDataReader(&*(&inputstream as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataReaderFactory>, base.5, CreateDataReader::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataReaderStaticsImpl: Sized {
    fn FromBuffer(&self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<DataReader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataReaderStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDataReaderStaticsVtbl {
    pub const fn new<Impl: IDataReaderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataReaderStaticsVtbl {
        unsafe extern "system" fn FromBuffer<Impl: IDataReaderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromBuffer(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataReaderStatics>, base.5, FromBuffer::<Impl, OFFSET>)
    }
}
pub trait IDataWriterImpl: Sized {
    fn UnstoredBufferLength(&self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn WriteByte(&self, value: u8) -> ::windows::core::Result<()>;
    fn WriteBytes(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WriteBuffer(&self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<()>;
    fn WriteBufferRange(&self, buffer: &::core::option::Option<IBuffer>, start: u32, count: u32) -> ::windows::core::Result<()>;
    fn WriteBoolean(&self, value: bool) -> ::windows::core::Result<()>;
    fn WriteGuid(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn WriteInt16(&self, value: i16) -> ::windows::core::Result<()>;
    fn WriteInt32(&self, value: i32) -> ::windows::core::Result<()>;
    fn WriteInt64(&self, value: i64) -> ::windows::core::Result<()>;
    fn WriteUInt16(&self, value: u16) -> ::windows::core::Result<()>;
    fn WriteUInt32(&self, value: u32) -> ::windows::core::Result<()>;
    fn WriteUInt64(&self, value: u64) -> ::windows::core::Result<()>;
    fn WriteSingle(&self, value: f32) -> ::windows::core::Result<()>;
    fn WriteDouble(&self, value: f64) -> ::windows::core::Result<()>;
    fn WriteDateTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn WriteTimeSpan(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn WriteString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32>;
    fn MeasureString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32>;
    fn StoreAsync(&self) -> ::windows::core::Result<DataWriterStoreOperation>;
    fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer>;
    fn DetachStream(&self) -> ::windows::core::Result<IOutputStream>;
}
impl ::windows::core::RuntimeName for IDataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriter";
}
impl IDataWriterVtbl {
    pub const fn new<Impl: IDataWriterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataWriterVtbl {
        unsafe extern "system" fn UnstoredBufferLength<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnstoredBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnicodeEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUnicodeEncoding(value).into()
        }
        unsafe extern "system" fn ByteOrder<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ByteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetByteOrder(value).into()
        }
        unsafe extern "system" fn WriteByte<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteByte(value).into()
        }
        unsafe extern "system" fn WriteBytes<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteBytes(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn WriteBuffer<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteBuffer(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteBufferRange<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, start: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteBufferRange(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType), start, count).into()
        }
        unsafe extern "system" fn WriteBoolean<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteBoolean(value).into()
        }
        unsafe extern "system" fn WriteGuid<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteGuid(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteInt16<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteInt16(value).into()
        }
        unsafe extern "system" fn WriteInt32<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteInt32(value).into()
        }
        unsafe extern "system" fn WriteInt64<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteInt64(value).into()
        }
        unsafe extern "system" fn WriteUInt16<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteUInt16(value).into()
        }
        unsafe extern "system" fn WriteUInt32<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteUInt32(value).into()
        }
        unsafe extern "system" fn WriteUInt64<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteUInt64(value).into()
        }
        unsafe extern "system" fn WriteSingle<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteSingle(value).into()
        }
        unsafe extern "system" fn WriteDouble<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteDouble(value).into()
        }
        unsafe extern "system" fn WriteDateTime<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteDateTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteTimeSpan<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WriteTimeSpan(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteString<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureString<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MeasureString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreAsync<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetachBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Impl: IDataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDataWriter>,
            base.5,
            UnstoredBufferLength::<Impl, OFFSET>,
            UnicodeEncoding::<Impl, OFFSET>,
            SetUnicodeEncoding::<Impl, OFFSET>,
            ByteOrder::<Impl, OFFSET>,
            SetByteOrder::<Impl, OFFSET>,
            WriteByte::<Impl, OFFSET>,
            WriteBytes::<Impl, OFFSET>,
            WriteBuffer::<Impl, OFFSET>,
            WriteBufferRange::<Impl, OFFSET>,
            WriteBoolean::<Impl, OFFSET>,
            WriteGuid::<Impl, OFFSET>,
            WriteInt16::<Impl, OFFSET>,
            WriteInt32::<Impl, OFFSET>,
            WriteInt64::<Impl, OFFSET>,
            WriteUInt16::<Impl, OFFSET>,
            WriteUInt32::<Impl, OFFSET>,
            WriteUInt64::<Impl, OFFSET>,
            WriteSingle::<Impl, OFFSET>,
            WriteDouble::<Impl, OFFSET>,
            WriteDateTime::<Impl, OFFSET>,
            WriteTimeSpan::<Impl, OFFSET>,
            WriteString::<Impl, OFFSET>,
            MeasureString::<Impl, OFFSET>,
            StoreAsync::<Impl, OFFSET>,
            FlushAsync::<Impl, OFFSET>,
            DetachBuffer::<Impl, OFFSET>,
            DetachStream::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataWriterFactoryImpl: Sized {
    fn CreateDataWriter(&self, outputstream: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<DataWriter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataWriterFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataWriterFactoryVtbl {
    pub const fn new<Impl: IDataWriterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataWriterFactoryVtbl {
        unsafe extern "system" fn CreateDataWriter<Impl: IDataWriterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDataWriter(&*(&outputstream as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataWriterFactory>, base.5, CreateDataWriter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileRandomAccessStreamStaticsImpl: Sized {
    fn OpenAsync(&self, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenWithOptionsAsync(&self, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&self, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenTransactedWriteWithOptionsAsync(&self, filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenForUserWithOptionsAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenTransactedWriteForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenTransactedWriteForUserWithOptionsAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileRandomAccessStreamStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IFileRandomAccessStreamStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFileRandomAccessStreamStaticsVtbl {
    pub const fn new<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFileRandomAccessStreamStaticsVtbl {
        unsafe extern "system" fn OpenAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenWithOptionsAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenWithOptionsAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode, sharingoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteWithOptionsAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteWithOptionsAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), openoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenForUserAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenForUserWithOptionsAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenForUserWithOptionsAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), accessmode, sharingoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteForUserAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteForUserWithOptionsAsync<Impl: IFileRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteForUserWithOptionsAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), openoptions, opendisposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFileRandomAccessStreamStatics>, base.5, OpenAsync::<Impl, OFFSET>, OpenWithOptionsAsync::<Impl, OFFSET>, OpenTransactedWriteAsync::<Impl, OFFSET>, OpenTransactedWriteWithOptionsAsync::<Impl, OFFSET>, OpenForUserAsync::<Impl, OFFSET>, OpenForUserWithOptionsAsync::<Impl, OFFSET>, OpenTransactedWriteForUserAsync::<Impl, OFFSET>, OpenTransactedWriteForUserWithOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IInputStreamImpl: Sized + IClosableImpl {
    fn ReadAsync(&self, buffer: &::core::option::Option<IBuffer>, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStream";
}
#[cfg(feature = "Foundation")]
impl IInputStreamVtbl {
    pub const fn new<Impl: IInputStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputStreamVtbl {
        unsafe extern "system" fn ReadAsync<Impl: IInputStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, count: u32, options: InputStreamOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadAsync(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType), count, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputStream>, base.5, ReadAsync::<Impl, OFFSET>)
    }
}
pub trait IInputStreamReferenceImpl: Sized {
    fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IInputStream>>;
}
impl ::windows::core::RuntimeName for IInputStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStreamReference";
}
impl IInputStreamReferenceVtbl {
    pub const fn new<Impl: IInputStreamReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputStreamReferenceVtbl {
        unsafe extern "system" fn OpenSequentialReadAsync<Impl: IInputStreamReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenSequentialReadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputStreamReference>, base.5, OpenSequentialReadAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IOutputStreamImpl: Sized + IClosableImpl {
    fn WriteAsync(&self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
}
#[cfg(feature = "Foundation")]
impl IOutputStreamVtbl {
    pub const fn new<Impl: IOutputStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOutputStreamVtbl {
        unsafe extern "system" fn WriteAsync<Impl: IOutputStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteAsync(&*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Impl: IOutputStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOutputStream>, base.5, WriteAsync::<Impl, OFFSET>, FlushAsync::<Impl, OFFSET>)
    }
}
pub trait IPropertySetSerializerImpl: Sized {
    fn Serialize(&self, propertyset: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<IBuffer>;
    fn Deserialize(&self, propertyset: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPropertySetSerializer {
    const NAME: &'static str = "Windows.Storage.Streams.IPropertySetSerializer";
}
impl IPropertySetSerializerVtbl {
    pub const fn new<Impl: IPropertySetSerializerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertySetSerializerVtbl {
        unsafe extern "system" fn Serialize<Impl: IPropertySetSerializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&propertyset as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deserialize<Impl: IPropertySetSerializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyset: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Deserialize(&*(&propertyset as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType), &*(&buffer as *const <IBuffer as ::windows::core::Abi>::Abi as *const <IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertySetSerializer>, base.5, Serialize::<Impl, OFFSET>, Deserialize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamImpl: Sized + IClosableImpl + IInputStreamImpl + IOutputStreamImpl {
    fn Size(&self) -> ::windows::core::Result<u64>;
    fn SetSize(&self, value: u64) -> ::windows::core::Result<()>;
    fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream>;
    fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream>;
    fn Position(&self) -> ::windows::core::Result<u64>;
    fn Seek(&self, position: u64) -> ::windows::core::Result<()>;
    fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream>;
    fn CanRead(&self) -> ::windows::core::Result<bool>;
    fn CanWrite(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStream";
}
#[cfg(feature = "Foundation")]
impl IRandomAccessStreamVtbl {
    pub const fn new<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRandomAccessStreamVtbl {
        unsafe extern "system" fn Size<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSize(value).into()
        }
        unsafe extern "system" fn GetInputStreamAt<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAt<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Seek(position).into()
        }
        unsafe extern "system" fn CloneStream<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloneStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanWrite<Impl: IRandomAccessStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRandomAccessStream>, base.5, Size::<Impl, OFFSET>, SetSize::<Impl, OFFSET>, GetInputStreamAt::<Impl, OFFSET>, GetOutputStreamAt::<Impl, OFFSET>, Position::<Impl, OFFSET>, Seek::<Impl, OFFSET>, CloneStream::<Impl, OFFSET>, CanRead::<Impl, OFFSET>, CanWrite::<Impl, OFFSET>)
    }
}
pub trait IRandomAccessStreamReferenceImpl: Sized {
    fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
impl ::windows::core::RuntimeName for IRandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
}
impl IRandomAccessStreamReferenceVtbl {
    pub const fn new<Impl: IRandomAccessStreamReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRandomAccessStreamReferenceVtbl {
        unsafe extern "system" fn OpenReadAsync<Impl: IRandomAccessStreamReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenReadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRandomAccessStreamReference>, base.5, OpenReadAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRandomAccessStreamReferenceStaticsImpl: Sized {
    fn CreateFromFile(&self, file: &::core::option::Option<super::IStorageFile>) -> ::windows::core::Result<RandomAccessStreamReference>;
    fn CreateFromUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<RandomAccessStreamReference>;
    fn CreateFromStream(&self, stream: &::core::option::Option<IRandomAccessStream>) -> ::windows::core::Result<RandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRandomAccessStreamReferenceStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReferenceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRandomAccessStreamReferenceStaticsVtbl {
    pub const fn new<Impl: IRandomAccessStreamReferenceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRandomAccessStreamReferenceStaticsVtbl {
        unsafe extern "system" fn CreateFromFile<Impl: IRandomAccessStreamReferenceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromFile(&*(&file as *const <super::IStorageFile as ::windows::core::Abi>::Abi as *const <super::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUri<Impl: IRandomAccessStreamReferenceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStream<Impl: IRandomAccessStreamReferenceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStream(&*(&stream as *const <IRandomAccessStream as ::windows::core::Abi>::Abi as *const <IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRandomAccessStreamReferenceStatics>, base.5, CreateFromFile::<Impl, OFFSET>, CreateFromUri::<Impl, OFFSET>, CreateFromStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRandomAccessStreamStaticsImpl: Sized {
    fn CopyAsync(&self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn CopySizeAsync(&self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>, bytestocopy: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn CopyAndCloseAsync(&self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRandomAccessStreamStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRandomAccessStreamStaticsVtbl {
    pub const fn new<Impl: IRandomAccessStreamStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRandomAccessStreamStaticsVtbl {
        unsafe extern "system" fn CopyAsync<Impl: IRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyAsync(&*(&source as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopySizeAsync<Impl: IRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, bytestocopy: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopySizeAsync(&*(&source as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType), bytestocopy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndCloseAsync<Impl: IRandomAccessStreamStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyAndCloseAsync(&*(&source as *const <IInputStream as ::windows::core::Abi>::Abi as *const <IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&destination as *const <IOutputStream as ::windows::core::Abi>::Abi as *const <IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRandomAccessStreamStatics>, base.5, CopyAsync::<Impl, OFFSET>, CopySizeAsync::<Impl, OFFSET>, CopyAndCloseAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamWithContentTypeImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl {}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IRandomAccessStreamWithContentType {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamWithContentType";
}
#[cfg(feature = "Foundation")]
impl IRandomAccessStreamWithContentTypeVtbl {
    pub const fn new<Impl: IRandomAccessStreamWithContentTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRandomAccessStreamWithContentTypeVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRandomAccessStreamWithContentType>, base.5)
    }
}
