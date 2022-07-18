#[repr(C)]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl ::core::marker::Copy for DateTime {}
impl ::core::clone::Clone for DateTime {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DateTime").field("UniversalTime", &self.UniversalTime).finish()
    }
}
unsafe impl ::windows::core::Abi for DateTime {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DateTime {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DateTime>()) == 0 }
    }
}
impl ::core::cmp::Eq for DateTime {}
impl ::core::default::Default for DateTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IPropertyValue(::windows::core::IUnknown);
impl IPropertyValue {
    pub fn Type(&self) -> ::windows::core::Result<PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PropertyType>(result__)
        }
    }
    pub fn IsNumericScalar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsNumericScalar)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetUInt8(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetUInt8)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn GetInt16(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetInt16)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i16>(result__)
        }
    }
    pub fn GetUInt16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetUInt16)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetInt32)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GetUInt32(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetUInt32)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetInt64)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn GetUInt64(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetUInt64)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn GetSingle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetSingle)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn GetDouble(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDouble)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetChar16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetChar16)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetBoolean)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetString)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetGuid)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn GetDateTime(&self) -> ::windows::core::Result<DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDateTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTime>(result__)
        }
    }
    pub fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetTimeSpan)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimeSpan>(result__)
        }
    }
    pub fn GetPoint(&self) -> ::windows::core::Result<Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetPoint)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Point>(result__)
        }
    }
    pub fn GetSize(&self) -> ::windows::core::Result<Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Size>(result__)
        }
    }
    pub fn GetRect(&self) -> ::windows::core::Result<Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetRect)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Rect>(result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetUInt8Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetInt16Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetUInt16Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetInt32Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetUInt32Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetInt64Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetUInt64Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetSingleArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetDoubleArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetChar16Array)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetBooleanArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetStringArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetInspectableArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetGuidArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetDateTimeArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetTimeSpanArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetPointArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetSizeArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetRectArray)(::windows::core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl ::core::convert::From<IPropertyValue> for ::windows::core::IUnknown {
    fn from(value: IPropertyValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropertyValue> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPropertyValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyValue> for ::windows::core::IUnknown {
    fn from(value: &IPropertyValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPropertyValue> for ::windows::core::IInspectable {
    fn from(value: IPropertyValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropertyValue> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPropertyValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyValue> for ::windows::core::IInspectable {
    fn from(value: &IPropertyValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPropertyValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyValue {}
impl ::core::fmt::Debug for IPropertyValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPropertyValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4bd682dd-7554-40e9-9a9b-82654ede7e62}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPropertyValue {
    type Vtable = IPropertyValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bd682dd_7554_40e9_9a9b_82654ede7e62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValue_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PropertyType) -> ::windows::core::HRESULT,
    pub IsNumericScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetUInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub GetInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT,
    pub GetUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub GetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub GetUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub GetSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub GetDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DateTime) -> ::windows::core::HRESULT,
    pub GetTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimeSpan) -> ::windows::core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Point) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Size) -> ::windows::core::HRESULT,
    pub GetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Rect) -> ::windows::core::HRESULT,
    pub GetUInt8Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::core::HRESULT,
    pub GetUInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT,
    pub GetInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::core::HRESULT,
    pub GetUInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::core::HRESULT,
    pub GetInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::core::HRESULT,
    pub GetUInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::core::HRESULT,
    pub GetSingleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::core::HRESULT,
    pub GetDoubleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::core::HRESULT,
    pub GetChar16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT,
    pub GetBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetInspectableArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGuidArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::core::HRESULT,
    pub GetTimeSpanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::core::HRESULT,
    pub GetPointArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::core::HRESULT,
    pub GetSizeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::core::HRESULT,
    pub GetRectArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPropertyValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPropertyValueStatics {
    type Vtable = IPropertyValueStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInspectable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt8Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSingleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDoubleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateChar16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInspectableArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGuidArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTimeSpanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePointArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSizeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRectArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IReference<T>(::windows::core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;
impl<T: ::windows::core::RuntimeType + 'static> IReference<T> {
    pub fn Value(&self) -> ::windows::core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<T>(result__)
        }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::core::IUnknown {
    fn from(value: IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IReference<T>> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::core::IUnknown {
    fn from(value: &IReference<T>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::core::IInspectable {
    fn from(value: IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IReference<T>> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::core::IInspectable {
    fn from(value: &IReference<T>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IReference<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: IReference<T>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReference<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReference<T>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReference<T>> for ::windows::core::InParam<'a, IPropertyValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReference<T>) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for IReference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IReference<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IReference<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IReference<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReference").field(&self.0).finish()
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IReference<T> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IReference<T> {
    type Vtable = IReference_Vtbl<T>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_Vtbl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IInspectableVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[repr(transparent)]
pub struct IStringable(::windows::core::IUnknown);
impl IStringable {
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IStringable> for ::windows::core::IUnknown {
    fn from(value: IStringable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IStringable> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IStringable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStringable> for ::windows::core::IUnknown {
    fn from(value: &IStringable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IStringable> for ::windows::core::IInspectable {
    fn from(value: IStringable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IStringable> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IStringable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStringable> for ::windows::core::IInspectable {
    fn from(value: &IStringable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IStringable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStringable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStringable {}
impl ::core::fmt::Debug for IStringable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStringable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStringable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStringable {
    type Vtable = IStringable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96369f54_8eb6_48f0_abce_c1b211e627c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl ::core::marker::Copy for Point {}
impl ::core::clone::Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Point").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows::core::Abi for Point {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Point {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Point>()) == 0 }
    }
}
impl ::core::cmp::Eq for Point {}
impl ::core::default::Default for Point {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: Self = Self(0i32);
    pub const UInt8: Self = Self(1i32);
    pub const Int16: Self = Self(2i32);
    pub const UInt16: Self = Self(3i32);
    pub const Int32: Self = Self(4i32);
    pub const UInt32: Self = Self(5i32);
    pub const Int64: Self = Self(6i32);
    pub const UInt64: Self = Self(7i32);
    pub const Single: Self = Self(8i32);
    pub const Double: Self = Self(9i32);
    pub const Char16: Self = Self(10i32);
    pub const Boolean: Self = Self(11i32);
    pub const String: Self = Self(12i32);
    pub const Inspectable: Self = Self(13i32);
    pub const DateTime: Self = Self(14i32);
    pub const TimeSpan: Self = Self(15i32);
    pub const Guid: Self = Self(16i32);
    pub const Point: Self = Self(17i32);
    pub const Size: Self = Self(18i32);
    pub const Rect: Self = Self(19i32);
    pub const OtherType: Self = Self(20i32);
    pub const UInt8Array: Self = Self(1025i32);
    pub const Int16Array: Self = Self(1026i32);
    pub const UInt16Array: Self = Self(1027i32);
    pub const Int32Array: Self = Self(1028i32);
    pub const UInt32Array: Self = Self(1029i32);
    pub const Int64Array: Self = Self(1030i32);
    pub const UInt64Array: Self = Self(1031i32);
    pub const SingleArray: Self = Self(1032i32);
    pub const DoubleArray: Self = Self(1033i32);
    pub const Char16Array: Self = Self(1034i32);
    pub const BooleanArray: Self = Self(1035i32);
    pub const StringArray: Self = Self(1036i32);
    pub const InspectableArray: Self = Self(1037i32);
    pub const DateTimeArray: Self = Self(1038i32);
    pub const TimeSpanArray: Self = Self(1039i32);
    pub const GuidArray: Self = Self(1040i32);
    pub const PointArray: Self = Self(1041i32);
    pub const SizeArray: Self = Self(1042i32);
    pub const RectArray: Self = Self(1043i32);
    pub const OtherTypeArray: Self = Self(1044i32);
}
impl ::core::marker::Copy for PropertyType {}
impl ::core::clone::Clone for PropertyType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PropertyType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
pub struct PropertyValue;
impl PropertyValue {
    pub fn CreateEmpty() -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateEmpty)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt8(value: u8) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt8)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInt16(value: i16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInt16)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt16(value: u16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt16)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInt32(value: i32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInt32)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt32(value: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt32)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInt64(value: i64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInt64)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt64(value: u64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt64)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateSingle(value: f32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateSingle)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateDouble(value: f64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateDouble)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateChar16(value: u16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateChar16)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateBoolean)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateString(value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInspectable<'a, P0>(value: P0) -> ::windows::core::Result<::windows::core::IInspectable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInspectable)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateGuid(value: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateGuid)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateDateTime(value: DateTime) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTime)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateTimeSpan(value: TimeSpan) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateTimeSpan)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreatePoint(value: Point) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreatePoint)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateSize(value: Size) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateSize)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateRect(value: Rect) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateRect)(::windows::core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt8Array(value: &[u8]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt8Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInt16Array(value: &[i16]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInt16Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt16Array(value: &[u16]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt16Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInt32Array(value: &[i32]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInt32Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt32Array(value: &[u32]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt32Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInt64Array(value: &[i64]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInt64Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateUInt64Array(value: &[u64]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateUInt64Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateSingleArray(value: &[f32]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateSingleArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateDoubleArray(value: &[f64]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateDoubleArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateChar16Array(value: &[u16]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateChar16Array)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateBooleanArray(value: &[bool]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateBooleanArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateStringArray(value: &[::windows::core::HSTRING]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateStringArray)(::windows::core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateInspectableArray(value: &[::core::option::Option<::windows::core::IInspectable>]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInspectableArray)(::windows::core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateGuidArray(value: &[::windows::core::GUID]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateGuidArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateDateTimeArray(value: &[DateTime]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateDateTimeArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateTimeSpanArray(value: &[TimeSpan]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateTimeSpanArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreatePointArray(value: &[Point]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreatePointArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateSizeArray(value: &[Size]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateSizeArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn CreateRectArray(value: &[Rect]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateRectArray)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PropertyValue, IPropertyValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PropertyValue {
    const NAME: &'static str = "Windows.Foundation.PropertyValue";
}
#[repr(C)]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Rect {}
impl ::core::clone::Clone for Rect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Rect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Rect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for Rect {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Rect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Rect>()) == 0 }
    }
}
impl ::core::cmp::Eq for Rect {}
impl ::core::default::Default for Rect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Size {}
impl ::core::clone::Clone for Size {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Size {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Size").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for Size {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Size {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Size>()) == 0 }
    }
}
impl ::core::cmp::Eq for Size {}
impl ::core::default::Default for Size {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TimeSpan {
    pub Duration: i64,
}
impl ::core::marker::Copy for TimeSpan {}
impl ::core::clone::Clone for TimeSpan {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimeSpan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimeSpan").field("Duration", &self.Duration).finish()
    }
}
unsafe impl ::windows::core::Abi for TimeSpan {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimeSpan {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TimeSpan {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimeSpan>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimeSpan {}
impl ::core::default::Default for TimeSpan {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::convert::From<::core::time::Duration> for TimeSpan {
    fn from(value: ::core::time::Duration) -> Self {
        Self { Duration: (value.as_nanos() / 100) as i64 }
    }
}
impl ::core::convert::From<TimeSpan> for ::core::time::Duration {
    fn from(value: TimeSpan) -> Self {
        ::core::time::Duration::from_nanos((value.Duration * 100) as u64)
    }
}
#[repr(transparent)]
pub struct BOOL(pub i32);
impl BOOL {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    #[inline]
    pub fn ok(self) -> ::windows::core::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl ::core::default::Default for BOOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::clone::Clone for BOOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for BOOL {}
impl ::core::cmp::PartialEq for BOOL {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BOOL {}
impl ::core::fmt::Debug for BOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BOOL").field(&self.0).finish()
    }
}
impl ::core::convert::From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.as_bool()
    }
}
impl ::core::convert::From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}
impl ::core::convert::From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            BOOL(1)
        } else {
            BOOL(0)
        }
    }
}
impl ::core::convert::From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl ::core::cmp::PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl ::core::cmp::PartialEq<BOOL> for bool {
    fn eq(&self, other: &BOOL) -> bool {
        *self == other.as_bool()
    }
}
impl ::core::ops::Not for BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            BOOL(0)
        } else {
            BOOL(1)
        }
    }
}
unsafe impl ::windows::core::Abi for BOOL {
    type Abi = Self;
}
#[repr(transparent)]
pub struct BSTR(*const u16);
impl BSTR {
    pub fn new() -> Self {
        Self(core::ptr::null_mut())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        if self.0.is_null() {
            0
        } else {
            unsafe { SysStringLen(self) as usize }
        }
    }
    pub fn from_wide(value: &[u16]) -> Self {
        if value.is_empty() {
            return Self(::core::ptr::null_mut());
        }
        unsafe { SysAllocStringLen(value) }
    }
    pub fn as_wide(&self) -> &[u16] {
        if self.0.is_null() {
            return &[];
        }
        unsafe { ::core::slice::from_raw_parts(self.0, self.len()) }
    }
    pub unsafe fn from_raw(raw: *const u16) -> Self {
        Self(raw)
    }
    pub fn into_raw(self) -> *const u16 {
        unsafe { std::mem::transmute(self) }
    }
}
impl ::core::clone::Clone for BSTR {
    fn clone(&self) -> Self {
        Self::from_wide(self.as_wide())
    }
}
impl ::core::convert::From<&str> for BSTR {
    fn from(value: &str) -> Self {
        let value: ::windows::core::alloc::vec::Vec<u16> = value.encode_utf16().collect();
        Self::from_wide(&value)
    }
}
impl ::core::convert::From<::windows::core::alloc::string::String> for BSTR {
    fn from(value: ::windows::core::alloc::string::String) -> Self {
        value.as_str().into()
    }
}
impl ::core::convert::From<&::windows::core::alloc::string::String> for BSTR {
    fn from(value: &::windows::core::alloc::string::String) -> Self {
        value.as_str().into()
    }
}
impl<'a> ::core::convert::TryFrom<&'a BSTR> for ::windows::core::alloc::string::String {
    type Error = ::windows::core::alloc::string::FromUtf16Error;
    fn try_from(value: &BSTR) -> ::core::result::Result<Self, Self::Error> {
        ::windows::core::alloc::string::String::from_utf16(value.as_wide())
    }
}
impl ::core::convert::TryFrom<BSTR> for ::windows::core::alloc::string::String {
    type Error = ::windows::core::alloc::string::FromUtf16Error;
    fn try_from(value: BSTR) -> ::core::result::Result<Self, Self::Error> {
        ::windows::core::alloc::string::String::try_from(&value)
    }
}
impl ::core::default::Default for BSTR {
    fn default() -> Self {
        Self(::core::ptr::null_mut())
    }
}
impl ::core::fmt::Display for BSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::write!(f, "{}", ::windows::core::Decode(|| ::core::char::decode_utf16(self.as_wide().iter().cloned())))
    }
}
impl ::core::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::write!(f, "{}", self)
    }
}
impl ::core::cmp::PartialEq for BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.as_wide() == other.as_wide()
    }
}
impl ::core::cmp::Eq for BSTR {}
impl ::core::cmp::PartialEq<::windows::core::alloc::string::String> for BSTR {
    fn eq(&self, other: &::windows::core::alloc::string::String) -> bool {
        self == other.as_str()
    }
}
impl ::core::cmp::PartialEq<str> for BSTR {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}
impl ::core::cmp::PartialEq<&str> for BSTR {
    fn eq(&self, other: &&str) -> bool {
        self.as_wide().iter().copied().eq(other.encode_utf16())
    }
}
impl ::core::cmp::PartialEq<BSTR> for &str {
    fn eq(&self, other: &BSTR) -> bool {
        other == self
    }
}
impl ::core::ops::Drop for BSTR {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { SysFreeString(self as &Self) }
        }
    }
}
unsafe impl ::windows::core::Abi for BSTR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const CLASS_E_CLASSNOTAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221231i32);
#[inline]
pub unsafe fn CloseHandle<'a, P0>(hobject: P0) -> BOOL
where
    P0: ::std::convert::Into<HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseHandle(hobject: HANDLE) -> BOOL;
    }
    CloseHandle(hobject.into())
}
pub const CO_E_NOTINITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221008i32);
pub const E_NOINTERFACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147467262i32);
pub const E_OUTOFMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024882i32);
pub const RPC_E_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147417848i32);
pub const JSCRIPT_E_CANTEXECUTE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1996357631i32);
pub type FARPROC = ::core::option::Option<unsafe extern "system" fn() -> isize>;
#[inline]
pub unsafe fn GetLastError() -> WIN32_ERROR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetLastError() -> WIN32_ERROR;
    }
    GetLastError()
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HANDLE(pub isize);
impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HANDLE {}
impl ::core::fmt::Debug for HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HANDLE>> for HANDLE {
    fn from(optional: ::core::option::Option<HANDLE>) -> HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HANDLE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HINSTANCE(pub isize);
impl HINSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for HINSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HINSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HINSTANCE {}
impl ::core::fmt::Debug for HINSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HINSTANCE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HINSTANCE>> for HINSTANCE {
    fn from(optional: ::core::option::Option<HINSTANCE>) -> HINSTANCE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HINSTANCE {
    type Abi = Self;
}
pub const S_OK: ::windows::core::HRESULT = ::windows::core::HRESULT(0i32);
#[inline]
pub unsafe fn SysAllocStringByteLen<'a, P0>(psz: P0, len: u32) -> BSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SysAllocStringByteLen(psz: ::windows::core::PCSTR, len: u32) -> BSTR;
    }
    SysAllocStringByteLen(psz.into(), len)
}
#[inline]
pub unsafe fn SysAllocStringLen(strin: &[u16]) -> BSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SysAllocStringLen(strin: ::windows::core::PCWSTR, ui: u32) -> BSTR;
    }
    SysAllocStringLen(::core::mem::transmute(::windows::core::as_ptr_or_null(strin)), strin.len() as _)
}
#[inline]
pub unsafe fn SysFreeString<'a, P0>(bstrstring: P0)
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, BSTR>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SysFreeString(bstrstring: ::core::mem::ManuallyDrop<BSTR>);
    }
    SysFreeString(bstrstring.into().abi())
}
#[inline]
pub unsafe fn SysStringLen<'a, P0>(pbstr: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, BSTR>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SysStringLen(pbstr: ::core::mem::ManuallyDrop<BSTR>) -> u32;
    }
    SysStringLen(pbstr.into().abi())
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WIN32_ERROR(pub u32);
impl ::core::marker::Copy for WIN32_ERROR {}
impl ::core::clone::Clone for WIN32_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN32_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WIN32_ERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for WIN32_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_ERROR").field(&self.0).finish()
    }
}
impl WIN32_ERROR {
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == 0
    }
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }
    #[inline]
    pub const fn to_hresult(self) -> ::windows::core::HRESULT {
        ::windows::core::HRESULT(if self.0 == 0 { self.0 } else { (self.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000 } as _)
    }
    #[inline]
    pub fn from_error(error: &::windows::core::Error) -> ::core::option::Option<Self> {
        let hresult = error.code().0 as u32;
        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(Self(hresult & 0xFFFF))
        } else {
            None
        }
    }
    #[inline]
    pub const fn ok(self) -> ::windows::core::Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(::windows::core::Error { code: self.to_hresult(), info: None })
        }
    }
}
impl ::core::convert::From<WIN32_ERROR> for ::windows::core::HRESULT {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult()
    }
}
impl ::core::convert::From<WIN32_ERROR> for ::windows::core::Error {
    fn from(value: WIN32_ERROR) -> Self {
        Self { code: value.to_hresult(), info: None }
    }
}
#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut ::core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl ::core::marker::Copy for SECURITY_ATTRIBUTES {}
impl ::core::clone::Clone for SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SECURITY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_ATTRIBUTES").field("nLength", &self.nLength).field("lpSecurityDescriptor", &self.lpSecurityDescriptor).field("bInheritHandle", &self.bInheritHandle).finish()
    }
}
unsafe impl ::windows::core::Abi for SECURITY_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SECURITY_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for SECURITY_ATTRIBUTES {}
impl ::core::default::Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn CoCreateGuid() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoCreateGuid(pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoCreateGuid(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
}
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
    }
    CoTaskMemAlloc(cb)
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: *const ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
    }
    CoTaskMemFree(::core::mem::transmute(pv))
}
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::core::Result<IErrorInfo> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetErrorInfo(dwreserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IErrorInfo>(result__)
}
#[repr(transparent)]
pub struct IAgileObject(::windows::core::IUnknown);
impl IAgileObject {}
impl ::core::convert::From<IAgileObject> for ::windows::core::IUnknown {
    fn from(value: IAgileObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAgileObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAgileObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAgileObject> for ::windows::core::IUnknown {
    fn from(value: &IAgileObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileObject {}
impl ::core::fmt::Debug for IAgileObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAgileObject {
    type Vtable = IAgileObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IErrorInfo(::windows::core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetGUID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BSTR>(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BSTR>(result__)
    }
    pub unsafe fn GetHelpFile(&self) -> ::windows::core::Result<BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHelpFile)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BSTR>(result__)
    }
    pub unsafe fn GetHelpContext(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHelpContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: IErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IErrorInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &IErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorInfo {}
impl ::core::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IErrorInfo {
    type Vtable = IErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut BSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut BSTR) -> ::windows::core::HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut BSTR) -> ::windows::core::HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT,
}
#[inline]
pub unsafe fn SetErrorInfo<'a, P0>(dwreserved: u32, perrinfo: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IErrorInfo>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetErrorInfo(dwreserved: u32, perrinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    SetErrorInfo(dwreserved, perrinfo.into().abi()).ok()
}
#[inline]
pub unsafe fn EncodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EncodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    }
    EncodePointer(::core::mem::transmute(ptr))
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FORMAT_MESSAGE_OPTIONS(pub u32);
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(256u32);
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(8192u32);
pub const FORMAT_MESSAGE_FROM_HMODULE: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(2048u32);
pub const FORMAT_MESSAGE_FROM_STRING: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(1024u32);
pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(4096u32);
pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(512u32);
impl ::core::marker::Copy for FORMAT_MESSAGE_OPTIONS {}
impl ::core::clone::Clone for FORMAT_MESSAGE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FORMAT_MESSAGE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FORMAT_MESSAGE_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FORMAT_MESSAGE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORMAT_MESSAGE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FORMAT_MESSAGE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FORMAT_MESSAGE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn FormatMessageW(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::core::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: ::windows::core::PWSTR, nsize: u32, arguments: *const *const i8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FormatMessageW(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::core::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: ::windows::core::PWSTR, nsize: u32, arguments: *const *const i8) -> u32;
    }
    FormatMessageW(dwflags, ::core::mem::transmute(lpsource), dwmessageid, dwlanguageid, ::core::mem::transmute(lpbuffer), nsize, ::core::mem::transmute(arguments))
}
#[inline]
pub unsafe fn FreeLibrary<'a, P0>(hlibmodule: P0) -> BOOL
where
    P0: ::std::convert::Into<HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreeLibrary(hlibmodule: HINSTANCE) -> BOOL;
    }
    FreeLibrary(hlibmodule.into())
}
#[inline]
pub unsafe fn GetProcAddress<'a, P0, P1>(hmodule: P0, lpprocname: P1) -> FARPROC
where
    P0: ::std::convert::Into<HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetProcAddress(hmodule: HINSTANCE, lpprocname: ::windows::core::PCSTR) -> FARPROC;
    }
    GetProcAddress(hmodule.into(), lpprocname.into())
}
#[inline]
pub unsafe fn LoadLibraryA<'a, P0>(lplibfilename: P0) -> ::windows::core::Result<HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadLibraryA(lplibfilename: ::windows::core::PCSTR) -> HINSTANCE;
    }
    let result__ = LoadLibraryA(lplibfilename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[inline]
pub unsafe fn GetProcessHeap() -> ::windows::core::Result<HeapHandle> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetProcessHeap() -> HeapHandle;
    }
    let result__ = GetProcessHeap();
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEAP_FLAGS(pub u32);
pub const HEAP_NONE: HEAP_FLAGS = HEAP_FLAGS(0u32);
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = HEAP_FLAGS(1u32);
pub const HEAP_GROWABLE: HEAP_FLAGS = HEAP_FLAGS(2u32);
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = HEAP_FLAGS(4u32);
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = HEAP_FLAGS(8u32);
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = HEAP_FLAGS(16u32);
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(32u32);
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(64u32);
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = HEAP_FLAGS(128u32);
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = HEAP_FLAGS(65536u32);
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = HEAP_FLAGS(131072u32);
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = HEAP_FLAGS(262144u32);
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = HEAP_FLAGS(4095u32);
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = HEAP_FLAGS(32768u32);
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = HEAP_FLAGS(18u32);
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = HEAP_FLAGS(256u32);
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = HEAP_FLAGS(512u32);
impl ::core::marker::Copy for HEAP_FLAGS {}
impl ::core::clone::Clone for HEAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HEAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HEAP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn HeapAlloc<'a, P0>(hheap: P0, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<HeapHandle>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HeapAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::core::ffi::c_void;
    }
    HeapAlloc(hheap.into(), dwflags, dwbytes)
}
#[inline]
pub unsafe fn HeapFree<'a, P0>(hheap: P0, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> BOOL
where
    P0: ::std::convert::Into<HeapHandle>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HeapFree(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> BOOL;
    }
    HeapFree(hheap.into(), dwflags, ::core::mem::transmute(lpmem))
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HeapHandle(pub isize);
impl HeapHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HeapHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HeapHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HeapHandle {}
impl ::core::fmt::Debug for HeapHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HeapHandle").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HeapHandle>> for HeapHandle {
    fn from(optional: ::core::option::Option<HeapHandle>) -> HeapHandle {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HeapHandle {
    type Abi = Self;
}
#[inline]
pub unsafe fn CreateEventA<'a, P0, P1, P2>(lpeventattributes: *const SECURITY_ATTRIBUTES, bmanualreset: P0, binitialstate: P1, lpname: P2) -> ::windows::core::Result<HANDLE>
where
    P0: ::std::convert::Into<BOOL>,
    P1: ::std::convert::Into<BOOL>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateEventA(lpeventattributes: *const SECURITY_ATTRIBUTES, bmanualreset: BOOL, binitialstate: BOOL, lpname: ::windows::core::PCSTR) -> HANDLE;
    }
    let result__ = CreateEventA(::core::mem::transmute(lpeventattributes), bmanualreset.into(), binitialstate.into(), lpname.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[inline]
pub unsafe fn SetEvent<'a, P0>(hevent: P0) -> BOOL
where
    P0: ::std::convert::Into<HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetEvent(hevent: HANDLE) -> BOOL;
    }
    SetEvent(hevent.into())
}
#[inline]
pub unsafe fn WaitForSingleObject<'a, P0>(hhandle: P0, dwmilliseconds: u32) -> u32
where
    P0: ::std::convert::Into<HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WaitForSingleObject(hhandle: HANDLE, dwmilliseconds: u32) -> u32;
    }
    WaitForSingleObject(hhandle.into(), dwmilliseconds)
}
#[repr(transparent)]
pub struct IAgileReference(::windows::core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).Resolve)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAgileReference> for ::windows::core::IUnknown {
    fn from(value: IAgileReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAgileReference> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAgileReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAgileReference> for ::windows::core::IUnknown {
    fn from(value: &IAgileReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAgileReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileReference {}
impl ::core::fmt::Debug for IAgileReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAgileReference {
    type Vtable = IAgileReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AgileReferenceOptions(pub i32);
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
impl ::core::marker::Copy for AgileReferenceOptions {}
impl ::core::clone::Clone for AgileReferenceOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AgileReferenceOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AgileReferenceOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AgileReferenceOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AgileReferenceOptions").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn RoGetAgileReference<'a, P0>(options: AgileReferenceOptions, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<IAgileReference>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, ppagilereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetAgileReference(options, ::core::mem::transmute(riid), punk.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAgileReference>(result__)
}
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo(::windows::core::IUnknown);
impl ILanguageExceptionErrorInfo {
    pub unsafe fn GetLanguageException(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLanguageException)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo> for ::windows::core::IUnknown {
    fn from(value: ILanguageExceptionErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionErrorInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILanguageExceptionErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &ILanguageExceptionErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILanguageExceptionErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04a2dbf3_df83_116c_0946_0812abf6e07d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2(::windows::core::IUnknown);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn GetLanguageException(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLanguageException)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPreviousLanguageExceptionErrorInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
    pub unsafe fn CapturePropagationContext<'a, P0>(&self, languageexception: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CapturePropagationContext)(::windows::core::Interface::as_raw(self), languageexception.into().abi()).ok()
    }
    pub unsafe fn GetPropagationContextHead(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPropagationContextHead)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ::windows::core::IUnknown {
    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionErrorInfo2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ::windows::core::IUnknown {
    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionErrorInfo2> for &'a ILanguageExceptionErrorInfo {
    fn from(value: &'a ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILanguageExceptionErrorInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo2 {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5746e5c4_5b97_424c_b620_2822915734dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IRestrictedErrorInfo(::windows::core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: *mut BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut BSTR, capabilitysid: *mut BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetErrorDetails)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(description), ::core::mem::transmute(error), ::core::mem::transmute(restricteddescription), ::core::mem::transmute(capabilitysid)).ok()
    }
    pub unsafe fn GetReference(&self) -> ::windows::core::Result<BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetReference)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BSTR>(result__)
    }
}
impl ::core::convert::From<IRestrictedErrorInfo> for ::windows::core::IUnknown {
    fn from(value: IRestrictedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRestrictedErrorInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRestrictedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &IRestrictedErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRestrictedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRestrictedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedErrorInfo {}
impl ::core::fmt::Debug for IRestrictedErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRestrictedErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for IRestrictedErrorInfo {}
unsafe impl ::core::marker::Sync for IRestrictedErrorInfo {}
unsafe impl ::windows::core::Interface for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetErrorDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut BSTR, capabilitysid: *mut BSTR) -> ::windows::core::HRESULT,
    pub GetReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut BSTR) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IWeakReference(::windows::core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).Resolve)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWeakReference> for ::windows::core::IUnknown {
    fn from(value: IWeakReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWeakReference> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWeakReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWeakReference> for ::windows::core::IUnknown {
    fn from(value: &IWeakReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWeakReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReference {}
impl ::core::fmt::Debug for IWeakReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWeakReference {
    type Vtable = IWeakReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000037_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IWeakReferenceSource(::windows::core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> ::windows::core::Result<IWeakReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetWeakReference)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWeakReference>(result__)
    }
}
impl ::core::convert::From<IWeakReferenceSource> for ::windows::core::IUnknown {
    fn from(value: IWeakReferenceSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWeakReferenceSource> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWeakReferenceSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWeakReferenceSource> for ::windows::core::IUnknown {
    fn from(value: &IWeakReferenceSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWeakReferenceSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReferenceSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReferenceSource {}
impl ::core::fmt::Debug for IWeakReferenceSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReferenceSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000038_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weakreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
