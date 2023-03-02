#[doc = "*Required features: `\"Storage_Streams\"`, `\"implement\"`*"]
pub trait IBuffer_Impl: Sized {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, value: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBuffer {
    const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
}
impl IBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>() -> IBuffer_Vtbl {
        unsafe extern "system" fn Capacity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLength(value).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IBuffer, OFFSET>(),
            Capacity: Capacity::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBuffer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"implement\"`*"]
pub trait IContentTypeProvider_Impl: Sized {
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContentTypeProvider {
    const NAME: &'static str = "Windows.Storage.Streams.IContentTypeProvider";
}
impl IContentTypeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContentTypeProvider_Impl, const OFFSET: isize>() -> IContentTypeProvider_Vtbl {
        unsafe extern "system" fn ContentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContentTypeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IContentTypeProvider, OFFSET>(), ContentType: ContentType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentTypeProvider as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IDataReader_Impl: Sized {
    fn UnconsumedBufferLength(&self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn InputStreamOptions(&self) -> ::windows::core::Result<InputStreamOptions>;
    fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::core::Result<()>;
    fn ReadByte(&self) -> ::windows::core::Result<u8>;
    fn ReadBytes(&self, value: &mut [u8]) -> ::windows::core::Result<()>;
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
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDataReader {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReader";
}
#[cfg(feature = "Foundation")]
impl IDataReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>() -> IDataReader_Vtbl {
        unsafe extern "system" fn UnconsumedBufferLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnconsumedBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnicodeEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnicodeEncoding(value).into()
        }
        unsafe extern "system" fn ByteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ByteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetByteOrder(value).into()
        }
        unsafe extern "system" fn InputStreamOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InputStreamOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InputStreamOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputStreamOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InputStreamOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputStreamOptions(value).into()
        }
        unsafe extern "system" fn ReadByte<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadByte() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBytes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn ReadBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadBuffer(length) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBoolean<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadBoolean() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadInt16() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt32<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadInt32() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt64<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadInt64() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadUInt16() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt32<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadUInt32() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt64<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadUInt64() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSingle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadSingle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDouble<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadDouble() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codeunitcount: u32, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadString(codeunitcount) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDateTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTimeSpan<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadTimeSpan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DetachBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDataReader, OFFSET>(),
            UnconsumedBufferLength: UnconsumedBufferLength::<Identity, Impl, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, Impl, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, Impl, OFFSET>,
            ByteOrder: ByteOrder::<Identity, Impl, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, Impl, OFFSET>,
            InputStreamOptions: InputStreamOptions::<Identity, Impl, OFFSET>,
            SetInputStreamOptions: SetInputStreamOptions::<Identity, Impl, OFFSET>,
            ReadByte: ReadByte::<Identity, Impl, OFFSET>,
            ReadBytes: ReadBytes::<Identity, Impl, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, Impl, OFFSET>,
            ReadBoolean: ReadBoolean::<Identity, Impl, OFFSET>,
            ReadGuid: ReadGuid::<Identity, Impl, OFFSET>,
            ReadInt16: ReadInt16::<Identity, Impl, OFFSET>,
            ReadInt32: ReadInt32::<Identity, Impl, OFFSET>,
            ReadInt64: ReadInt64::<Identity, Impl, OFFSET>,
            ReadUInt16: ReadUInt16::<Identity, Impl, OFFSET>,
            ReadUInt32: ReadUInt32::<Identity, Impl, OFFSET>,
            ReadUInt64: ReadUInt64::<Identity, Impl, OFFSET>,
            ReadSingle: ReadSingle::<Identity, Impl, OFFSET>,
            ReadDouble: ReadDouble::<Identity, Impl, OFFSET>,
            ReadString: ReadString::<Identity, Impl, OFFSET>,
            ReadDateTime: ReadDateTime::<Identity, Impl, OFFSET>,
            ReadTimeSpan: ReadTimeSpan::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
            DetachStream: DetachStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IDataWriter_Impl: Sized {
    fn UnstoredBufferLength(&self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn WriteByte(&self, value: u8) -> ::windows::core::Result<()>;
    fn WriteBytes(&self, value: &[u8]) -> ::windows::core::Result<()>;
    fn WriteBuffer(&self, buffer: ::core::option::Option<&IBuffer>) -> ::windows::core::Result<()>;
    fn WriteBufferRange(&self, buffer: ::core::option::Option<&IBuffer>, start: u32, count: u32) -> ::windows::core::Result<()>;
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
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriter";
}
#[cfg(feature = "Foundation")]
impl IDataWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>() -> IDataWriter_Vtbl {
        unsafe extern "system" fn UnstoredBufferLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnstoredBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnicodeEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnicodeEncoding(value).into()
        }
        unsafe extern "system" fn ByteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ByteOrder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetByteOrder(value).into()
        }
        unsafe extern "system" fn WriteByte<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteByte(value).into()
        }
        unsafe extern "system" fn WriteBytes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteBytes(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn WriteBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteBuffer(::windows::core::from_raw_borrowed(&buffer)).into()
        }
        unsafe extern "system" fn WriteBufferRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, start: u32, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteBufferRange(::windows::core::from_raw_borrowed(&buffer), start, count).into()
        }
        unsafe extern "system" fn WriteBoolean<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteBoolean(value).into()
        }
        unsafe extern "system" fn WriteGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteGuid(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteInt16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteInt16(value).into()
        }
        unsafe extern "system" fn WriteInt32<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteInt32(value).into()
        }
        unsafe extern "system" fn WriteInt64<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteInt64(value).into()
        }
        unsafe extern "system" fn WriteUInt16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteUInt16(value).into()
        }
        unsafe extern "system" fn WriteUInt32<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteUInt32(value).into()
        }
        unsafe extern "system" fn WriteUInt64<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteUInt64(value).into()
        }
        unsafe extern "system" fn WriteSingle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteSingle(value).into()
        }
        unsafe extern "system" fn WriteDouble<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteDouble(value).into()
        }
        unsafe extern "system" fn WriteDateTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteDateTime(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteTimeSpan<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteTimeSpan(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteString(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MeasureString(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DetachBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDataWriter, OFFSET>(),
            UnstoredBufferLength: UnstoredBufferLength::<Identity, Impl, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, Impl, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, Impl, OFFSET>,
            ByteOrder: ByteOrder::<Identity, Impl, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, Impl, OFFSET>,
            WriteByte: WriteByte::<Identity, Impl, OFFSET>,
            WriteBytes: WriteBytes::<Identity, Impl, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, Impl, OFFSET>,
            WriteBufferRange: WriteBufferRange::<Identity, Impl, OFFSET>,
            WriteBoolean: WriteBoolean::<Identity, Impl, OFFSET>,
            WriteGuid: WriteGuid::<Identity, Impl, OFFSET>,
            WriteInt16: WriteInt16::<Identity, Impl, OFFSET>,
            WriteInt32: WriteInt32::<Identity, Impl, OFFSET>,
            WriteInt64: WriteInt64::<Identity, Impl, OFFSET>,
            WriteUInt16: WriteUInt16::<Identity, Impl, OFFSET>,
            WriteUInt32: WriteUInt32::<Identity, Impl, OFFSET>,
            WriteUInt64: WriteUInt64::<Identity, Impl, OFFSET>,
            WriteSingle: WriteSingle::<Identity, Impl, OFFSET>,
            WriteDouble: WriteDouble::<Identity, Impl, OFFSET>,
            WriteDateTime: WriteDateTime::<Identity, Impl, OFFSET>,
            WriteTimeSpan: WriteTimeSpan::<Identity, Impl, OFFSET>,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            MeasureString: MeasureString::<Identity, Impl, OFFSET>,
            StoreAsync: StoreAsync::<Identity, Impl, OFFSET>,
            FlushAsync: FlushAsync::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
            DetachStream: DetachStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IInputStream_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn ReadAsync(&self, buffer: ::core::option::Option<&IBuffer>, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStream";
}
#[cfg(feature = "Foundation")]
impl IInputStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputStream_Impl, const OFFSET: isize>() -> IInputStream_Vtbl {
        unsafe extern "system" fn ReadAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadAsync(::windows::core::from_raw_borrowed(&buffer), count, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IInputStream, OFFSET>(), ReadAsync: ReadAsync::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputStream as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IInputStreamReference_Impl: Sized {
    fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IInputStream>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IInputStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStreamReference";
}
#[cfg(feature = "Foundation")]
impl IInputStreamReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputStreamReference_Impl, const OFFSET: isize>() -> IInputStreamReference_Vtbl {
        unsafe extern "system" fn OpenSequentialReadAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputStreamReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenSequentialReadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IInputStreamReference, OFFSET>(),
            OpenSequentialReadAsync: OpenSequentialReadAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputStreamReference as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IOutputStream_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn WriteAsync(&self, buffer: ::core::option::Option<&IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
}
#[cfg(feature = "Foundation")]
impl IOutputStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: isize>() -> IOutputStream_Vtbl {
        unsafe extern "system" fn WriteAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteAsync(::windows::core::from_raw_borrowed(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FlushAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IOutputStream, OFFSET>(),
            WriteAsync: WriteAsync::<Identity, Impl, OFFSET>,
            FlushAsync: FlushAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOutputStream as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPropertySetSerializer_Impl: Sized {
    fn Serialize(&self, propertyset: ::core::option::Option<&super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<IBuffer>;
    fn Deserialize(&self, propertyset: ::core::option::Option<&super::super::Foundation::Collections::IPropertySet>, buffer: ::core::option::Option<&IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPropertySetSerializer {
    const NAME: &'static str = "Windows.Storage.Streams.IPropertySetSerializer";
}
#[cfg(feature = "Foundation_Collections")]
impl IPropertySetSerializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: isize>() -> IPropertySetSerializer_Vtbl {
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Serialize(::windows::core::from_raw_borrowed(&propertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deserialize(::windows::core::from_raw_borrowed(&propertyset), ::windows::core::from_raw_borrowed(&buffer)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPropertySetSerializer, OFFSET>(),
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySetSerializer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStream_Impl: Sized + super::super::Foundation::IClosable_Impl + IInputStream_Impl + IOutputStream_Impl {
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
impl IRandomAccessStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>() -> IRandomAccessStream_Vtbl {
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSize(value).into()
        }
        unsafe extern "system" fn GetInputStreamAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Position() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Seek(position).into()
        }
        unsafe extern "system" fn CloneStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CloneStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanWrite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IRandomAccessStream, OFFSET>(),
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            GetInputStreamAt: GetInputStreamAt::<Identity, Impl, OFFSET>,
            GetOutputStreamAt: GetOutputStreamAt::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            CloneStream: CloneStream::<Identity, Impl, OFFSET>,
            CanRead: CanRead::<Identity, Impl, OFFSET>,
            CanWrite: CanWrite::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStream as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamReference_Impl: Sized {
    fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IRandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
}
#[cfg(feature = "Foundation")]
impl IRandomAccessStreamReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamReference_Impl, const OFFSET: isize>() -> IRandomAccessStreamReference_Vtbl {
        unsafe extern "system" fn OpenReadAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenReadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamReference, OFFSET>(),
            OpenReadAsync: OpenReadAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamReference as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamWithContentType_Impl: Sized + super::super::Foundation::IClosable_Impl + IContentTypeProvider_Impl + IInputStream_Impl + IOutputStream_Impl + IRandomAccessStream_Impl {}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IRandomAccessStreamWithContentType {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamWithContentType";
}
#[cfg(feature = "Foundation")]
impl IRandomAccessStreamWithContentType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamWithContentType_Impl, const OFFSET: isize>() -> IRandomAccessStreamWithContentType_Vtbl {
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamWithContentType, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamWithContentType as ::windows::core::ComInterface>::IID
    }
}
