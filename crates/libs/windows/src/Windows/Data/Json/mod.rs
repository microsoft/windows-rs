#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArray(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonArray {
    type Vtable = IJsonArrayVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c1ddb6_0cbd_4a9a_b5d3_2f852dc37e81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArrayVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArrayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonArrayStatics {
    type Vtable = IJsonArrayStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb1434a9_e164_499f_93e2_8a8f49bb90ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArrayStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonErrorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonErrorStatics2 {
    type Vtable = IJsonErrorStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x404030da_87d0_436c_83ab_fc7b12c0cc26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonErrorStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObject {
    type Vtable = IJsonObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x064e24dd_29c2_4f83_9ac1_9ee11578beb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObjectStatics {
    type Vtable = IJsonObjectStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2289f159_54de_45d8_abcc_22603fa066a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectWithDefaultValues(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonObjectWithDefaultValues {
    type Vtable = IJsonObjectWithDefaultValuesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd960d2a2_b7f0_4f00_8e44_d82cf415ea13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectWithDefaultValuesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultvalue: bool, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Data_Json'*"]
#[repr(transparent)]
pub struct IJsonValue(::windows::core::IUnknown);
impl IJsonValue {
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IJsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IJsonValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
}
unsafe impl ::windows::core::Interface for IJsonValue {
    type Vtable = IJsonValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3219ecb_f0b3_4dcd_beee_19d48cd3ed1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonValueStatics {
    type Vtable = IJsonValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6b544a_2f53_48e1_91a3_f78b50a6345c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJsonValueStatics2 {
    type Vtable = IJsonValueStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d9ecbe4_3fe8_4335_8392_93d8e36865f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Data_Json'*"]
#[repr(transparent)]
pub struct JsonArray(::windows::core::IUnknown);
impl JsonArray {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonArray, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<IJsonValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetObjectAt(&self, index: u32) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetArrayAt(&self, index: u32) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetStringAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNumberAt(&self, index: u32) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetBooleanAt(&self, index: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonArray> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonArray>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonArray>) -> ::windows::core::Result<bool> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), result as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<IJsonValue>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IJsonValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, IJsonValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, IJsonValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<IJsonValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<IJsonValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
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
}
unsafe impl ::windows::core::Interface for JsonArray {
    type Vtable = IJsonArrayVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c1ddb6_0cbd_4a9a_b5d3_2f852dc37e81);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &JsonArray {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &JsonArray {
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
#[doc = "*Required features: 'Data_Json'*"]
pub struct JsonError {}
impl JsonError {
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetJsonStatus(hresult: i32) -> ::windows::core::Result<JsonErrorStatus> {
        Self::IJsonErrorStatics2(|this| unsafe {
            let mut result__: JsonErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), hresult, &mut result__).from_abi::<JsonErrorStatus>(result__)
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
#[doc = "*Required features: 'Data_Json'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for JsonErrorStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JsonErrorStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonErrorStatus {}
impl ::core::fmt::Debug for JsonErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonErrorStatus;i4)");
}
impl ::windows::core::DefaultType for JsonErrorStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Data_Json'*"]
#[repr(transparent)]
pub struct JsonObject(::windows::core::IUnknown);
impl JsonObject {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<JsonObject, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<JsonValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn SetNamedValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedBoolean<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonObject> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonObject>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonObject>) -> ::windows::core::Result<bool> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), result as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedValueOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, JsonValue>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<JsonValue> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedObjectOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, JsonObject>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedStringOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedArrayOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, JsonArray>>(&self, name: Param0, defaultvalue: Param1) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue.into_param().abi(), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedNumberOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, defaultvalue: f64) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNamedBooleanOrDefault<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, defaultvalue: bool) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi(), defaultvalue, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<IJsonValue>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IJsonValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, IJsonValue>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
}
unsafe impl ::windows::core::Interface for JsonObject {
    type Vtable = IJsonObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x064e24dd_29c2_4f83_9ac1_9ee11578beb3);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &JsonObject {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &JsonObject {
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
#[doc = "*Required features: 'Data_Json'*"]
#[repr(transparent)]
pub struct JsonValue(::windows::core::IUnknown);
impl JsonValue {
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__: JsonValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValueType>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonArray>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonObject>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonValue>) -> ::windows::core::Result<bool> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), result as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn CreateBooleanValue(input: bool) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), input, &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn CreateNumberValue(input: f64) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), input, &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn CreateStringValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json'*"]
    pub fn CreateNullValue() -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JsonValue>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Json', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
}
unsafe impl ::windows::core::Interface for JsonValue {
    type Vtable = IJsonValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3219ecb_f0b3_4dcd_beee_19d48cd3ed1e);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &JsonValue {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &JsonValue {
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
#[doc = "*Required features: 'Data_Json'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for JsonValueType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JsonValueType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonValueType {}
impl ::core::fmt::Debug for JsonValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValueType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonValueType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonValueType;i4)");
}
impl ::windows::core::DefaultType for JsonValueType {
    type DefaultType = Self;
}
