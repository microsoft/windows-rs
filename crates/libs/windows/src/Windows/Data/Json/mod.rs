#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArray(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonArray {
    type Vtable = IJsonArray_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c1ddb6_0cbd_4a9a_b5d3_2f852dc37e81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArray_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetObjectAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetArrayAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStringAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNumberAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetBooleanAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArrayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonArrayStatics {
    type Vtable = IJsonArrayStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb1434a9_e164_499f_93e2_8a8f49bb90ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArrayStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonErrorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonErrorStatics2 {
    type Vtable = IJsonErrorStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x404030da_87d0_436c_83ab_fc7b12c0cc26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonErrorStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetJsonStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObject {
    type Vtable = IJsonObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x064e24dd_29c2_4f83_9ac1_9ee11578beb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObject_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNamedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNamedArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNamedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNamedNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetNamedBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObjectStatics {
    type Vtable = IJsonObjectStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2289f159_54de_45d8_abcc_22603fa066a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectWithDefaultValues(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObjectWithDefaultValues {
    type Vtable = IJsonObjectWithDefaultValues_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd960d2a2_b7f0_4f00_8e44_d82cf415ea13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectWithDefaultValues_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetNamedValueOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNamedObjectOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNamedStringOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNamedArrayOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNamedNumberOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetNamedBooleanOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: bool, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct IJsonValue(::windows::core::IUnknown);
impl IJsonValue {
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stringify)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBoolean)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetArray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
}
impl ::core::convert::From<IJsonValue> for ::windows::core::IUnknown {
    fn from(value: IJsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IJsonValue> for ::windows::core::IUnknown {
    fn from(value: &IJsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IJsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IJsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IJsonValue> for ::windows::core::IInspectable {
    fn from(value: IJsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IJsonValue> for ::windows::core::IInspectable {
    fn from(value: &IJsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IJsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IJsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IJsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IJsonValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IJsonValue {
    type Vtable = IJsonValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3219ecb_f0b3_4dcd_beee_19d48cd3ed1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValue_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows::core::HRESULT,
    pub Stringify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonValueStatics {
    type Vtable = IJsonValueStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6b544a_2f53_48e1_91a3_f78b50a6345c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CreateBooleanValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateNumberValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonValueStatics2 {
    type Vtable = IJsonValueStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d9ecbe4_3fe8_4335_8392_93d8e36865f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateNullValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonArray(::windows::core::IUnknown);
impl JsonArray {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonArray, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<IJsonValue>>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetObjectAt(&self, index: u32) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObjectAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetArrayAt(&self, index: u32) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetArrayAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetStringAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStringAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNumberAt(&self, index: u32) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumberAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetBooleanAt(&self, index: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonArray> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parse)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonArray>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonArray>) -> ::windows::core::Result<bool> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryParse)(::core::mem::transmute_copy(this), input.into_param().abi(), result as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stringify)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBoolean)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetArray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<IJsonValue>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IJsonValue>>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, IJsonValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, IJsonValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IJsonValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<IJsonValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[doc(hidden)]
    pub fn IJsonArrayStatics<R, F: FnOnce(&IJsonArrayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonArray, IJsonArrayStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JsonArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for JsonArray {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonArray;{08c1ddb6-0cbd-4a9a-b5d3-2f852dc37e81})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for JsonArray {
    type Vtable = IJsonArray_Vtbl;
    const IID: ::windows::core::GUID = <IJsonArray as ::windows::core::Interface>::IID;
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
        super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<JsonArray> for ::windows::core::IUnknown {
    fn from(value: JsonArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonArray> for ::windows::core::IUnknown {
    fn from(value: &JsonArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JsonArray> for ::windows::core::IInspectable {
    fn from(value: JsonArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonArray> for ::windows::core::IInspectable {
    fn from(value: &JsonArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonArray> for super::super::Foundation::Collections::IIterable<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for super::super::Foundation::Collections::IIterable<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IJsonValue>> for JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<IJsonValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IJsonValue>> for &JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<IJsonValue>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<IJsonValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<JsonArray> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonArray> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IJsonValue> for JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, IJsonValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IJsonValue> for &JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, IJsonValue> {
        ::core::convert::TryInto::<IJsonValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<JsonArray> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonArray> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonArray> for super::super::Foundation::Collections::IVector<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for super::super::Foundation::Collections::IVector<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<IJsonValue>> for JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVector<IJsonValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<IJsonValue>> for &JsonArray {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVector<IJsonValue>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IVector<IJsonValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JsonArray {}
unsafe impl ::core::marker::Sync for JsonArray {}
#[doc = "*Required features: `\"Data_Json\"`*"]
pub struct JsonError {}
impl JsonError {
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetJsonStatus(hresult: i32) -> ::windows::core::Result<JsonErrorStatus> {
        Self::IJsonErrorStatics2(|this| unsafe {
            let mut result__: JsonErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetJsonStatus)(::core::mem::transmute_copy(this), hresult, &mut result__).from_abi::<JsonErrorStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJsonErrorStatics2<R, F: FnOnce(&IJsonErrorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonError, IJsonErrorStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for JsonError {
    const NAME: &'static str = "Windows.Data.Json.JsonError";
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for JsonErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsonErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonObject(::windows::core::IUnknown);
impl JsonObject {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonObject, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<JsonValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedValue)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn SetNamedValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNamedValue)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedObject)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedArray)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedString)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedNumber)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedBoolean<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedBoolean)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonObject> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parse)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonObject>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonObject>) -> ::windows::core::Result<bool> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryParse)(::core::mem::transmute_copy(this), input.into_param().abi(), result as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedValueOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, JsonValue>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<JsonValue> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedValueOrDefault)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedObjectOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, JsonObject>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedObjectOrDefault)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedStringOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedStringOrDefault)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedArrayOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, JsonArray>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedArrayOrDefault)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedNumberOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, defaultvalue: f64) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedNumberOrDefault)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNamedBooleanOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, defaultvalue: bool) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNamedBooleanOrDefault)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stringify)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBoolean)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetArray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<IJsonValue>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IJsonValue>>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Insert)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IJsonObjectStatics<R, F: FnOnce(&IJsonObjectStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonObject, IJsonObjectStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JsonObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for JsonObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonObject;{064e24dd-29c2-4f83-9ac1-9ee11578beb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for JsonObject {
    type Vtable = IJsonObject_Vtbl;
    const IID: ::windows::core::GUID = <IJsonObject as ::windows::core::Interface>::IID;
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
impl ::core::convert::From<JsonObject> for ::windows::core::IUnknown {
    fn from(value: JsonObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonObject> for ::windows::core::IUnknown {
    fn from(value: &JsonObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JsonObject> for ::windows::core::IInspectable {
    fn from(value: JsonObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonObject> for ::windows::core::IInspectable {
    fn from(value: &JsonObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonObject> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> for JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> for &JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<JsonObject> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonObject> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IJsonValue> for JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, IJsonValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IJsonValue> for &JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, IJsonValue> {
        ::core::convert::TryInto::<IJsonValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonObject> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>> for JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>> for &JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<JsonObject> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonObject> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &JsonObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JsonObject {}
unsafe impl ::core::marker::Sync for JsonObject {}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonValue(::windows::core::IUnknown);
impl JsonValue {
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stringify)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBoolean)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetArray)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parse)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonValue>) -> ::windows::core::Result<bool> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryParse)(::core::mem::transmute_copy(this), input.into_param().abi(), result as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn CreateBooleanValue(input: bool) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateBooleanValue)(::core::mem::transmute_copy(this), input, &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn CreateNumberValue(input: f64) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateNumberValue)(::core::mem::transmute_copy(this), input, &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn CreateStringValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStringValue)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`*"]
    pub fn CreateNullValue() -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateNullValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Json\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IJsonValueStatics<R, F: FnOnce(&IJsonValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonValue, IJsonValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IJsonValueStatics2<R, F: FnOnce(&IJsonValueStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonValue, IJsonValueStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for JsonValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonValue;{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for JsonValue {
    type Vtable = IJsonValue_Vtbl;
    const IID: ::windows::core::GUID = <IJsonValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JsonValue {
    const NAME: &'static str = "Windows.Data.Json.JsonValue";
}
impl ::core::convert::From<JsonValue> for ::windows::core::IUnknown {
    fn from(value: JsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonValue> for ::windows::core::IUnknown {
    fn from(value: &JsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JsonValue> for ::windows::core::IInspectable {
    fn from(value: JsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonValue> for ::windows::core::IInspectable {
    fn from(value: &JsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<JsonValue> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonValue> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IJsonValue> for JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, IJsonValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IJsonValue> for &JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, IJsonValue> {
        ::core::convert::TryInto::<IJsonValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<JsonValue> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonValue> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &JsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JsonValue {}
unsafe impl ::core::marker::Sync for JsonValue {}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for JsonValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsonValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValueType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonValueType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonValueType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
