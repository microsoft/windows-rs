#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArray(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonArray {
    type Vtable = IJsonArray_Vtbl;
}
impl ::core::clone::Clone for IJsonArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c1ddb6_0cbd_4a9a_b5d3_2f852dc37e81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArray_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetObjectAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetArrayAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStringAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNumberAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetBooleanAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArrayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonArrayStatics {
    type Vtable = IJsonArrayStatics_Vtbl;
}
impl ::core::clone::Clone for IJsonArrayStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonArrayStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb1434a9_e164_499f_93e2_8a8f49bb90ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArrayStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonErrorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonErrorStatics2 {
    type Vtable = IJsonErrorStatics2_Vtbl;
}
impl ::core::clone::Clone for IJsonErrorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonErrorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x404030da_87d0_436c_83ab_fc7b12c0cc26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonErrorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetJsonStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObject {
    type Vtable = IJsonObject_Vtbl;
}
impl ::core::clone::Clone for IJsonObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x064e24dd_29c2_4f83_9ac1_9ee11578beb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObject_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNamedNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetNamedBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObjectStatics {
    type Vtable = IJsonObjectStatics_Vtbl;
}
impl ::core::clone::Clone for IJsonObjectStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonObjectStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2289f159_54de_45d8_abcc_22603fa066a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectWithDefaultValues(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObjectWithDefaultValues {
    type Vtable = IJsonObjectWithDefaultValues_Vtbl;
}
impl ::core::clone::Clone for IJsonObjectWithDefaultValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonObjectWithDefaultValues {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd960d2a2_b7f0_4f00_8e44_d82cf415ea13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectWithDefaultValues_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetNamedValueOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedObjectOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedStringOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, defaultvalue: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNamedArrayOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedNumberOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, defaultvalue: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetNamedBooleanOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, defaultvalue: bool, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct IJsonValue(::windows::core::IUnknown);
impl IJsonValue {
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValueType>();
            (::windows::core::Interface::vtable(this).ValueType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Stringify)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).GetNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetBoolean)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).GetArray)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).GetObject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IJsonValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IJsonValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsonValue {}
