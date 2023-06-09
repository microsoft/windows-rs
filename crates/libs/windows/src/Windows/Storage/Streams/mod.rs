#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IBuffer(::windows_core::IUnknown);
impl IBuffer {
    pub fn Capacity(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Capacity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IBuffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBuffer {}
impl ::core::fmt::Debug for IBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBuffer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{905a0fe0-bc53-11df-8c49-001e4fc686da}");
}
unsafe impl ::windows_core::Interface for IBuffer {
    type Vtable = IBuffer_Vtbl;
}
impl ::core::clone::Clone for IBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905a0fe0_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBufferFactory {
    type Vtable = IBufferFactory_Vtbl;
}
impl ::core::clone::Clone for IBufferFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBufferFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71af914d_c10f_484b_bc50_14bc623b3a27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBufferStatics {
    type Vtable = IBufferStatics_Vtbl;
}
impl ::core::clone::Clone for IBufferStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBufferStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe901e65b_d716_475a_a90a_af7229b1e741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateCopyFromMemoryBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateCopyFromMemoryBuffer: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMemoryBufferOverIBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMemoryBufferOverIBuffer: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IContentTypeProvider(::windows_core::IUnknown);
impl IContentTypeProvider {
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContentTypeProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IContentTypeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentTypeProvider {}
impl ::core::fmt::Debug for IContentTypeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentTypeProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IContentTypeProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{97d098a5-3b99-4de9-88a5-e11d2f50c795}");
}
unsafe impl ::windows_core::Interface for IContentTypeProvider {
    type Vtable = IContentTypeProvider_Vtbl;
}
impl ::core::clone::Clone for IContentTypeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContentTypeProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97d098a5_3b99_4de9_88a5_e11d2f50c795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentTypeProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IDataReader(::windows_core::IUnknown);
impl IDataReader {
    pub fn UnconsumedBufferLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnconsumedBufferLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnicodeEncoding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnicodeEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ByteOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetByteOrder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> ::windows_core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputStreamOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInputStreamOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadByte(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadByte)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReadBytes)(::windows_core::Interface::as_raw(this), value.len() as u32, value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> ::windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBuffer)(::windows_core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    pub fn ReadBoolean(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBoolean)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadGuid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadGuid)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadInt16(&self) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadInt16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadInt32(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadInt32)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadInt64(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadInt64)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadUInt16(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadUInt16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadUInt32(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadUInt32)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadUInt64(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadUInt64)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadSingle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadSingle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadDouble(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadDouble)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadString)(::windows_core::Interface::as_raw(this), codeunitcount, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadDateTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadDateTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTimeSpan(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadTimeSpan)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadAsync(&self, count: u32) -> ::windows_core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadAsync)(::windows_core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IDataReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IDataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataReader {}
impl ::core::fmt::Debug for IDataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IDataReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e2b50029-b4c1-4314-a4b8-fb813a2f275e}");
}
unsafe impl ::windows_core::Interface for IDataReader {
    type Vtable = IDataReader_Vtbl;
}
impl ::core::clone::Clone for IDataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDataReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2b50029_b4c1_4314_a4b8_fb813a2f275e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnconsumedBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows_core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows_core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows_core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows_core::HRESULT,
    pub InputStreamOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputStreamOptions) -> ::windows_core::HRESULT,
    pub SetInputStreamOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InputStreamOptions) -> ::windows_core::HRESULT,
    pub ReadByte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ReadBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *mut u8) -> ::windows_core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReadBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ReadGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ReadInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT,
    pub ReadInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ReadInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub ReadUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub ReadUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ReadUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub ReadSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub ReadDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ReadString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codeunitcount: u32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub ReadTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTimeSpan: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsync: usize,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataReaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataReaderFactory {
    type Vtable = IDataReaderFactory_Vtbl;
}
impl ::core::clone::Clone for IDataReaderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDataReaderFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7527847_57da_4e15_914c_06806699a098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateDataReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataReaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataReaderStatics {
    type Vtable = IDataReaderStatics_Vtbl;
}
impl ::core::clone::Clone for IDataReaderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDataReaderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11fcbfc8_f93a_471b_b121_f379e349313c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IDataWriter(::windows_core::IUnknown);
impl IDataWriter {
    pub fn UnstoredBufferLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnstoredBufferLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnicodeEncoding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnicodeEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ByteOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetByteOrder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteByte)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBytes)(::windows_core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi()).ok() }
    }
    pub fn WriteBufferRange<P0>(&self, buffer: P0, start: u32, count: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBufferRange)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBoolean)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteGuid(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteGuid)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteInt16)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteInt32)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteInt64)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteUInt16)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteUInt32)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteUInt64)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteSingle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteDouble)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteDateTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteDateTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteTimeSpan(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteTimeSpan)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteString(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    pub fn MeasureString(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MeasureString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreAsync(&self) -> ::windows_core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StoreAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IDataWriter, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IDataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataWriter {}
impl ::core::fmt::Debug for IDataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataWriter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IDataWriter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{64b89265-d341-4922-b38a-dd4af8808c4e}");
}
unsafe impl ::windows_core::Interface for IDataWriter {
    type Vtable = IDataWriter_Vtbl;
}
impl ::core::clone::Clone for IDataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDataWriter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64b89265_d341_4922_b38a_dd4af8808c4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnstoredBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows_core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows_core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows_core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows_core::HRESULT,
    pub WriteByte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub WriteBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WriteBufferRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, start: u32, count: u32) -> ::windows_core::HRESULT,
    pub WriteBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub WriteGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub WriteInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub WriteInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub WriteInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows_core::HRESULT,
    pub WriteUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
    pub WriteUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub WriteUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub WriteSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub WriteDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WriteDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub WriteTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTimeSpan: usize,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MeasureString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataWriterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataWriterFactory {
    type Vtable = IDataWriterFactory_Vtbl;
}
impl ::core::clone::Clone for IDataWriterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDataWriterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x338c67c2_8b84_4c2b_9c50_7b8767847a1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateDataWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileRandomAccessStreamStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileRandomAccessStreamStatics {
    type Vtable = IFileRandomAccessStreamStatics_Vtbl;
}
impl ::core::clone::Clone for IFileRandomAccessStreamStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFileRandomAccessStreamStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73550107_3b57_4b5d_8345_554d2fc621f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRandomAccessStreamStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenForUserWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenForUserWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenTransactedWriteForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenTransactedWriteForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenTransactedWriteForUserWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenTransactedWriteForUserWithOptionsAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IInputStream(::windows_core::IUnknown);
impl IInputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IInputStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for IInputStream {}
impl ::core::cmp::PartialEq for IInputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputStream {}
impl ::core::fmt::Debug for IInputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IInputStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{905a0fe2-bc53-11df-8c49-001e4fc686da}");
}
unsafe impl ::windows_core::Interface for IInputStream {
    type Vtable = IInputStream_Vtbl;
}
impl ::core::clone::Clone for IInputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInputStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905a0fe2_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStream_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IInputStreamReference(::windows_core::IUnknown);
impl IInputStreamReference {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenSequentialReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IInputStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenSequentialReadAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IInputStreamReference, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IInputStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputStreamReference {}
impl ::core::fmt::Debug for IInputStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputStreamReference").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IInputStreamReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{43929d18-5ec9-4b5a-919c-4205b0c804b6}");
}
unsafe impl ::windows_core::Interface for IInputStreamReference {
    type Vtable = IInputStreamReference_Vtbl;
}
impl ::core::clone::Clone for IInputStreamReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInputStreamReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43929d18_5ec9_4b5a_919c_4205b0c804b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStreamReference_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenSequentialReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenSequentialReadAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IOutputStream(::windows_core::IUnknown);
impl IOutputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IOutputStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for IOutputStream {}
impl ::core::cmp::PartialEq for IOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOutputStream {}
impl ::core::fmt::Debug for IOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOutputStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IOutputStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{905a0fe6-bc53-11df-8c49-001e4fc686da}");
}
unsafe impl ::windows_core::Interface for IOutputStream {
    type Vtable = IOutputStream_Vtbl;
}
impl ::core::clone::Clone for IOutputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOutputStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905a0fe6_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOutputStream_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub WriteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IPropertySetSerializer(::windows_core::IUnknown);
impl IPropertySetSerializer {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Serialize<P0>(&self, propertyset: P0) -> ::windows_core::Result<IBuffer>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Serialize)(::windows_core::Interface::as_raw(this), propertyset.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Deserialize<P0, P1>(&self, propertyset: P0, buffer: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
        P1: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Deserialize)(::windows_core::Interface::as_raw(this), propertyset.try_into_param()?.abi(), buffer.try_into_param()?.abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IPropertySetSerializer, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IPropertySetSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySetSerializer {}
impl ::core::fmt::Debug for IPropertySetSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySetSerializer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IPropertySetSerializer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6e8ebf1c-ef3d-4376-b20e-5be638aeac77}");
}
unsafe impl ::windows_core::Interface for IPropertySetSerializer {
    type Vtable = IPropertySetSerializer_Vtbl;
}
impl ::core::clone::Clone for IPropertySetSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertySetSerializer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e8ebf1c_ef3d_4376_b20e_5be638aeac77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetSerializer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Serialize: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Deserialize: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IRandomAccessStream(::windows_core::IUnknown);
impl IRandomAccessStream {
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IRandomAccessStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for IRandomAccessStream {}
impl ::windows_core::CanTryInto<IInputStream> for IRandomAccessStream {}
impl ::windows_core::CanTryInto<IOutputStream> for IRandomAccessStream {}
impl ::core::cmp::PartialEq for IRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStream {}
impl ::core::fmt::Debug for IRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IRandomAccessStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{905a0fe1-bc53-11df-8c49-001e4fc686da}");
}
unsafe impl ::windows_core::Interface for IRandomAccessStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
impl ::core::clone::Clone for IRandomAccessStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRandomAccessStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905a0fe1_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStream_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub GetInputStreamAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOutputStreamAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64) -> ::windows_core::HRESULT,
    pub CloneStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CanRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IRandomAccessStreamReference(::windows_core::IUnknown);
