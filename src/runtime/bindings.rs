#[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub mod Windows {
    pub mod Foundation {
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct DateTime {
            pub UniversalTime: i64,
        }
        impl DateTime {}
        impl ::core::default::Default for DateTime {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::fmt::Debug for DateTime {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                fmt.debug_struct("DateTime").field("UniversalTime", &self.UniversalTime).finish()
            }
        }
        impl ::core::cmp::PartialEq for DateTime {
            fn eq(&self, other: &Self) -> bool {
                self.UniversalTime == other.UniversalTime
            }
        }
        impl ::core::cmp::Eq for DateTime {}
        unsafe impl ::windows::runtime::Abi for DateTime {
            type Abi = Self;
        }
        unsafe impl ::windows::runtime::RuntimeType for DateTime {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
        }
        impl ::windows::runtime::DefaultType for DateTime {
            type DefaultType = Self;
        }
        #[repr(transparent)]
        #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
        pub struct IPropertyValue(pub ::windows::runtime::IInspectable);
        unsafe impl ::windows::runtime::Interface for IPropertyValue {
            type Vtable = IPropertyValue_abi;
            const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1272349405, 30036, 16617, [154, 155, 130, 101, 78, 222, 126, 98]);
        }
        impl IPropertyValue {
            pub fn Type(&self) -> ::windows::runtime::Result<PropertyType> {
                let this = self;
                unsafe {
                    let mut result__: PropertyType = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
                }
            }
            pub fn IsNumericScalar(&self) -> ::windows::runtime::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: bool = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
                }
            }
            pub fn GetUInt8(&self) -> ::windows::runtime::Result<u8> {
                let this = self;
                unsafe {
                    let mut result__: u8 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
                }
            }
            pub fn GetInt16(&self) -> ::windows::runtime::Result<i16> {
                let this = self;
                unsafe {
                    let mut result__: i16 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
                }
            }
            pub fn GetUInt16(&self) -> ::windows::runtime::Result<u16> {
                let this = self;
                unsafe {
                    let mut result__: u16 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
                }
            }
            pub fn GetInt32(&self) -> ::windows::runtime::Result<i32> {
                let this = self;
                unsafe {
                    let mut result__: i32 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
                }
            }
            pub fn GetUInt32(&self) -> ::windows::runtime::Result<u32> {
                let this = self;
                unsafe {
                    let mut result__: u32 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
                }
            }
            pub fn GetInt64(&self) -> ::windows::runtime::Result<i64> {
                let this = self;
                unsafe {
                    let mut result__: i64 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
                }
            }
            pub fn GetUInt64(&self) -> ::windows::runtime::Result<u64> {
                let this = self;
                unsafe {
                    let mut result__: u64 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
                }
            }
            pub fn GetSingle(&self) -> ::windows::runtime::Result<f32> {
                let this = self;
                unsafe {
                    let mut result__: f32 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
                }
            }
            pub fn GetDouble(&self) -> ::windows::runtime::Result<f64> {
                let this = self;
                unsafe {
                    let mut result__: f64 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
                }
            }
            pub fn GetChar16(&self) -> ::windows::runtime::Result<u16> {
                let this = self;
                unsafe {
                    let mut result__: u16 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
                }
            }
            pub fn GetBoolean(&self) -> ::windows::runtime::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: bool = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
                }
            }
            pub fn GetString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
                }
            }
            pub fn GetGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
                let this = self;
                unsafe {
                    let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
                }
            }
            pub fn GetDateTime(&self) -> ::windows::runtime::Result<DateTime> {
                let this = self;
                unsafe {
                    let mut result__: DateTime = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
                }
            }
            pub fn GetTimeSpan(&self) -> ::windows::runtime::Result<TimeSpan> {
                let this = self;
                unsafe {
                    let mut result__: TimeSpan = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
                }
            }
            pub fn GetPoint(&self) -> ::windows::runtime::Result<Point> {
                let this = self;
                unsafe {
                    let mut result__: Point = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
                }
            }
            pub fn GetSize(&self) -> ::windows::runtime::Result<Size> {
                let this = self;
                unsafe {
                    let mut result__: Size = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
                }
            }
            pub fn GetRect(&self) -> ::windows::runtime::Result<Rect> {
                let this = self;
                unsafe {
                    let mut result__: Rect = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
                }
            }
            pub fn GetUInt8Array(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInt16Array(&self, value: &mut ::windows::runtime::Array<i16>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetUInt16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInt32Array(&self, value: &mut ::windows::runtime::Array<i32>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetUInt32Array(&self, value: &mut ::windows::runtime::Array<u32>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInt64Array(&self, value: &mut ::windows::runtime::Array<i64>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetUInt64Array(&self, value: &mut ::windows::runtime::Array<u64>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetSingleArray(&self, value: &mut ::windows::runtime::Array<f32>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetDoubleArray(&self, value: &mut ::windows::runtime::Array<f64>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetChar16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetBooleanArray(&self, value: &mut ::windows::runtime::Array<bool>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetStringArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::HSTRING>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInspectableArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetGuidArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::GUID>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetDateTimeArray(&self, value: &mut ::windows::runtime::Array<DateTime>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetTimeSpanArray(&self, value: &mut ::windows::runtime::Array<TimeSpan>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetPointArray(&self, value: &mut ::windows::runtime::Array<Point>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetSizeArray(&self, value: &mut ::windows::runtime::Array<Size>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetRectArray(&self, value: &mut ::windows::runtime::Array<Rect>) -> ::windows::runtime::Result<()> {
                let this = self;
                unsafe { (::windows::runtime::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
        }
        unsafe impl ::windows::runtime::RuntimeType for IPropertyValue {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4bd682dd-7554-40e9-9a9b-82654ede7e62}");
        }
        impl ::core::convert::From<IPropertyValue> for ::windows::runtime::IUnknown {
            fn from(value: IPropertyValue) -> Self {
                value.0 .0
            }
        }
        impl ::core::convert::From<&IPropertyValue> for ::windows::runtime::IUnknown {
            fn from(value: &IPropertyValue) -> Self {
                value.0 .0.clone()
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyValue {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(self.0 .0)
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyValue {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Borrowed(&self.0 .0)
            }
        }
        impl ::core::convert::From<IPropertyValue> for ::windows::runtime::IInspectable {
            fn from(value: IPropertyValue) -> Self {
                value.0
            }
        }
        impl ::core::convert::From<&IPropertyValue> for ::windows::runtime::IInspectable {
            fn from(value: &IPropertyValue) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPropertyValue {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPropertyValue {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IPropertyValue_abi(
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PropertyType) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i16) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DateTime) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TimeSpan) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Point) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Size) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Rect) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::runtime::HRESULT,
        );
        #[repr(transparent)]
        #[doc(hidden)]
        pub struct IPropertyValueStatics(pub ::windows::runtime::IInspectable);
        unsafe impl ::windows::runtime::Interface for IPropertyValueStatics {
            type Vtable = IPropertyValueStatics_abi;
            const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1654381512, 55602, 20468, [150, 185, 141, 150, 197, 193, 232, 88]);
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IPropertyValueStatics_abi(
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Size, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const i16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const i64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const Size, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
        );
        #[repr(transparent)]
        #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
        pub struct IReference<T>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<T>)
        where
            T: ::windows::runtime::RuntimeType + 'static;
        unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IReference<T> {
            type Vtable = IReference_abi<T>;
            const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IReference<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
        }
        impl<T: ::windows::runtime::RuntimeType + 'static> IReference<T> {
            pub fn Value(&self) -> ::windows::runtime::Result<T> {
                let this = self;
                unsafe {
                    let mut result__: <T as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<T>(result__)
                }
            }
            pub fn Type(&self) -> ::windows::runtime::Result<PropertyType> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: PropertyType = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
                }
            }
            pub fn IsNumericScalar(&self) -> ::windows::runtime::Result<bool> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: bool = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
                }
            }
            pub fn GetUInt8(&self) -> ::windows::runtime::Result<u8> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: u8 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
                }
            }
            pub fn GetInt16(&self) -> ::windows::runtime::Result<i16> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: i16 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
                }
            }
            pub fn GetUInt16(&self) -> ::windows::runtime::Result<u16> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: u16 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
                }
            }
            pub fn GetInt32(&self) -> ::windows::runtime::Result<i32> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: i32 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
                }
            }
            pub fn GetUInt32(&self) -> ::windows::runtime::Result<u32> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: u32 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
                }
            }
            pub fn GetInt64(&self) -> ::windows::runtime::Result<i64> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: i64 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
                }
            }
            pub fn GetUInt64(&self) -> ::windows::runtime::Result<u64> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: u64 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
                }
            }
            pub fn GetSingle(&self) -> ::windows::runtime::Result<f32> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: f32 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
                }
            }
            pub fn GetDouble(&self) -> ::windows::runtime::Result<f64> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: f64 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
                }
            }
            pub fn GetChar16(&self) -> ::windows::runtime::Result<u16> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: u16 = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
                }
            }
            pub fn GetBoolean(&self) -> ::windows::runtime::Result<bool> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: bool = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
                }
            }
            pub fn GetString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
                }
            }
            pub fn GetGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
                }
            }
            pub fn GetDateTime(&self) -> ::windows::runtime::Result<DateTime> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: DateTime = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
                }
            }
            pub fn GetTimeSpan(&self) -> ::windows::runtime::Result<TimeSpan> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: TimeSpan = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
                }
            }
            pub fn GetPoint(&self) -> ::windows::runtime::Result<Point> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: Point = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
                }
            }
            pub fn GetSize(&self) -> ::windows::runtime::Result<Size> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: Size = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
                }
            }
            pub fn GetRect(&self) -> ::windows::runtime::Result<Rect> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__: Rect = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
                }
            }
            pub fn GetUInt8Array(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInt16Array(&self, value: &mut ::windows::runtime::Array<i16>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetUInt16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInt32Array(&self, value: &mut ::windows::runtime::Array<i32>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetUInt32Array(&self, value: &mut ::windows::runtime::Array<u32>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInt64Array(&self, value: &mut ::windows::runtime::Array<i64>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetUInt64Array(&self, value: &mut ::windows::runtime::Array<u64>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetSingleArray(&self, value: &mut ::windows::runtime::Array<f32>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetDoubleArray(&self, value: &mut ::windows::runtime::Array<f64>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetChar16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetBooleanArray(&self, value: &mut ::windows::runtime::Array<bool>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetStringArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::HSTRING>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetInspectableArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetGuidArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::GUID>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetDateTimeArray(&self, value: &mut ::windows::runtime::Array<DateTime>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetTimeSpanArray(&self, value: &mut ::windows::runtime::Array<TimeSpan>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetPointArray(&self, value: &mut ::windows::runtime::Array<Point>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetSizeArray(&self, value: &mut ::windows::runtime::Array<Size>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
            pub fn GetRectArray(&self, value: &mut ::windows::runtime::Array<Rect>) -> ::windows::runtime::Result<()> {
                let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
                unsafe { (::windows::runtime::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
            }
        }
        unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IReference<T> {
            const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
        }
        impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::runtime::IUnknown {
            fn from(value: IReference<T>) -> Self {
                value.0 .0
            }
        }
        impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::runtime::IUnknown {
            fn from(value: &IReference<T>) -> Self {
                value.0 .0.clone()
            }
        }
        impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReference<T> {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(self.0 .0)
            }
        }
        impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReference<T> {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Borrowed(&self.0 .0)
            }
        }
        impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::runtime::IInspectable {
            fn from(value: IReference<T>) -> Self {
                value.0
            }
        }
        impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::runtime::IInspectable {
            fn from(value: &IReference<T>) -> Self {
                value.0.clone()
            }
        }
        impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IReference<T> {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Owned(self.0)
            }
        }
        impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IReference<T> {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Borrowed(&self.0)
            }
        }
        impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IReference<T>> for IPropertyValue {
            type Error = ::windows::runtime::Error;
            fn try_from(value: IReference<T>) -> ::windows::runtime::Result<Self> {
                ::core::convert::TryFrom::try_from(&value)
            }
        }
        impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IReference<T>> for IPropertyValue {
            type Error = ::windows::runtime::Error;
            fn try_from(value: &IReference<T>) -> ::windows::runtime::Result<Self> {
                ::windows::runtime::Interface::cast(value)
            }
        }
        impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IPropertyValue> for IReference<T> {
            fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyValue> {
                ::windows::runtime::IntoParam::into_param(&self)
            }
        }
        impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IPropertyValue> for &IReference<T> {
            fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyValue> {
                ::core::convert::TryInto::<IPropertyValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IReference_abi<T>(
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
            pub ::core::marker::PhantomData<T>,
        )
        where
            T: ::windows::runtime::RuntimeType + 'static;
        #[repr(transparent)]
        #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
        pub struct IStringable(pub ::windows::runtime::IInspectable);
        unsafe impl ::windows::runtime::Interface for IStringable {
            type Vtable = IStringable_abi;
            const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520162132, 36534, 18672, [171, 206, 193, 178, 17, 230, 39, 195]);
        }
        impl IStringable {
            pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
                }
            }
        }
        unsafe impl ::windows::runtime::RuntimeType for IStringable {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
        }
        impl ::core::convert::From<IStringable> for ::windows::runtime::IUnknown {
            fn from(value: IStringable) -> Self {
                value.0 .0
            }
        }
        impl ::core::convert::From<&IStringable> for ::windows::runtime::IUnknown {
            fn from(value: &IStringable) -> Self {
                value.0 .0.clone()
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStringable {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(self.0 .0)
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStringable {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Borrowed(&self.0 .0)
            }
        }
        impl ::core::convert::From<IStringable> for ::windows::runtime::IInspectable {
            fn from(value: IStringable) -> Self {
                value.0
            }
        }
        impl ::core::convert::From<&IStringable> for ::windows::runtime::IInspectable {
            fn from(value: &IStringable) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IStringable {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IStringable {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStringable_abi(
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
        );
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct Point {
            pub X: f32,
            pub Y: f32,
        }
        impl Point {}
        impl ::core::default::Default for Point {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::fmt::Debug for Point {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                fmt.debug_struct("Point").field("X", &self.X).field("Y", &self.Y).finish()
            }
        }
        impl ::core::cmp::PartialEq for Point {
            fn eq(&self, other: &Self) -> bool {
                self.X == other.X && self.Y == other.Y
            }
        }
        impl ::core::cmp::Eq for Point {}
        unsafe impl ::windows::runtime::Abi for Point {
            type Abi = Self;
        }
        unsafe impl ::windows::runtime::RuntimeType for Point {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
        }
        impl ::windows::runtime::DefaultType for Point {
            type DefaultType = Self;
        }
        #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: core :: fmt :: Debug)]
        #[repr(transparent)]
        pub struct PropertyType(pub i32);
        impl PropertyType {
            pub const Empty: PropertyType = PropertyType(0i32);
            pub const UInt8: PropertyType = PropertyType(1i32);
            pub const Int16: PropertyType = PropertyType(2i32);
            pub const UInt16: PropertyType = PropertyType(3i32);
            pub const Int32: PropertyType = PropertyType(4i32);
            pub const UInt32: PropertyType = PropertyType(5i32);
            pub const Int64: PropertyType = PropertyType(6i32);
            pub const UInt64: PropertyType = PropertyType(7i32);
            pub const Single: PropertyType = PropertyType(8i32);
            pub const Double: PropertyType = PropertyType(9i32);
            pub const Char16: PropertyType = PropertyType(10i32);
            pub const Boolean: PropertyType = PropertyType(11i32);
            pub const String: PropertyType = PropertyType(12i32);
            pub const Inspectable: PropertyType = PropertyType(13i32);
            pub const DateTime: PropertyType = PropertyType(14i32);
            pub const TimeSpan: PropertyType = PropertyType(15i32);
            pub const Guid: PropertyType = PropertyType(16i32);
            pub const Point: PropertyType = PropertyType(17i32);
            pub const Size: PropertyType = PropertyType(18i32);
            pub const Rect: PropertyType = PropertyType(19i32);
            pub const OtherType: PropertyType = PropertyType(20i32);
            pub const UInt8Array: PropertyType = PropertyType(1025i32);
            pub const Int16Array: PropertyType = PropertyType(1026i32);
            pub const UInt16Array: PropertyType = PropertyType(1027i32);
            pub const Int32Array: PropertyType = PropertyType(1028i32);
            pub const UInt32Array: PropertyType = PropertyType(1029i32);
            pub const Int64Array: PropertyType = PropertyType(1030i32);
            pub const UInt64Array: PropertyType = PropertyType(1031i32);
            pub const SingleArray: PropertyType = PropertyType(1032i32);
            pub const DoubleArray: PropertyType = PropertyType(1033i32);
            pub const Char16Array: PropertyType = PropertyType(1034i32);
            pub const BooleanArray: PropertyType = PropertyType(1035i32);
            pub const StringArray: PropertyType = PropertyType(1036i32);
            pub const InspectableArray: PropertyType = PropertyType(1037i32);
            pub const DateTimeArray: PropertyType = PropertyType(1038i32);
            pub const TimeSpanArray: PropertyType = PropertyType(1039i32);
            pub const GuidArray: PropertyType = PropertyType(1040i32);
            pub const PointArray: PropertyType = PropertyType(1041i32);
            pub const SizeArray: PropertyType = PropertyType(1042i32);
            pub const RectArray: PropertyType = PropertyType(1043i32);
            pub const OtherTypeArray: PropertyType = PropertyType(1044i32);
        }
        impl ::core::convert::From<i32> for PropertyType {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }
        unsafe impl ::windows::runtime::Abi for PropertyType {
            type Abi = Self;
        }
        unsafe impl ::windows::runtime::RuntimeType for PropertyType {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
        }
        impl ::windows::runtime::DefaultType for PropertyType {
            type DefaultType = Self;
        }
        pub struct PropertyValue {}
        impl PropertyValue {
            pub fn CreateEmpty() -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt8(value: u8) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInt16(value: i16) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt16(value: u16) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInt32(value: i32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt32(value: u32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInt64(value: i64) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt64(value: u64) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateSingle(value: f32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateDouble(value: f64) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateChar16(value: u16) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateBoolean(value: bool) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInspectable<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateGuid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, DateTime>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, TimeSpan>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreatePoint<'a, Param0: ::windows::runtime::IntoParam<'a, Point>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateSize<'a, Param0: ::windows::runtime::IntoParam<'a, Size>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateRect<'a, Param0: ::windows::runtime::IntoParam<'a, Rect>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt8Array(value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInt16Array(value: &[<i16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt16Array(value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInt32Array(value: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt32Array(value: &[<u32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInt64Array(value: &[<i64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateUInt64Array(value: &[<u64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateSingleArray(value: &[<f32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateDoubleArray(value: &[<f64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateChar16Array(value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateBooleanArray(value: &[<bool as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateStringArray(value: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateInspectableArray(value: &[<::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateGuidArray(value: &[<::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateDateTimeArray(value: &[<DateTime as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateTimeSpanArray(value: &[<TimeSpan as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreatePointArray(value: &[<Point as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateSizeArray(value: &[<Size as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn CreateRectArray(value: &[<Rect as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
                    (::windows::runtime::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
                })
            }
            pub fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
                static mut SHARED: ::windows::runtime::FactoryCache<PropertyValue, IPropertyValueStatics> = ::windows::runtime::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
        }
        impl ::windows::runtime::RuntimeName for PropertyValue {
            const NAME: &'static str = "Windows.Foundation.PropertyValue";
        }
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct Rect {
            pub X: f32,
            pub Y: f32,
            pub Width: f32,
            pub Height: f32,
        }
        impl Rect {}
        impl ::core::default::Default for Rect {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::fmt::Debug for Rect {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                fmt.debug_struct("Rect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
            }
        }
        impl ::core::cmp::PartialEq for Rect {
            fn eq(&self, other: &Self) -> bool {
                self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
            }
        }
        impl ::core::cmp::Eq for Rect {}
        unsafe impl ::windows::runtime::Abi for Rect {
            type Abi = Self;
        }
        unsafe impl ::windows::runtime::RuntimeType for Rect {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
        }
        impl ::windows::runtime::DefaultType for Rect {
            type DefaultType = Self;
        }
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct Size {
            pub Width: f32,
            pub Height: f32,
        }
        impl Size {}
        impl ::core::default::Default for Size {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::fmt::Debug for Size {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                fmt.debug_struct("Size").field("Width", &self.Width).field("Height", &self.Height).finish()
            }
        }
        impl ::core::cmp::PartialEq for Size {
            fn eq(&self, other: &Self) -> bool {
                self.Width == other.Width && self.Height == other.Height
            }
        }
        impl ::core::cmp::Eq for Size {}
        unsafe impl ::windows::runtime::Abi for Size {
            type Abi = Self;
        }
        unsafe impl ::windows::runtime::RuntimeType for Size {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
        }
        impl ::windows::runtime::DefaultType for Size {
            type DefaultType = Self;
        }
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct TimeSpan {
            pub Duration: i64,
        }
        impl TimeSpan {}
        impl ::core::default::Default for TimeSpan {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::fmt::Debug for TimeSpan {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                fmt.debug_struct("TimeSpan").field("Duration", &self.Duration).finish()
            }
        }
        impl ::core::cmp::PartialEq for TimeSpan {
            fn eq(&self, other: &Self) -> bool {
                self.Duration == other.Duration
            }
        }
        impl ::core::cmp::Eq for TimeSpan {}
        unsafe impl ::windows::runtime::Abi for TimeSpan {
            type Abi = Self;
        }
        unsafe impl ::windows::runtime::RuntimeType for TimeSpan {
            const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
        }
        impl ::windows::runtime::DefaultType for TimeSpan {
            type DefaultType = Self;
        }
        impl ::core::convert::From<::std::time::Duration> for TimeSpan {
            fn from(value: ::std::time::Duration) -> Self {
                Self { Duration: (value.as_nanos() / 100) as i64 }
            }
        }
        impl ::core::convert::From<TimeSpan> for ::std::time::Duration {
            fn from(value: TimeSpan) -> Self {
                ::std::time::Duration::from_nanos((value.Duration * 100) as u64)
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, TimeSpan> for ::std::time::Duration {
            fn into_param(self) -> ::windows::runtime::Param<'a, TimeSpan> {
                ::windows::runtime::Param::Owned(self.into())
            }
        }
    }
    pub mod Win32 {
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(:: std :: default :: Default, :: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: core :: fmt :: Debug)]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::runtime::Abi for BOOL {
                type Abi = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::runtime::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::runtime::Error::from_win32())
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
            impl core::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::runtime::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::runtime::Param<'a, BOOL> {
                    ::windows::runtime::Param::Owned(self.into())
                }
            }
            #[repr(transparent)]
            #[derive(:: std :: cmp :: Eq)]
            pub struct BSTR(pub *mut u16);
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
                    if value.len() == 0 {
                        return Self(::core::ptr::null_mut());
                    }
                    unsafe { SysAllocStringLen(PWSTR(value.as_ptr() as *mut _), value.len() as u32) }
                }
                pub fn as_wide(&self) -> &[u16] {
                    if self.0.is_null() {
                        return &[];
                    }
                    unsafe { ::core::slice::from_raw_parts(self.0 as *const u16, self.len()) }
                }
            }
            impl ::core::clone::Clone for BSTR {
                fn clone(&self) -> Self {
                    Self::from_wide(self.as_wide())
                }
            }
            impl ::core::convert::From<&str> for BSTR {
                fn from(value: &str) -> Self {
                    let value: ::std::vec::Vec<u16> = value.encode_utf16().collect();
                    Self::from_wide(&value)
                }
            }
            impl ::core::convert::From<::std::string::String> for BSTR {
                fn from(value: ::std::string::String) -> Self {
                    value.as_str().into()
                }
            }
            impl ::core::convert::From<&::std::string::String> for BSTR {
                fn from(value: &::std::string::String) -> Self {
                    value.as_str().into()
                }
            }
            impl<'a> ::core::convert::TryFrom<&'a BSTR> for ::std::string::String {
                type Error = ::std::string::FromUtf16Error;
                fn try_from(value: &BSTR) -> ::core::result::Result<Self, Self::Error> {
                    ::std::string::String::from_utf16(value.as_wide())
                }
            }
            impl ::core::convert::TryFrom<BSTR> for ::std::string::String {
                type Error = ::std::string::FromUtf16Error;
                fn try_from(value: BSTR) -> ::core::result::Result<Self, Self::Error> {
                    ::std::string::String::try_from(&value)
                }
            }
            impl ::core::default::Default for BSTR {
                fn default() -> Self {
                    Self(::core::ptr::null_mut())
                }
            }
            impl ::core::fmt::Display for BSTR {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    use core::fmt::Write;
                    for c in ::std::char::decode_utf16(self.as_wide().iter().cloned()) {
                        f.write_char(c.map_err(|_| ::core::fmt::Error)?)?
                    }
                    Ok(())
                }
            }
            impl ::core::fmt::Debug for BSTR {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::std::write!(f, "{}", self)
                }
            }
            impl ::core::cmp::PartialEq for BSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.as_wide() == other.as_wide()
                }
            }
            impl ::core::cmp::PartialEq<::std::string::String> for BSTR {
                fn eq(&self, other: &::std::string::String) -> bool {
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
            unsafe impl ::windows::runtime::Abi for BSTR {
                type Abi = ::core::mem::ManuallyDrop<Self>;
            }
            pub type BSTR_abi = *mut u16;
            impl<'a> ::windows::runtime::IntoParam<'a, BSTR> for &str {
                fn into_param(self) -> ::windows::runtime::Param<'a, BSTR> {
                    ::windows::runtime::Param::Owned(self.into())
                }
            }
            impl<'a> ::windows::runtime::IntoParam<'a, BSTR> for String {
                fn into_param(self) -> ::windows::runtime::Param<'a, BSTR> {
                    ::windows::runtime::Param::Owned(self.into())
                }
            }
            #[inline]
            pub unsafe fn CloseHandle<'a, Param0: ::windows::runtime::IntoParam<'a, HANDLE>>(hobject: Param0) -> BOOL {
                #[cfg(windows)]
                {
                    #[link(name = "windows")]
                    extern "system" {
                        fn CloseHandle(hobject: HANDLE) -> BOOL;
                    }
                    ::core::mem::transmute(CloseHandle(hobject.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            pub const E_NOINTERFACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147467262i32 as _);
            pub type FARPROC = unsafe extern "system" fn() -> isize;
            #[inline]
            pub unsafe fn GetLastError() -> WIN32_ERROR {
                #[cfg(windows)]
                {
                    #[link(name = "windows")]
                    extern "system" {
                        fn GetLastError() -> WIN32_ERROR;
                    }
                    ::core::mem::transmute(GetLastError())
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: default :: Default, :: core :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
            #[repr(transparent)]
            pub struct HANDLE(pub isize);
            unsafe impl ::windows::runtime::Handle for HANDLE {
                fn is_invalid(&self) -> bool {
                    self.0 == 0 || self.0 == -1
                }
                fn ok(self) -> ::windows::runtime::Result<Self> {
                    if self.is_invalid() {
                        Err(::windows::runtime::Error::from_win32())
                    } else {
                        Ok(self)
                    }
                }
            }
            unsafe impl ::windows::runtime::Abi for HANDLE {
                type Abi = Self;
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: core :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
            #[repr(transparent)]
            pub struct HINSTANCE(pub isize);
            impl ::core::default::Default for HINSTANCE {
                fn default() -> Self {
                    unsafe { ::core::mem::zeroed() }
                }
            }
            unsafe impl ::windows::runtime::Handle for HINSTANCE {}
            unsafe impl ::windows::runtime::Abi for HINSTANCE {
                type Abi = Self;
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: core :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
            #[repr(transparent)]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::core::default::Default for PSTR {
                fn default() -> Self {
                    Self(::core::ptr::null_mut())
                }
            }
            unsafe impl ::windows::runtime::Abi for PSTR {
                type Abi = Self;
                unsafe fn drop_param(param: &mut ::windows::runtime::Param<'_, Self>) {
                    if let ::windows::runtime::Param::Boxed(value) = param {
                        if !value.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::runtime::IntoParam<'a, PSTR> for &str {
                fn into_param(self) -> ::windows::runtime::Param<'a, PSTR> {
                    ::windows::runtime::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::std::iter::once(0)).collect::<std::vec::Vec<u8>>().into_boxed_slice()) as _))
                }
            }
            impl<'a> ::windows::runtime::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::runtime::Param<'a, PSTR> {
                    ::windows::runtime::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::std::iter::once(0)).collect::<std::vec::Vec<u8>>().into_boxed_slice()) as _))
                }
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: core :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
            #[repr(transparent)]
            pub struct PWSTR(pub *mut u16);
            impl PWSTR {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::core::default::Default for PWSTR {
                fn default() -> Self {
                    Self(::core::ptr::null_mut())
                }
            }
            unsafe impl ::windows::runtime::Abi for PWSTR {
                type Abi = Self;
                unsafe fn drop_param(param: &mut ::windows::runtime::Param<'_, Self>) {
                    if let ::windows::runtime::Param::Boxed(value) = param {
                        if !value.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for &str {
                fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                    ::windows::runtime::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
                }
            }
            impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for String {
                fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                    ::windows::runtime::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
                fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::runtime::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
                fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::runtime::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
                }
            }
            pub const S_OK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(0i32 as _);
            #[inline]
            pub unsafe fn SysAllocStringLen<'a, Param0: ::windows::runtime::IntoParam<'a, PWSTR>>(strin: Param0, ui: u32) -> BSTR {
                #[cfg(windows)]
                {
                    #[link(name = "windows")]
                    extern "system" {
                        fn SysAllocStringLen(strin: PWSTR, ui: u32) -> BSTR;
                    }
                    ::core::mem::transmute(SysAllocStringLen(strin.into_param().abi(), ::core::mem::transmute(ui)))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            #[inline]
            pub unsafe fn SysFreeString<'a, Param0: ::windows::runtime::IntoParam<'a, BSTR>>(bstrstring: Param0) {
                #[cfg(windows)]
                {
                    #[link(name = "windows")]
                    extern "system" {
                        fn SysFreeString(bstrstring: ::core::mem::ManuallyDrop<BSTR>);
                    }
                    ::core::mem::transmute(SysFreeString(bstrstring.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            #[inline]
            pub unsafe fn SysStringLen<'a, Param0: ::windows::runtime::IntoParam<'a, BSTR>>(pbstr: Param0) -> u32 {
                #[cfg(windows)]
                {
                    #[link(name = "windows")]
                    extern "system" {
                        fn SysStringLen(pbstr: ::core::mem::ManuallyDrop<BSTR>) -> u32;
                    }
                    ::core::mem::transmute(SysStringLen(pbstr.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: core :: fmt :: Debug)]
            #[repr(transparent)]
            pub struct WIN32_ERROR(pub u32);
            impl ::core::convert::From<u32> for WIN32_ERROR {
                fn from(value: u32) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::runtime::Abi for WIN32_ERROR {
                type Abi = Self;
            }
            impl ::core::ops::BitOr for WIN32_ERROR {
                type Output = Self;
                fn bitor(self, rhs: Self) -> Self {
                    Self(self.0 | rhs.0)
                }
            }
            impl ::core::ops::BitAnd for WIN32_ERROR {
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self {
                    Self(self.0 & rhs.0)
                }
            }
            impl ::core::ops::BitOrAssign for WIN32_ERROR {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0.bitor_assign(rhs.0)
                }
            }
            impl ::core::ops::BitAndAssign for WIN32_ERROR {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0.bitand_assign(rhs.0)
                }
            }
            impl ::core::ops::Not for WIN32_ERROR {
                type Output = Self;
                fn not(self) -> Self {
                    Self(self.0.not())
                }
            }
            impl ::core::convert::From<WIN32_ERROR> for ::windows::runtime::HRESULT {
                fn from(value: WIN32_ERROR) -> Self {
                    Self(if value.0 as i32 <= 0 { value.0 } else { (value.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000 })
                }
            }
        }
        pub mod Graphics {
            pub mod DirectDraw {
                pub const CO_E_NOTINITIALIZED: i32 = -2147221008i32;
            }
        }
        pub mod Security {
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct SECURITY_ATTRIBUTES {
                pub nLength: u32,
                pub lpSecurityDescriptor: *mut ::std::ffi::c_void,
                pub bInheritHandle: super::Foundation::BOOL,
            }
            impl SECURITY_ATTRIBUTES {}
            impl ::core::default::Default for SECURITY_ATTRIBUTES {
                fn default() -> Self {
                    unsafe { ::core::mem::zeroed() }
                }
            }
            impl ::core::fmt::Debug for SECURITY_ATTRIBUTES {
                fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    fmt.debug_struct("SECURITY_ATTRIBUTES").field("nLength", &self.nLength).field("lpSecurityDescriptor", &self.lpSecurityDescriptor).field("bInheritHandle", &self.bInheritHandle).finish()
                }
            }
            impl ::core::cmp::PartialEq for SECURITY_ATTRIBUTES {
                fn eq(&self, other: &Self) -> bool {
                    self.nLength == other.nLength && self.lpSecurityDescriptor == other.lpSecurityDescriptor && self.bInheritHandle == other.bInheritHandle
                }
            }
            impl ::core::cmp::Eq for SECURITY_ATTRIBUTES {}
            unsafe impl ::windows::runtime::Abi for SECURITY_ATTRIBUTES {
                type Abi = Self;
            }
        }
        pub mod System {
            pub mod Com {
                #[inline]
                pub unsafe fn CoCreateGuid() -> ::windows::runtime::Result<::windows::runtime::GUID> {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn CoCreateGuid(pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
                        }
                        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                        CoCreateGuid(&mut result__).from_abi::<::windows::runtime::GUID>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[inline]
                pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::std::ffi::c_void {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn CoTaskMemAlloc(cb: usize) -> *mut ::std::ffi::c_void;
                        }
                        ::core::mem::transmute(CoTaskMemAlloc(::core::mem::transmute(cb)))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[inline]
                pub unsafe fn CoTaskMemFree(pv: *const ::std::ffi::c_void) {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn CoTaskMemFree(pv: *const ::std::ffi::c_void);
                        }
                        ::core::mem::transmute(CoTaskMemFree(::core::mem::transmute(pv)))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[repr(transparent)]
                #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
                pub struct IAgileObject(pub ::windows::runtime::IUnknown);
                impl IAgileObject {}
                unsafe impl ::windows::runtime::Interface for IAgileObject {
                    type Vtable = IAgileObject_abi;
                    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2498374548, 59852, 18912, [192, 255, 238, 100, 202, 143, 91, 144]);
                }
                impl ::core::convert::From<IAgileObject> for ::windows::runtime::IUnknown {
                    fn from(value: IAgileObject) -> Self {
                        value.0
                    }
                }
                impl ::core::convert::From<&IAgileObject> for ::windows::runtime::IUnknown {
                    fn from(value: &IAgileObject) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAgileObject {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAgileObject {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Borrowed(&self.0)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IAgileObject_abi(
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                );
            }
            pub mod Diagnostics {
                pub mod Debug {
                    #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: core :: fmt :: Debug)]
                    #[repr(transparent)]
                    pub struct FORMAT_MESSAGE_OPTIONS(pub u32);
                    pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(256u32);
                    pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(8192u32);
                    pub const FORMAT_MESSAGE_FROM_HMODULE: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(2048u32);
                    pub const FORMAT_MESSAGE_FROM_STRING: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(1024u32);
                    pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(4096u32);
                    pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(512u32);
                    impl ::core::convert::From<u32> for FORMAT_MESSAGE_OPTIONS {
                        fn from(value: u32) -> Self {
                            Self(value)
                        }
                    }
                    unsafe impl ::windows::runtime::Abi for FORMAT_MESSAGE_OPTIONS {
                        type Abi = Self;
                    }
                    impl ::core::ops::BitOr for FORMAT_MESSAGE_OPTIONS {
                        type Output = Self;
                        fn bitor(self, rhs: Self) -> Self {
                            Self(self.0 | rhs.0)
                        }
                    }
                    impl ::core::ops::BitAnd for FORMAT_MESSAGE_OPTIONS {
                        type Output = Self;
                        fn bitand(self, rhs: Self) -> Self {
                            Self(self.0 & rhs.0)
                        }
                    }
                    impl ::core::ops::BitOrAssign for FORMAT_MESSAGE_OPTIONS {
                        fn bitor_assign(&mut self, rhs: Self) {
                            self.0.bitor_assign(rhs.0)
                        }
                    }
                    impl ::core::ops::BitAndAssign for FORMAT_MESSAGE_OPTIONS {
                        fn bitand_assign(&mut self, rhs: Self) {
                            self.0.bitand_assign(rhs.0)
                        }
                    }
                    impl ::core::ops::Not for FORMAT_MESSAGE_OPTIONS {
                        type Output = Self;
                        fn not(self) -> Self {
                            Self(self.0.not())
                        }
                    }
                    #[inline]
                    pub unsafe fn FormatMessageW(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::std::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: super::super::super::Foundation::PWSTR, nsize: u32, arguments: *const *const i8) -> u32 {
                        #[cfg(windows)]
                        {
                            #[link(name = "windows")]
                            extern "system" {
                                fn FormatMessageW(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::std::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: super::super::super::Foundation::PWSTR, nsize: u32, arguments: *const *const i8) -> u32;
                            }
                            ::core::mem::transmute(FormatMessageW(::core::mem::transmute(dwflags), ::core::mem::transmute(lpsource), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(dwlanguageid), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize), ::core::mem::transmute(arguments)))
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            pub mod LibraryLoader {
                #[inline]
                pub unsafe fn FreeLibrary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hlibmodule: Param0) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn FreeLibrary(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
                        }
                        ::core::mem::transmute(FreeLibrary(hlibmodule.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[inline]
                pub unsafe fn GetProcAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lpprocname: Param1) -> ::std::option::Option<super::super::Foundation::FARPROC> {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn GetProcAddress(hmodule: super::super::Foundation::HINSTANCE, lpprocname: super::super::Foundation::PSTR) -> ::std::option::Option<super::super::Foundation::FARPROC>;
                        }
                        ::core::mem::transmute(GetProcAddress(hmodule.into_param().abi(), lpprocname.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[inline]
                pub unsafe fn LoadLibraryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lplibfilename: Param0) -> super::super::Foundation::HINSTANCE {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn LoadLibraryA(lplibfilename: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
                        }
                        ::core::mem::transmute(LoadLibraryA(lplibfilename.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            pub mod Memory {
                #[inline]
                pub unsafe fn GetProcessHeap() -> HeapHandle {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn GetProcessHeap() -> HeapHandle;
                        }
                        ::core::mem::transmute(GetProcessHeap())
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: core :: fmt :: Debug)]
                #[repr(transparent)]
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
                impl ::core::convert::From<u32> for HEAP_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::runtime::Abi for HEAP_FLAGS {
                    type Abi = Self;
                }
                impl ::core::ops::BitOr for HEAP_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::core::ops::BitAnd for HEAP_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::core::ops::BitOrAssign for HEAP_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::core::ops::BitAndAssign for HEAP_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::core::ops::Not for HEAP_FLAGS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                #[inline]
                pub unsafe fn HeapAlloc<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::std::ffi::c_void {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn HeapAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::std::ffi::c_void;
                        }
                        ::core::mem::transmute(HeapAlloc(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwbytes)))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[inline]
                pub unsafe fn HeapFree<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS, lpmem: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn HeapFree(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
                        }
                        ::core::mem::transmute(HeapFree(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpmem)))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: core :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
                #[repr(transparent)]
                pub struct HeapHandle(pub isize);
                impl ::core::default::Default for HeapHandle {
                    fn default() -> Self {
                        unsafe { ::core::mem::zeroed() }
                    }
                }
                unsafe impl ::windows::runtime::Handle for HeapHandle {}
                unsafe impl ::windows::runtime::Abi for HeapHandle {
                    type Abi = Self;
                }
            }
            pub mod Ole {
                pub mod Automation {
                    #[inline]
                    pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::runtime::Result<IErrorInfo> {
                        #[cfg(windows)]
                        {
                            #[link(name = "windows")]
                            extern "system" {
                                fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
                            }
                            let mut result__: <IErrorInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                            GetErrorInfo(::core::mem::transmute(dwreserved), &mut result__).from_abi::<IErrorInfo>(result__)
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                    #[repr(transparent)]
                    #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
                    pub struct IErrorInfo(pub ::windows::runtime::IUnknown);
                    impl IErrorInfo {
                        pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
                            let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                            (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
                        }
                        pub unsafe fn GetSource(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
                            let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                            (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
                        }
                        pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
                            let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                            (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
                        }
                        pub unsafe fn GetHelpFile(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
                            let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                            (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
                        }
                        pub unsafe fn GetHelpContext(&self) -> ::windows::runtime::Result<u32> {
                            let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                            (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
                        }
                    }
                    unsafe impl ::windows::runtime::Interface for IErrorInfo {
                        type Vtable = IErrorInfo_abi;
                        const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(485667104, 21629, 4123, [142, 101, 8, 0, 43, 43, 209, 25]);
                    }
                    impl ::core::convert::From<IErrorInfo> for ::windows::runtime::IUnknown {
                        fn from(value: IErrorInfo) -> Self {
                            value.0
                        }
                    }
                    impl ::core::convert::From<&IErrorInfo> for ::windows::runtime::IUnknown {
                        fn from(value: &IErrorInfo) -> Self {
                            value.0.clone()
                        }
                    }
                    impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IErrorInfo {
                        fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                            ::windows::runtime::Param::Owned(self.0)
                        }
                    }
                    impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IErrorInfo {
                        fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                            ::windows::runtime::Param::Borrowed(&self.0)
                        }
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IErrorInfo_abi(
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsource: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrhelpfile: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
                        pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhelpcontext: *mut u32) -> ::windows::runtime::HRESULT,
                    );
                    #[inline]
                    pub unsafe fn SetErrorInfo<'a, Param1: ::windows::runtime::IntoParam<'a, IErrorInfo>>(dwreserved: u32, perrinfo: Param1) -> ::windows::runtime::Result<()> {
                        #[cfg(windows)]
                        {
                            #[link(name = "windows")]
                            extern "system" {
                                fn SetErrorInfo(dwreserved: u32, perrinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
                            }
                            SetErrorInfo(::core::mem::transmute(dwreserved), perrinfo.into_param().abi()).ok()
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
            pub mod Threading {
                #[inline]
                pub unsafe fn CreateEventA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: Param1, binitialstate: Param2, lpname: Param3) -> super::super::Foundation::HANDLE {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn CreateEventA(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
                        }
                        ::core::mem::transmute(CreateEventA(::core::mem::transmute(lpeventattributes), bmanualreset.into_param().abi(), binitialstate.into_param().abi(), lpname.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[inline]
                pub unsafe fn SetEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn SetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
                        }
                        ::core::mem::transmute(SetEvent(hevent.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[inline]
                pub unsafe fn WaitForSingleObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hhandle: Param0, dwmilliseconds: u32) -> u32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "windows")]
                        extern "system" {
                            fn WaitForSingleObject(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
                        }
                        ::core::mem::transmute(WaitForSingleObject(hhandle.into_param().abi(), ::core::mem::transmute(dwmilliseconds)))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            pub mod WinRT {
                #[repr(transparent)]
                #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
                pub struct ILanguageExceptionErrorInfo(pub ::windows::runtime::IUnknown);
                impl ILanguageExceptionErrorInfo {
                    pub unsafe fn GetLanguageException(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
                        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
                    }
                }
                unsafe impl ::windows::runtime::Interface for ILanguageExceptionErrorInfo {
                    type Vtable = ILanguageExceptionErrorInfo_abi;
                    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(77782003, 57219, 4460, [9, 70, 8, 18, 171, 246, 224, 125]);
                }
                impl ::core::convert::From<ILanguageExceptionErrorInfo> for ::windows::runtime::IUnknown {
                    fn from(value: ILanguageExceptionErrorInfo) -> Self {
                        value.0
                    }
                }
                impl ::core::convert::From<&ILanguageExceptionErrorInfo> for ::windows::runtime::IUnknown {
                    fn from(value: &ILanguageExceptionErrorInfo) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILanguageExceptionErrorInfo {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILanguageExceptionErrorInfo {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Borrowed(&self.0)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ILanguageExceptionErrorInfo_abi(
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languageexception: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                );
                #[repr(transparent)]
                #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
                pub struct ILanguageExceptionErrorInfo2(pub ::windows::runtime::IUnknown);
                impl ILanguageExceptionErrorInfo2 {
                    pub unsafe fn GetLanguageException(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
                        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
                    }
                    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> ::windows::runtime::Result<ILanguageExceptionErrorInfo2> {
                        let mut result__: <ILanguageExceptionErrorInfo2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ILanguageExceptionErrorInfo2>(result__)
                    }
                    pub unsafe fn CapturePropagationContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, languageexception: Param0) -> ::windows::runtime::Result<()> {
                        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), languageexception.into_param().abi()).ok()
                    }
                    pub unsafe fn GetPropagationContextHead(&self) -> ::windows::runtime::Result<ILanguageExceptionErrorInfo2> {
                        let mut result__: <ILanguageExceptionErrorInfo2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ILanguageExceptionErrorInfo2>(result__)
                    }
                }
                unsafe impl ::windows::runtime::Interface for ILanguageExceptionErrorInfo2 {
                    type Vtable = ILanguageExceptionErrorInfo2_abi;
                    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1464264132, 23447, 16972, [182, 32, 40, 34, 145, 87, 52, 221]);
                }
                impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ::windows::runtime::IUnknown {
                    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
                        value.0
                    }
                }
                impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ::windows::runtime::IUnknown {
                    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Borrowed(&self.0)
                    }
                }
                impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
                    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
                        unsafe { ::core::mem::transmute(value) }
                    }
                }
                impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
                    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
                        ::core::convert::From::from(::core::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ILanguageExceptionErrorInfo> for ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ILanguageExceptionErrorInfo> {
                        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ILanguageExceptionErrorInfo> for &ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ILanguageExceptionErrorInfo> {
                        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ILanguageExceptionErrorInfo2_abi(
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languageexception: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previouslanguageexceptionerrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languageexception: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propagatedlanguageexceptionerrorinfohead: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                );
                #[repr(transparent)]
                #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
                pub struct IRestrictedErrorInfo(pub ::windows::runtime::IUnknown);
                impl IRestrictedErrorInfo {
                    pub unsafe fn GetErrorDetails(&self, description: *mut super::super::Foundation::BSTR, error: *mut ::windows::runtime::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
                        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(description), ::core::mem::transmute(error), ::core::mem::transmute(restricteddescription), ::core::mem::transmute(capabilitysid)).ok()
                    }
                    pub unsafe fn GetReference(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
                        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
                    }
                }
                unsafe impl ::windows::runtime::Interface for IRestrictedErrorInfo {
                    type Vtable = IRestrictedErrorInfo_abi;
                    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2193256594, 19592, 17021, [167, 188, 22, 221, 147, 254, 182, 126]);
                }
                impl ::core::convert::From<IRestrictedErrorInfo> for ::windows::runtime::IUnknown {
                    fn from(value: IRestrictedErrorInfo) -> Self {
                        value.0
                    }
                }
                impl ::core::convert::From<&IRestrictedErrorInfo> for ::windows::runtime::IUnknown {
                    fn from(value: &IRestrictedErrorInfo) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRestrictedErrorInfo {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRestrictedErrorInfo {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Borrowed(&self.0)
                    }
                }
                unsafe impl ::core::marker::Send for IRestrictedErrorInfo {}
                unsafe impl ::core::marker::Sync for IRestrictedErrorInfo {}
                #[repr(C)]
                #[doc(hidden)]
                pub struct IRestrictedErrorInfo_abi(
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: *mut ::windows::runtime::HRESULT, restricteddescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, capabilitysid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reference: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
                );
                #[repr(transparent)]
                #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
                pub struct IWeakReference(pub ::windows::runtime::IUnknown);
                impl IWeakReference {
                    pub unsafe fn Resolve<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
                        let mut result__ = ::std::option::Option::None;
                        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                    }
                }
                unsafe impl ::windows::runtime::Interface for IWeakReference {
                    type Vtable = IWeakReference_abi;
                    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(55, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
                }
                impl ::core::convert::From<IWeakReference> for ::windows::runtime::IUnknown {
                    fn from(value: IWeakReference) -> Self {
                        value.0
                    }
                }
                impl ::core::convert::From<&IWeakReference> for ::windows::runtime::IUnknown {
                    fn from(value: &IWeakReference) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWeakReference {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWeakReference {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Borrowed(&self.0)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWeakReference_abi(
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, objectreference: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
                );
                #[repr(transparent)]
                #[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: core :: fmt :: Debug)]
                pub struct IWeakReferenceSource(pub ::windows::runtime::IUnknown);
                impl IWeakReferenceSource {
                    pub unsafe fn GetWeakReference(&self) -> ::windows::runtime::Result<IWeakReference> {
                        let mut result__: <IWeakReference as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
                        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWeakReference>(result__)
                    }
                }
                unsafe impl ::windows::runtime::Interface for IWeakReferenceSource {
                    type Vtable = IWeakReferenceSource_abi;
                    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(56, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
                }
                impl ::core::convert::From<IWeakReferenceSource> for ::windows::runtime::IUnknown {
                    fn from(value: IWeakReferenceSource) -> Self {
                        value.0
                    }
                }
                impl ::core::convert::From<&IWeakReferenceSource> for ::windows::runtime::IUnknown {
                    fn from(value: &IWeakReferenceSource) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWeakReferenceSource {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWeakReferenceSource {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                        ::windows::runtime::Param::Borrowed(&self.0)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWeakReferenceSource_abi(
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, weakreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
                );
            }
        }
    }
}