impl ::core::fmt::Debug for IJsonValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsonValue").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IJsonValue {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e}");
}
unsafe impl ::windows::core::Interface for IJsonValue {
    type Vtable = IJsonValue_Vtbl;
}
impl ::core::clone::Clone for IJsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3219ecb_f0b3_4dcd_beee_19d48cd3ed1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows::core::HRESULT,
    pub Stringify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonValueStatics {
    type Vtable = IJsonValueStatics_Vtbl;
}
impl ::core::clone::Clone for IJsonValueStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6b544a_2f53_48e1_91a3_f78b50a6345c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CreateBooleanValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNumberValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonValueStatics2 {
    type Vtable = IJsonValueStatics2_Vtbl;
}
impl ::core::clone::Clone for IJsonValueStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsonValueStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d9ecbe4_3fe8_4335_8392_93d8e36865f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateNullValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonArray(::windows::core::IUnknown);
impl JsonArray {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<JsonArray, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IJsonValue>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<IJsonValue>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetObjectAt(&self, index: u32) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).GetObjectAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn GetArrayAt(&self, index: u32) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).GetArrayAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn GetStringAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetStringAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn GetNumberAt(&self, index: u32) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).GetNumberAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn GetBooleanAt(&self, index: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetBooleanAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).Parse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonArray>) -> ::windows::core::Result<bool> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryParse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), result as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValueType>();
            (::windows::core::Interface::vtable(this).ValueType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Stringify)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).GetNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetBoolean)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).GetArray)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).GetObject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IJsonValue>();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IJsonValue>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<IJsonValue>>();
            (::windows::core::Interface::vtable(this).GetView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::TryIntoParam<IJsonValue>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IJsonValue>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::windows::core::Interface::as_raw(this), index, value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IJsonValue>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::windows::core::Interface::as_raw(this), index, value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::windows::core::Interface::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IJsonValue>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IJsonValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<IJsonValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::windows::core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[doc(hidden)]
    pub fn IJsonArrayStatics<R, F: FnOnce(&IJsonArrayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<JsonArray, IJsonArrayStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for JsonArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonArray {}
impl ::core::fmt::Debug for JsonArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonArray").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for JsonArray {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonArray;{08c1ddb6-0cbd-4a9a-b5d3-2f852dc37e81})");
}
impl ::core::clone::Clone for JsonArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for JsonArray {
    type Vtable = IJsonArray_Vtbl;
}
unsafe impl ::windows::core::ComInterface for JsonArray {
    const IID: ::windows::core::GUID = <IJsonArray as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for JsonArray {
    const NAME: &'static str = "Windows.Data.Json.JsonArray";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for JsonArray {
    type Item = IJsonValue;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &JsonArray {
    type Item = IJsonValue;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(::windows::core::ComInterface::cast(self).ok())
    }
}
::windows::imp::interface_hierarchy!(JsonArray, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<IJsonValue>> for JsonArray {}
impl ::windows::core::CanTryInto<IJsonValue> for JsonArray {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for JsonArray {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IVector<IJsonValue>> for JsonArray {}
unsafe impl ::core::marker::Send for JsonArray {}
unsafe impl ::core::marker::Sync for JsonArray {}
#[doc = "*Required features: `\"Data_Json\"`*"]
pub struct JsonError;
impl JsonError {
    pub fn GetJsonStatus(hresult: i32) -> ::windows::core::Result<JsonErrorStatus> {
        Self::IJsonErrorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonErrorStatus>();
            (::windows::core::Interface::vtable(this).GetJsonStatus)(::windows::core::Interface::as_raw(this), hresult, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJsonErrorStatics2<R, F: FnOnce(&IJsonErrorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<JsonError, IJsonErrorStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for JsonError {
    const NAME: &'static str = "Windows.Data.Json.JsonError";
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonObject(::windows::core::IUnknown);
impl JsonObject {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<JsonObject, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedValue(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValue>();
            (::windows::core::Interface::vtable(this).GetNamedValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNamedValue<P0>(&self, name: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IJsonValue>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNamedValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.try_into_param()?.abi()).ok() }
    }
    pub fn GetNamedObject(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).GetNamedObject)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedArray(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).GetNamedArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedString(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetNamedString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedNumber(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).GetNamedNumber)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedBoolean(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetNamedBoolean)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).Parse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonObject>) -> ::windows::core::Result<bool> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryParse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), result as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    pub fn GetNamedValueOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &JsonValue) -> ::windows::core::Result<JsonValue> {
        let this = &::windows::core::ComInterface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValue>();
            (::windows::core::Interface::vtable(this).GetNamedValueOrDefault)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedObjectOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &JsonObject) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::ComInterface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).GetNamedObjectOrDefault)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedStringOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetNamedStringOrDefault)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedArrayOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &JsonArray) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::ComInterface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).GetNamedArrayOrDefault)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedNumberOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: f64) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).GetNamedNumberOrDefault)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), defaultvalue, &mut result__).from_abi(result__)
        }
    }
    pub fn GetNamedBooleanOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: bool) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetNamedBooleanOrDefault)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), defaultvalue, &mut result__).from_abi(result__)
        }
    }
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValueType>();
            (::windows::core::Interface::vtable(this).ValueType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Stringify)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).GetNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetBoolean)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).GetArray)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::ComInterface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).GetObject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IJsonValue>();
            (::windows::core::Interface::vtable(this).Lookup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasKey)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IJsonValue>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IJsonValue>>();
            (::windows::core::Interface::vtable(this).GetView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::TryIntoParam<IJsonValue>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Insert)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IJsonObjectStatics<R, F: FnOnce(&IJsonObjectStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<JsonObject, IJsonObjectStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for JsonObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonObject {}
impl ::core::fmt::Debug for JsonObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonObject").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for JsonObject {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonObject;{064e24dd-29c2-4f83-9ac1-9ee11578beb3})");
}
impl ::core::clone::Clone for JsonObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for JsonObject {
    type Vtable = IJsonObject_Vtbl;
}
unsafe impl ::windows::core::ComInterface for JsonObject {
    const IID: ::windows::core::GUID = <IJsonObject as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for JsonObject {
    const NAME: &'static str = "Windows.Data.Json.JsonObject";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for JsonObject {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &JsonObject {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::imp::interface_hierarchy!(JsonObject, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> for JsonObject {}
impl ::windows::core::CanTryInto<IJsonValue> for JsonObject {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>> for JsonObject {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for JsonObject {}
unsafe impl ::core::marker::Send for JsonObject {}
unsafe impl ::core::marker::Sync for JsonObject {}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonValue(::windows::core::IUnknown);
impl JsonValue {
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValueType>();
            (::windows::core::Interface::vtable(this).ValueType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Stringify)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).GetNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetBoolean)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonArray>();
            (::windows::core::Interface::vtable(this).GetArray)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonObject>();
            (::windows::core::Interface::vtable(this).GetObject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValue>();
            (::windows::core::Interface::vtable(this).Parse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonValue>) -> ::windows::core::Result<bool> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TryParse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), result as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateBooleanValue(input: bool) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValue>();
            (::windows::core::Interface::vtable(this).CreateBooleanValue)(::windows::core::Interface::as_raw(this), input, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateNumberValue(input: f64) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValue>();
            (::windows::core::Interface::vtable(this).CreateNumberValue)(::windows::core::Interface::as_raw(this), input, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateStringValue(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValue>();
            (::windows::core::Interface::vtable(this).CreateStringValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateNullValue() -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<JsonValue>();
            (::windows::core::Interface::vtable(this).CreateNullValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IJsonValueStatics<R, F: FnOnce(&IJsonValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<JsonValue, IJsonValueStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IJsonValueStatics2<R, F: FnOnce(&IJsonValueStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<JsonValue, IJsonValueStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for JsonValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonValue {}
impl ::core::fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValue").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for JsonValue {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonValue;{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e})");
}
impl ::core::clone::Clone for JsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for JsonValue {
    type Vtable = IJsonValue_Vtbl;
}
unsafe impl ::windows::core::ComInterface for JsonValue {
    const IID: ::windows::core::GUID = <IJsonValue as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for JsonValue {
    const NAME: &'static str = "Windows.Data.Json.JsonValue";
}
::windows::imp::interface_hierarchy!(JsonValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IJsonValue> for JsonValue {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for JsonValue {}
unsafe impl ::core::marker::Send for JsonValue {}
unsafe impl ::core::marker::Sync for JsonValue {}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsonErrorStatus(pub i32);
impl JsonErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const InvalidJsonString: Self = Self(1i32);
    pub const InvalidJsonNumber: Self = Self(2i32);
    pub const JsonValueNotFound: Self = Self(3i32);
    pub const ImplementationLimit: Self = Self(4i32);
}
impl ::core::marker::Copy for JsonErrorStatus {}
impl ::core::clone::Clone for JsonErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsonErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for JsonErrorStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for JsonErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonErrorStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for JsonErrorStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonErrorStatus;i4)");
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsonValueType(pub i32);
impl JsonValueType {
    pub const Null: Self = Self(0i32);
    pub const Boolean: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const String: Self = Self(3i32);
    pub const Array: Self = Self(4i32);
    pub const Object: Self = Self(5i32);
}
impl ::core::marker::Copy for JsonValueType {}
impl ::core::clone::Clone for JsonValueType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsonValueType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for JsonValueType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for JsonValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValueType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for JsonValueType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonValueType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