impl IRandomAccessStreamReference {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenReadAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IRandomAccessStreamReference, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IRandomAccessStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamReference {}
impl ::core::fmt::Debug for IRandomAccessStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamReference").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IRandomAccessStreamReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{33ee3134-1dd6-4e3a-8067-d1c162e8642b}");
}
unsafe impl ::windows_core::Interface for IRandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_Vtbl;
}
impl ::core::clone::Clone for IRandomAccessStreamReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRandomAccessStreamReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReference_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenReadAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRandomAccessStreamReferenceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRandomAccessStreamReferenceStatics {
    type Vtable = IRandomAccessStreamReferenceStatics_Vtbl;
}
impl ::core::clone::Clone for IRandomAccessStreamReferenceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRandomAccessStreamReferenceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x857309dc_3fbf_4e7d_986f_ef3b1a07a964);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUri: usize,
    pub CreateFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRandomAccessStreamStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRandomAccessStreamStatics {
    type Vtable = IRandomAccessStreamStatics_Vtbl;
}
impl ::core::clone::Clone for IRandomAccessStreamStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRandomAccessStreamStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x524cedcf_6e29_4ce5_9573_6b753db66c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CopyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopySizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, bytestocopy: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopySizeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopyAndCloseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAndCloseAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IRandomAccessStreamWithContentType(::windows_core::IUnknown);
impl IRandomAccessStreamWithContentType {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<IInputStream> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<IOutputStream> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows_core::Result<IRandomAccessStream> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IRandomAccessStreamWithContentType, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for IRandomAccessStreamWithContentType {}
impl ::windows_core::CanTryInto<IContentTypeProvider> for IRandomAccessStreamWithContentType {}
impl ::windows_core::CanTryInto<IInputStream> for IRandomAccessStreamWithContentType {}
impl ::windows_core::CanTryInto<IOutputStream> for IRandomAccessStreamWithContentType {}
impl ::windows_core::CanTryInto<IRandomAccessStream> for IRandomAccessStreamWithContentType {}
impl ::core::cmp::PartialEq for IRandomAccessStreamWithContentType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamWithContentType {}
impl ::core::fmt::Debug for IRandomAccessStreamWithContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamWithContentType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IRandomAccessStreamWithContentType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{cc254827-4b3d-438f-9232-10c76bc7e038}");
}
unsafe impl ::windows_core::Interface for IRandomAccessStreamWithContentType {
    type Vtable = IRandomAccessStreamWithContentType_Vtbl;
}
impl ::core::clone::Clone for IRandomAccessStreamWithContentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRandomAccessStreamWithContentType {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc254827_4b3d_438f_9232_10c76bc7e038);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamWithContentType_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct Buffer(::windows_core::IUnknown);
impl Buffer {
    pub fn Capacity(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Capacity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(capacity: u32) -> ::windows_core::Result<Buffer> {
        Self::IBufferFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), capacity, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateCopyFromMemoryBuffer<P0>(input: P0) -> ::windows_core::Result<Buffer>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IMemoryBuffer>,
    {
        Self::IBufferStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyFromMemoryBuffer)(::windows_core::Interface::as_raw(this), input.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateMemoryBufferOverIBuffer<P0>(input: P0) -> ::windows_core::Result<super::super::Foundation::MemoryBuffer>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        Self::IBufferStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMemoryBufferOverIBuffer)(::windows_core::Interface::as_raw(this), input.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBufferFactory<R, F: FnOnce(&IBufferFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Buffer, IBufferFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBufferStatics<R, F: FnOnce(&IBufferStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Buffer, IBufferStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Buffer {}
impl ::core::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Buffer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Buffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.Buffer;{905a0fe0-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for Buffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Buffer {
    type Vtable = IBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Buffer {
    const IID: ::windows_core::GUID = <IBuffer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Buffer {
    const NAME: &'static str = "Windows.Storage.Streams.Buffer";
}
::windows_core::imp::interface_hierarchy!(Buffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBuffer> for Buffer {}
unsafe impl ::core::marker::Send for Buffer {}
unsafe impl ::core::marker::Sync for Buffer {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct DataReader(::windows_core::IUnknown);
impl DataReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UnconsumedBufferLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnconsumedBufferLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnicodeEncoding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnicodeEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ByteOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetByteOrder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> ::windows_core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputStreamOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInputStreamOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadByte(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadByte)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReadBytes)(::windows_core::Interface::as_raw(this), value.len() as u32, value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> ::windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBuffer)(::windows_core::Interface::as_raw(this), length, &mut result__).from_abi(result__)
        }
    }
    pub fn ReadBoolean(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadBoolean)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadGuid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadGuid)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadInt16(&self) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadInt16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadInt32(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadInt32)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadInt64(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadInt64)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadUInt16(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadUInt16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadUInt32(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadUInt32)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadUInt64(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadUInt64)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadSingle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadSingle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadDouble(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadDouble)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadString)(::windows_core::Interface::as_raw(this), codeunitcount, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadDateTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadDateTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTimeSpan(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadTimeSpan)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadAsync(&self, count: u32) -> ::windows_core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadAsync)(::windows_core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateDataReader<P0>(inputstream: P0) -> ::windows_core::Result<DataReader>
    where
        P0: ::windows_core::TryIntoParam<IInputStream>,
    {
        Self::IDataReaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateDataReader)(::windows_core::Interface::as_raw(this), inputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn FromBuffer<P0>(buffer: P0) -> ::windows_core::Result<DataReader>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        Self::IDataReaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DataReader, IDataReaderFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DataReader, IDataReaderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataReader {}
impl ::core::fmt::Debug for DataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DataReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReader;{e2b50029-b4c1-4314-a4b8-fb813a2f275e})");
}
impl ::core::clone::Clone for DataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DataReader {
    type Vtable = IDataReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataReader {
    const IID: ::windows_core::GUID = <IDataReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataReader {
    const NAME: &'static str = "Windows.Storage.Streams.DataReader";
}
::windows_core::imp::interface_hierarchy!(DataReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for DataReader {}
impl ::windows_core::CanTryInto<IDataReader> for DataReader {}
unsafe impl ::core::marker::Send for DataReader {}
unsafe impl ::core::marker::Sync for DataReader {}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct DataReaderLoadOperation(::windows_core::IUnknown);
#[cfg(feature = "Foundation")]
impl DataReaderLoadOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows_core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::AsyncOperationCompletedHandler<u32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows_core::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for DataReaderLoadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for DataReaderLoadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataReaderLoadOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeType for DataReaderLoadOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReaderLoadOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for DataReaderLoadOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows_core::Interface for DataReaderLoadOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_Vtbl<u32>;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows_core::ComInterface for DataReaderLoadOperation {
    const IID: ::windows_core::GUID = <super::super::Foundation::IAsyncOperation<u32> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for DataReaderLoadOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataReaderLoadOperation";
}
#[cfg(feature = "Foundation")]
impl DataReaderLoadOperation {
    pub fn get(&self) -> ::windows_core::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
impl ::std::future::Future for DataReaderLoadOperation {
    type Output = ::windows_core::Result<u32>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
::windows_core::imp::interface_hierarchy!(DataReaderLoadOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IAsyncInfo> for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IAsyncOperation<u32>> for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for DataReaderLoadOperation {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct DataWriter(::windows_core::IUnknown);
impl DataWriter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DataWriter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UnstoredBufferLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnstoredBufferLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnicodeEncoding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnicodeEncoding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ByteOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetByteOrder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteByte)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBytes)(::windows_core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi()).ok() }
    }
    pub fn WriteBufferRange<P0>(&self, buffer: P0, start: u32, count: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBufferRange)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteBoolean)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteGuid(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteGuid)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteInt16)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteInt32)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteInt64)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteUInt16)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteUInt32)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteUInt64)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteSingle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteDouble)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteDateTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteDateTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteTimeSpan(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteTimeSpan)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteString(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    pub fn MeasureString(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MeasureString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreAsync(&self) -> ::windows_core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StoreAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetachStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateDataWriter<P0>(outputstream: P0) -> ::windows_core::Result<DataWriter>
    where
        P0: ::windows_core::TryIntoParam<IOutputStream>,
    {
        Self::IDataWriterFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateDataWriter)(::windows_core::Interface::as_raw(this), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataWriterFactory<R, F: FnOnce(&IDataWriterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DataWriter, IDataWriterFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataWriter {}
impl ::core::fmt::Debug for DataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataWriter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DataWriter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriter;{64b89265-d341-4922-b38a-dd4af8808c4e})");
}
impl ::core::clone::Clone for DataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DataWriter {
    type Vtable = IDataWriter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataWriter {
    const IID: ::windows_core::GUID = <IDataWriter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriter";
}
::windows_core::imp::interface_hierarchy!(DataWriter, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for DataWriter {}
impl ::windows_core::CanTryInto<IDataWriter> for DataWriter {}
unsafe impl ::core::marker::Send for DataWriter {}
unsafe impl ::core::marker::Sync for DataWriter {}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct DataWriterStoreOperation(::windows_core::IUnknown);
#[cfg(feature = "Foundation")]
impl DataWriterStoreOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows_core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::AsyncOperationCompletedHandler<u32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows_core::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for DataWriterStoreOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for DataWriterStoreOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataWriterStoreOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeType for DataWriterStoreOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriterStoreOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for DataWriterStoreOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows_core::Interface for DataWriterStoreOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_Vtbl<u32>;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows_core::ComInterface for DataWriterStoreOperation {
    const IID: ::windows_core::GUID = <super::super::Foundation::IAsyncOperation<u32> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for DataWriterStoreOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriterStoreOperation";
}
#[cfg(feature = "Foundation")]
impl DataWriterStoreOperation {
    pub fn get(&self) -> ::windows_core::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
impl ::std::future::Future for DataWriterStoreOperation {
    type Output = ::windows_core::Result<u32>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
::windows_core::imp::interface_hierarchy!(DataWriterStoreOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IAsyncInfo> for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IAsyncOperation<u32>> for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for DataWriterStoreOperation {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct FileInputStream(::windows_core::IUnknown);
impl FileInputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FileInputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInputStream {}
impl ::core::fmt::Debug for FileInputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInputStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FileInputStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileInputStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for FileInputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FileInputStream {
    type Vtable = IInputStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileInputStream {
    const IID: ::windows_core::GUID = <IInputStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileInputStream";
}
::windows_core::imp::interface_hierarchy!(FileInputStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for FileInputStream {}
impl ::windows_core::CanTryInto<IInputStream> for FileInputStream {}
unsafe impl ::core::marker::Send for FileInputStream {}
unsafe impl ::core::marker::Sync for FileInputStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct FileOutputStream(::windows_core::IUnknown);
impl FileOutputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FileOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOutputStream {}
impl ::core::fmt::Debug for FileOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOutputStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FileOutputStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileOutputStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for FileOutputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FileOutputStream {
    type Vtable = IOutputStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileOutputStream {
    const IID: ::windows_core::GUID = <IOutputStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileOutputStream";
}
::windows_core::imp::interface_hierarchy!(FileOutputStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for FileOutputStream {}
impl ::windows_core::CanTryInto<IOutputStream> for FileOutputStream {}
unsafe impl ::core::marker::Send for FileOutputStream {}
unsafe impl ::core::marker::Sync for FileOutputStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct FileRandomAccessStream(::windows_core::IUnknown);
impl FileRandomAccessStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(filepath: &::windows_core::HSTRING, accessmode: super::FileAccessMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), accessmode, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenWithOptionsAsync(filepath: &::windows_core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), accessmode, sharingoptions, opendisposition, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(filepath: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenTransactedWriteAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(filepath: &::windows_core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), openoptions, opendisposition, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenForUserAsync<P0>(user: P0, filepath: &::windows_core::HSTRING, accessmode: super::FileAccessMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(filepath), accessmode, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenForUserWithOptionsAsync<P0>(user: P0, filepath: &::windows_core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenForUserWithOptionsAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(filepath), accessmode, sharingoptions, opendisposition, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenTransactedWriteForUserAsync<P0>(user: P0, filepath: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenTransactedWriteForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(filepath), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenTransactedWriteForUserWithOptionsAsync<P0>(user: P0, filepath: &::windows_core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenTransactedWriteForUserWithOptionsAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(filepath), openoptions, opendisposition, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IFileRandomAccessStreamStatics<R, F: FnOnce(&IFileRandomAccessStreamStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileRandomAccessStream, IFileRandomAccessStreamStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for FileRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileRandomAccessStream {}
impl ::core::fmt::Debug for FileRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileRandomAccessStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FileRandomAccessStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for FileRandomAccessStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FileRandomAccessStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileRandomAccessStream {
    const IID: ::windows_core::GUID = <IRandomAccessStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileRandomAccessStream";
}
::windows_core::imp::interface_hierarchy!(FileRandomAccessStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for FileRandomAccessStream {}
impl ::windows_core::CanTryInto<IInputStream> for FileRandomAccessStream {}
impl ::windows_core::CanTryInto<IOutputStream> for FileRandomAccessStream {}
impl ::windows_core::CanTryInto<IRandomAccessStream> for FileRandomAccessStream {}
unsafe impl ::core::marker::Send for FileRandomAccessStream {}
unsafe impl ::core::marker::Sync for FileRandomAccessStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct InMemoryRandomAccessStream(::windows_core::IUnknown);
impl InMemoryRandomAccessStream {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InMemoryRandomAccessStream, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InMemoryRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InMemoryRandomAccessStream {}
impl ::core::fmt::Debug for InMemoryRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InMemoryRandomAccessStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InMemoryRandomAccessStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InMemoryRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for InMemoryRandomAccessStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InMemoryRandomAccessStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InMemoryRandomAccessStream {
    const IID: ::windows_core::GUID = <IRandomAccessStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InMemoryRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.InMemoryRandomAccessStream";
}
::windows_core::imp::interface_hierarchy!(InMemoryRandomAccessStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for InMemoryRandomAccessStream {}
impl ::windows_core::CanTryInto<IInputStream> for InMemoryRandomAccessStream {}
impl ::windows_core::CanTryInto<IOutputStream> for InMemoryRandomAccessStream {}
impl ::windows_core::CanTryInto<IRandomAccessStream> for InMemoryRandomAccessStream {}
unsafe impl ::core::marker::Send for InMemoryRandomAccessStream {}
unsafe impl ::core::marker::Sync for InMemoryRandomAccessStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct InputStreamOverStream(::windows_core::IUnknown);
impl InputStreamOverStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InputStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputStreamOverStream {}
impl ::core::fmt::Debug for InputStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputStreamOverStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InputStreamOverStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InputStreamOverStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for InputStreamOverStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InputStreamOverStream {
    type Vtable = IInputStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputStreamOverStream {
    const IID: ::windows_core::GUID = <IInputStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.InputStreamOverStream";
}
::windows_core::imp::interface_hierarchy!(InputStreamOverStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for InputStreamOverStream {}
impl ::windows_core::CanTryInto<IInputStream> for InputStreamOverStream {}
unsafe impl ::core::marker::Send for InputStreamOverStream {}
unsafe impl ::core::marker::Sync for InputStreamOverStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct OutputStreamOverStream(::windows_core::IUnknown);
impl OutputStreamOverStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OutputStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OutputStreamOverStream {}
impl ::core::fmt::Debug for OutputStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OutputStreamOverStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for OutputStreamOverStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.OutputStreamOverStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for OutputStreamOverStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OutputStreamOverStream {
    type Vtable = IOutputStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OutputStreamOverStream {
    const IID: ::windows_core::GUID = <IOutputStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OutputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.OutputStreamOverStream";
}
::windows_core::imp::interface_hierarchy!(OutputStreamOverStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for OutputStreamOverStream {}
impl ::windows_core::CanTryInto<IOutputStream> for OutputStreamOverStream {}
unsafe impl ::core::marker::Send for OutputStreamOverStream {}
unsafe impl ::core::marker::Sync for OutputStreamOverStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
pub struct RandomAccessStream;
impl RandomAccessStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAsync<P0, P1>(source: P0, destination: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows_core::TryIntoParam<IInputStream>,
        P1: ::windows_core::TryIntoParam<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), destination.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopySizeAsync<P0, P1>(source: P0, destination: P1, bytestocopy: u64) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows_core::TryIntoParam<IInputStream>,
        P1: ::windows_core::TryIntoParam<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopySizeAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), destination.try_into_param()?.abi(), bytestocopy, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndCloseAsync<P0, P1>(source: P0, destination: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::windows_core::TryIntoParam<IInputStream>,
        P1: ::windows_core::TryIntoParam<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyAndCloseAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), destination.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRandomAccessStreamStatics<R, F: FnOnce(&IRandomAccessStreamStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RandomAccessStream, IRandomAccessStreamStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for RandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStream";
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct RandomAccessStreamOverStream(::windows_core::IUnknown);
impl RandomAccessStreamOverStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RandomAccessStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RandomAccessStreamOverStream {}
impl ::core::fmt::Debug for RandomAccessStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RandomAccessStreamOverStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RandomAccessStreamOverStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamOverStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
}
impl ::core::clone::Clone for RandomAccessStreamOverStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RandomAccessStreamOverStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RandomAccessStreamOverStream {
    const IID: ::windows_core::GUID = <IRandomAccessStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RandomAccessStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamOverStream";
}
::windows_core::imp::interface_hierarchy!(RandomAccessStreamOverStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for RandomAccessStreamOverStream {}
impl ::windows_core::CanTryInto<IInputStream> for RandomAccessStreamOverStream {}
impl ::windows_core::CanTryInto<IOutputStream> for RandomAccessStreamOverStream {}
impl ::windows_core::CanTryInto<IRandomAccessStream> for RandomAccessStreamOverStream {}
unsafe impl ::core::marker::Send for RandomAccessStreamOverStream {}
unsafe impl ::core::marker::Sync for RandomAccessStreamOverStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct RandomAccessStreamReference(::windows_core::IUnknown);
impl RandomAccessStreamReference {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenReadAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromFile<P0>(file: P0) -> ::windows_core::Result<RandomAccessStreamReference>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFile>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromFile)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUri<P0>(uri: P0) -> ::windows_core::Result<RandomAccessStreamReference>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromStream<P0>(stream: P0) -> ::windows_core::Result<RandomAccessStreamReference>
    where
        P0: ::windows_core::TryIntoParam<IRandomAccessStream>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStream)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRandomAccessStreamReferenceStatics<R, F: FnOnce(&IRandomAccessStreamReferenceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RandomAccessStreamReference, IRandomAccessStreamReferenceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for RandomAccessStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RandomAccessStreamReference {}
impl ::core::fmt::Debug for RandomAccessStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RandomAccessStreamReference").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RandomAccessStreamReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamReference;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
}
impl ::core::clone::Clone for RandomAccessStreamReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RandomAccessStreamReference {
    const IID: ::windows_core::GUID = <IRandomAccessStreamReference as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
::windows_core::imp::interface_hierarchy!(RandomAccessStreamReference, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IRandomAccessStreamReference> for RandomAccessStreamReference {}
unsafe impl ::core::marker::Send for RandomAccessStreamReference {}
unsafe impl ::core::marker::Sync for RandomAccessStreamReference {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: Self = Self(0i32);
    pub const BigEndian: Self = Self(1i32);
}
impl ::core::marker::Copy for ByteOrder {}
impl ::core::clone::Clone for ByteOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ByteOrder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ByteOrder {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ByteOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ByteOrder").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ByteOrder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.ByteOrder;i4)");
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: Self = Self(0i32);
    pub const OpenAlways: Self = Self(1i32);
    pub const CreateNew: Self = Self(2i32);
    pub const CreateAlways: Self = Self(3i32);
    pub const TruncateExisting: Self = Self(4i32);
}
impl ::core::marker::Copy for FileOpenDisposition {}
impl ::core::clone::Clone for FileOpenDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileOpenDisposition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FileOpenDisposition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FileOpenDisposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenDisposition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FileOpenDisposition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.FileOpenDisposition;i4)");
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: Self = Self(0u32);
    pub const Partial: Self = Self(1u32);
    pub const ReadAhead: Self = Self(2u32);
}
impl ::core::marker::Copy for InputStreamOptions {}
impl ::core::clone::Clone for InputStreamOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputStreamOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InputStreamOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InputStreamOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputStreamOptions").field(&self.0).finish()
    }
}
impl InputStreamOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for InputStreamOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InputStreamOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InputStreamOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InputStreamOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InputStreamOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for InputStreamOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.InputStreamOptions;u4)");
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}
impl ::core::marker::Copy for UnicodeEncoding {}
impl ::core::clone::Clone for UnicodeEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnicodeEncoding {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UnicodeEncoding {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UnicodeEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnicodeEncoding").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UnicodeEncoding {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.UnicodeEncoding;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
