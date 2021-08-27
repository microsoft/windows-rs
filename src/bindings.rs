#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Foundation {
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct DateTime {
            pub UniversalTime: i64,
        }
        impl DateTime {}
        impl ::std::default::Default for DateTime {
            fn default() -> Self {
                Self { UniversalTime: 0 }
            }
        }
        impl ::std::fmt::Debug for DateTime {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct("DateTime")
                    .field("UniversalTime", &self.UniversalTime)
                    .finish()
            }
        }
        impl ::std::cmp::PartialEq for DateTime {
            fn eq(&self, other: &Self) -> bool {
                self.UniversalTime == other.UniversalTime
            }
        }
        impl ::std::cmp::Eq for DateTime {}
        unsafe impl ::windows::Abi for DateTime {
            type Abi = Self;
            type DefaultType = Self;
        }
        unsafe impl ::windows::RuntimeType for DateTime {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IPropertyValue(::windows::IInspectable);
        unsafe impl ::windows::Interface for IPropertyValue {
            type Vtable = IPropertyValue_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1272349405,
                30036,
                16617,
                [154, 155, 130, 101, 78, 222, 126, 98],
            );
        }
        impl IPropertyValue {
            pub fn Type(&self) -> ::windows::Result<PropertyType> {
                let this = self;
                unsafe {
                    let mut result__: <PropertyType as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<PropertyType>(result__)
                }
            }
            pub fn IsNumericScalar(&self) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<bool>(result__)
                }
            }
            pub fn GetUInt8(&self) -> ::windows::Result<u8> {
                let this = self;
                unsafe {
                    let mut result__: <u8 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<u8>(result__)
                }
            }
            pub fn GetInt16(&self) -> ::windows::Result<i16> {
                let this = self;
                unsafe {
                    let mut result__: <i16 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<i16>(result__)
                }
            }
            pub fn GetUInt16(&self) -> ::windows::Result<u16> {
                let this = self;
                unsafe {
                    let mut result__: <u16 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u16>(result__)
                }
            }
            pub fn GetInt32(&self) -> ::windows::Result<i32> {
                let this = self;
                unsafe {
                    let mut result__: <i32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<i32>(result__)
                }
            }
            pub fn GetUInt32(&self) -> ::windows::Result<u32> {
                let this = self;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u32>(result__)
                }
            }
            pub fn GetInt64(&self) -> ::windows::Result<i64> {
                let this = self;
                unsafe {
                    let mut result__: <i64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).13)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<i64>(result__)
                }
            }
            pub fn GetUInt64(&self) -> ::windows::Result<u64> {
                let this = self;
                unsafe {
                    let mut result__: <u64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).14)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u64>(result__)
                }
            }
            pub fn GetSingle(&self) -> ::windows::Result<f32> {
                let this = self;
                unsafe {
                    let mut result__: <f32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).15)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<f32>(result__)
                }
            }
            pub fn GetDouble(&self) -> ::windows::Result<f64> {
                let this = self;
                unsafe {
                    let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).16)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<f64>(result__)
                }
            }
            pub fn GetChar16(&self) -> ::windows::Result<u16> {
                let this = self;
                unsafe {
                    let mut result__: <u16 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).17)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u16>(result__)
                }
            }
            pub fn GetBoolean(&self) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).18)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn GetString(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).19)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn GetGuid(&self) -> ::windows::Result<::windows::Guid> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::Guid as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).20)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::Guid>(result__)
                }
            }
            pub fn GetDateTime(&self) -> ::windows::Result<DateTime> {
                let this = self;
                unsafe {
                    let mut result__: <DateTime as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).21)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<DateTime>(result__)
                }
            }
            pub fn GetTimeSpan(&self) -> ::windows::Result<TimeSpan> {
                let this = self;
                unsafe {
                    let mut result__: <TimeSpan as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).22)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<TimeSpan>(result__)
                }
            }
            pub fn GetPoint(&self) -> ::windows::Result<Point> {
                let this = self;
                unsafe {
                    let mut result__: <Point as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).23)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<Point>(result__)
                }
            }
            pub fn GetSize(&self) -> ::windows::Result<Size> {
                let this = self;
                unsafe {
                    let mut result__: <Size as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).24)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<Size>(result__)
                }
            }
            pub fn GetRect(&self) -> ::windows::Result<Rect> {
                let this = self;
                unsafe {
                    let mut result__: <Rect as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).25)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<Rect>(result__)
                }
            }
            pub fn GetUInt8Array(&self, value: &mut ::windows::Array<u8>) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).26)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInt16Array(
                &self,
                value: &mut ::windows::Array<i16>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).27)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetUInt16Array(
                &self,
                value: &mut ::windows::Array<u16>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).28)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInt32Array(
                &self,
                value: &mut ::windows::Array<i32>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).29)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetUInt32Array(
                &self,
                value: &mut ::windows::Array<u32>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).30)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInt64Array(
                &self,
                value: &mut ::windows::Array<i64>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).31)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetUInt64Array(
                &self,
                value: &mut ::windows::Array<u64>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).32)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetSingleArray(
                &self,
                value: &mut ::windows::Array<f32>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).33)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetDoubleArray(
                &self,
                value: &mut ::windows::Array<f64>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).34)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetChar16Array(
                &self,
                value: &mut ::windows::Array<u16>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).35)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetBooleanArray(
                &self,
                value: &mut ::windows::Array<bool>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).36)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetStringArray(
                &self,
                value: &mut ::windows::Array<::windows::HSTRING>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).37)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInspectableArray(
                &self,
                value: &mut ::windows::Array<::windows::IInspectable>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).38)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetGuidArray(
                &self,
                value: &mut ::windows::Array<::windows::Guid>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).39)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetDateTimeArray(
                &self,
                value: &mut ::windows::Array<DateTime>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).40)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetTimeSpanArray(
                &self,
                value: &mut ::windows::Array<TimeSpan>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).41)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetPointArray(
                &self,
                value: &mut ::windows::Array<Point>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).42)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetSizeArray(
                &self,
                value: &mut ::windows::Array<Size>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).43)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetRectArray(
                &self,
                value: &mut ::windows::Array<Rect>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).44)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IPropertyValue {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{4bd682dd-7554-40e9-9a9b-82654ede7e62}");
        }
        impl ::std::convert::From<IPropertyValue> for ::windows::IUnknown {
            fn from(value: IPropertyValue) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&IPropertyValue> for ::windows::IUnknown {
            fn from(value: &IPropertyValue) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IPropertyValue {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IPropertyValue {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<IPropertyValue> for ::windows::IInspectable {
            fn from(value: IPropertyValue) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IPropertyValue> for ::windows::IInspectable {
            fn from(value: &IPropertyValue) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IPropertyValue {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IPropertyValue {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IPropertyValue_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut PropertyType,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u8,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut i16,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u16,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut i64,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u64,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut f32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut f64,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u16,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut DateTime,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut TimeSpan,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut Point,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut Size,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut Rect,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut u8,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut i16,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut u16,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut u32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut i64,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut u64,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut f32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut f64,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut u16,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut bool,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut DateTime,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut TimeSpan,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut Point,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut Size,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: *mut u32,
                value: *mut *mut Rect,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IPropertyValueStatics(::windows::IInspectable);
        unsafe impl ::windows::Interface for IPropertyValueStatics {
            type Vtable = IPropertyValueStatics_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1654381512,
                55602,
                20468,
                [150, 185, 141, 150, 197, 193, 232, 88],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IPropertyValueStatics_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: u8,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: i16,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: u16,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: i32,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: u32,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: i64,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: u64,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: f32,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: f64,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: u16,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: bool,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: ::windows::Guid,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: DateTime,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: TimeSpan,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: Point,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: Size,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: Rect,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const u8,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const i16,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const u16,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const i32,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const u32,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const i64,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const u64,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const f32,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const f64,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const u16,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const bool,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const ::windows::Guid,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const DateTime,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const TimeSpan,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const Point,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const Size,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value_array_size: u32,
                value: *const Rect,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IReference<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
        where
            T: ::windows::RuntimeType + 'static;
        unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IReference<T> {
            type Vtable = IReference_abi<T>;
            const IID: ::windows::Guid = ::windows::Guid::from_signature(
                <IReference<T> as ::windows::RuntimeType>::SIGNATURE,
            );
        }
        impl<T: ::windows::RuntimeType + 'static> IReference<T> {
            pub fn Value(&self) -> ::windows::Result<T> {
                let this = self;
                unsafe {
                    let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<T>(result__)
                }
            }
            pub fn Type(&self) -> ::windows::Result<PropertyType> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <PropertyType as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<PropertyType>(result__)
                }
            }
            pub fn IsNumericScalar(&self) -> ::windows::Result<bool> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<bool>(result__)
                }
            }
            pub fn GetUInt8(&self) -> ::windows::Result<u8> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <u8 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<u8>(result__)
                }
            }
            pub fn GetInt16(&self) -> ::windows::Result<i16> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <i16 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<i16>(result__)
                }
            }
            pub fn GetUInt16(&self) -> ::windows::Result<u16> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <u16 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u16>(result__)
                }
            }
            pub fn GetInt32(&self) -> ::windows::Result<i32> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <i32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<i32>(result__)
                }
            }
            pub fn GetUInt32(&self) -> ::windows::Result<u32> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u32>(result__)
                }
            }
            pub fn GetInt64(&self) -> ::windows::Result<i64> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <i64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).13)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<i64>(result__)
                }
            }
            pub fn GetUInt64(&self) -> ::windows::Result<u64> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <u64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).14)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u64>(result__)
                }
            }
            pub fn GetSingle(&self) -> ::windows::Result<f32> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <f32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).15)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<f32>(result__)
                }
            }
            pub fn GetDouble(&self) -> ::windows::Result<f64> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).16)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<f64>(result__)
                }
            }
            pub fn GetChar16(&self) -> ::windows::Result<u16> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <u16 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).17)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<u16>(result__)
                }
            }
            pub fn GetBoolean(&self) -> ::windows::Result<bool> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).18)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn GetString(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).19)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn GetGuid(&self) -> ::windows::Result<::windows::Guid> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::Guid as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).20)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::Guid>(result__)
                }
            }
            pub fn GetDateTime(&self) -> ::windows::Result<DateTime> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <DateTime as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).21)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<DateTime>(result__)
                }
            }
            pub fn GetTimeSpan(&self) -> ::windows::Result<TimeSpan> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <TimeSpan as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).22)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<TimeSpan>(result__)
                }
            }
            pub fn GetPoint(&self) -> ::windows::Result<Point> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <Point as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).23)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<Point>(result__)
                }
            }
            pub fn GetSize(&self) -> ::windows::Result<Size> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <Size as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).24)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<Size>(result__)
                }
            }
            pub fn GetRect(&self) -> ::windows::Result<Rect> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    let mut result__: <Rect as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).25)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<Rect>(result__)
                }
            }
            pub fn GetUInt8Array(&self, value: &mut ::windows::Array<u8>) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).26)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInt16Array(
                &self,
                value: &mut ::windows::Array<i16>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).27)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetUInt16Array(
                &self,
                value: &mut ::windows::Array<u16>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).28)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInt32Array(
                &self,
                value: &mut ::windows::Array<i32>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).29)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetUInt32Array(
                &self,
                value: &mut ::windows::Array<u32>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).30)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInt64Array(
                &self,
                value: &mut ::windows::Array<i64>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).31)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetUInt64Array(
                &self,
                value: &mut ::windows::Array<u64>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).32)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetSingleArray(
                &self,
                value: &mut ::windows::Array<f32>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).33)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetDoubleArray(
                &self,
                value: &mut ::windows::Array<f64>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).34)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetChar16Array(
                &self,
                value: &mut ::windows::Array<u16>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).35)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetBooleanArray(
                &self,
                value: &mut ::windows::Array<bool>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).36)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetStringArray(
                &self,
                value: &mut ::windows::Array<::windows::HSTRING>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).37)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetInspectableArray(
                &self,
                value: &mut ::windows::Array<::windows::IInspectable>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).38)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetGuidArray(
                &self,
                value: &mut ::windows::Array<::windows::Guid>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).39)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetDateTimeArray(
                &self,
                value: &mut ::windows::Array<DateTime>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).40)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetTimeSpanArray(
                &self,
                value: &mut ::windows::Array<TimeSpan>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).41)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetPointArray(
                &self,
                value: &mut ::windows::Array<Point>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).42)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetSizeArray(
                &self,
                value: &mut ::windows::Array<Size>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).43)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
            pub fn GetRectArray(
                &self,
                value: &mut ::windows::Array<Rect>,
            ) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IPropertyValue>(self).unwrap();
                unsafe {
                    (::windows::Interface::vtable(this).44)(
                        ::windows::Abi::abi(this),
                        value.set_abi_len(),
                        value.set_abi(),
                    )
                    .ok()
                }
            }
        }
        unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IReference<T> {
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}")
                    .push_slice(b";")
                    .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IReference<T>>
            for ::windows::IUnknown
        {
            fn from(value: IReference<T>) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IReference<T>>
            for ::windows::IUnknown
        {
            fn from(value: &IReference<T>) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, ::windows::IUnknown>
            for IReference<T>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, ::windows::IUnknown>
            for &IReference<T>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IReference<T>>
            for ::windows::IInspectable
        {
            fn from(value: IReference<T>) -> Self {
                value.0
            }
        }
        impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IReference<T>>
            for ::windows::IInspectable
        {
            fn from(value: &IReference<T>) -> Self {
                value.0.clone()
            }
        }
        impl<'a, T: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::IInspectable> for IReference<T>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a, T: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IReference<T>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IReference<T>> for IPropertyValue {
            fn from(value: IReference<T>) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IReference<T>> for IPropertyValue {
            fn from(value: &IReference<T>) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IPropertyValue>
            for IReference<T>
        {
            fn into_param(self) -> ::windows::Param<'a, IPropertyValue> {
                ::windows::Param::Owned(::std::convert::Into::<IPropertyValue>::into(self))
            }
        }
        impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IPropertyValue>
            for &IReference<T>
        {
            fn into_param(self) -> ::windows::Param<'a, IPropertyValue> {
                ::windows::Param::Owned(::std::convert::Into::<IPropertyValue>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IReference_abi<T>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut <T as ::windows::Abi>::Abi,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<T>,
        )
        where
            T: ::windows::RuntimeType + 'static;
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStringable(::windows::IInspectable);
        unsafe impl ::windows::Interface for IStringable {
            type Vtable = IStringable_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2520162132,
                36534,
                18672,
                [171, 206, 193, 178, 17, 230, 39, 195],
            );
        }
        impl IStringable {
            pub fn ToString(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStringable {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
        }
        impl ::std::convert::From<IStringable> for ::windows::IUnknown {
            fn from(value: IStringable) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&IStringable> for ::windows::IUnknown {
            fn from(value: &IStringable) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<IStringable> for ::windows::IInspectable {
            fn from(value: IStringable) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStringable> for ::windows::IInspectable {
            fn from(value: &IStringable) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStringable_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct Point {
            pub X: f32,
            pub Y: f32,
        }
        impl Point {}
        impl ::std::default::Default for Point {
            fn default() -> Self {
                Self { X: 0.0, Y: 0.0 }
            }
        }
        impl ::std::fmt::Debug for Point {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct("Point")
                    .field("X", &self.X)
                    .field("Y", &self.Y)
                    .finish()
            }
        }
        impl ::std::cmp::PartialEq for Point {
            fn eq(&self, other: &Self) -> bool {
                self.X == other.X && self.Y == other.Y
            }
        }
        impl ::std::cmp::Eq for Point {}
        unsafe impl ::windows::Abi for Point {
            type Abi = Self;
            type DefaultType = Self;
        }
        unsafe impl ::windows::RuntimeType for Point {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
        }
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: marker :: Copy,
            :: std :: clone :: Clone,
            :: std :: default :: Default,
            :: std :: fmt :: Debug,
        )]
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
        impl ::std::convert::From<i32> for PropertyType {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }
        unsafe impl ::windows::Abi for PropertyType {
            type Abi = Self;
            type DefaultType = Self;
        }
        unsafe impl ::windows::RuntimeType for PropertyType {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
        }
        pub struct PropertyValue {}
        impl PropertyValue {
            pub fn CreateEmpty() -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt8(value: u8) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInt16(value: i16) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt16(value: u16) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInt32(value: i32) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt32(value: u32) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInt64(value: i64) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt64(value: u64) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).13)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateSingle(value: f32) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).14)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateDouble(value: f64) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).15)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateChar16(value: u16) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).16)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateBoolean(value: bool) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).17)(
                        ::windows::Abi::abi(this),
                        value,
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateString<'a>(
                value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).18)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInspectable<'a>(
                value: impl ::windows::IntoParam<'a, ::windows::IInspectable>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).19)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateGuid<'a>(
                value: impl ::windows::IntoParam<'a, ::windows::Guid>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).20)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateDateTime<'a>(
                value: impl ::windows::IntoParam<'a, DateTime>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).21)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateTimeSpan<'a>(
                value: impl ::windows::IntoParam<'a, TimeSpan>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).22)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreatePoint<'a>(
                value: impl ::windows::IntoParam<'a, Point>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).23)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateSize<'a>(
                value: impl ::windows::IntoParam<'a, Size>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).24)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateRect<'a>(
                value: impl ::windows::IntoParam<'a, Rect>,
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).25)(
                        ::windows::Abi::abi(this),
                        value.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt8Array(
                value: &[<u8 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).26)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInt16Array(
                value: &[<i16 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).27)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt16Array(
                value: &[<u16 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).28)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInt32Array(
                value: &[<i32 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).29)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt32Array(
                value: &[<u32 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).30)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInt64Array(
                value: &[<i64 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).31)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateUInt64Array(
                value: &[<u64 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).32)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateSingleArray(
                value: &[<f32 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).33)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateDoubleArray(
                value: &[<f64 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).34)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateChar16Array(
                value: &[<u16 as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).35)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateBooleanArray(
                value: &[<bool as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).36)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateStringArray(
                value: &[<::windows::HSTRING as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).37)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateInspectableArray(
                value: &[<::windows::IInspectable as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).38)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateGuidArray(
                value: &[<::windows::Guid as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).39)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateDateTimeArray(
                value: &[<DateTime as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).40)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateTimeSpanArray(
                value: &[<TimeSpan as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).41)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreatePointArray(
                value: &[<Point as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).42)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateSizeArray(
                value: &[<Size as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).43)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn CreateRectArray(
                value: &[<Rect as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<::windows::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).44)(
                        ::windows::Abi::abi(this),
                        value.len() as u32,
                        ::std::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .from_abi::<::windows::IInspectable>(result__)
                })
            }
            pub fn IPropertyValueStatics<
                R,
                F: FnOnce(&IPropertyValueStatics) -> ::windows::Result<R>,
            >(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<PropertyValue, IPropertyValueStatics> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
        }
        impl ::windows::RuntimeName for PropertyValue {
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
        impl ::std::default::Default for Rect {
            fn default() -> Self {
                Self {
                    X: 0.0,
                    Y: 0.0,
                    Width: 0.0,
                    Height: 0.0,
                }
            }
        }
        impl ::std::fmt::Debug for Rect {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct("Rect")
                    .field("X", &self.X)
                    .field("Y", &self.Y)
                    .field("Width", &self.Width)
                    .field("Height", &self.Height)
                    .finish()
            }
        }
        impl ::std::cmp::PartialEq for Rect {
            fn eq(&self, other: &Self) -> bool {
                self.X == other.X
                    && self.Y == other.Y
                    && self.Width == other.Width
                    && self.Height == other.Height
            }
        }
        impl ::std::cmp::Eq for Rect {}
        unsafe impl ::windows::Abi for Rect {
            type Abi = Self;
            type DefaultType = Self;
        }
        unsafe impl ::windows::RuntimeType for Rect {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
        }
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct Size {
            pub Width: f32,
            pub Height: f32,
        }
        impl Size {}
        impl ::std::default::Default for Size {
            fn default() -> Self {
                Self {
                    Width: 0.0,
                    Height: 0.0,
                }
            }
        }
        impl ::std::fmt::Debug for Size {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct("Size")
                    .field("Width", &self.Width)
                    .field("Height", &self.Height)
                    .finish()
            }
        }
        impl ::std::cmp::PartialEq for Size {
            fn eq(&self, other: &Self) -> bool {
                self.Width == other.Width && self.Height == other.Height
            }
        }
        impl ::std::cmp::Eq for Size {}
        unsafe impl ::windows::Abi for Size {
            type Abi = Self;
            type DefaultType = Self;
        }
        unsafe impl ::windows::RuntimeType for Size {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
        }
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        #[repr(C)]
        pub struct TimeSpan {
            pub Duration: i64,
        }
        impl TimeSpan {}
        impl ::std::default::Default for TimeSpan {
            fn default() -> Self {
                Self { Duration: 0 }
            }
        }
        impl ::std::fmt::Debug for TimeSpan {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct("TimeSpan")
                    .field("Duration", &self.Duration)
                    .finish()
            }
        }
        impl ::std::cmp::PartialEq for TimeSpan {
            fn eq(&self, other: &Self) -> bool {
                self.Duration == other.Duration
            }
        }
        impl ::std::cmp::Eq for TimeSpan {}
        unsafe impl ::windows::Abi for TimeSpan {
            type Abi = Self;
            type DefaultType = Self;
        }
        unsafe impl ::windows::RuntimeType for TimeSpan {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
        }
        impl ::std::convert::From<::std::time::Duration> for TimeSpan {
            fn from(value: ::std::time::Duration) -> Self {
                Self {
                    Duration: (value.as_nanos() / 100) as i64,
                }
            }
        }
        impl ::std::convert::From<TimeSpan> for ::std::time::Duration {
            fn from(value: TimeSpan) -> Self {
                ::std::time::Duration::from_nanos((value.Duration * 100) as u64)
            }
        }
        impl<'a> ::windows::IntoParam<'a, TimeSpan> for ::std::time::Duration {
            fn into_param(self) -> ::windows::Param<'a, TimeSpan> {
                ::windows::Param::Owned(self.into())
            }
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
                type DefaultType = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::HRESULT::from_thread().into())
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
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
            #[repr(transparent)]
            #[derive(:: std :: cmp :: Eq)]
            pub struct BSTR(pub *mut u16);
            impl BSTR {
                #[doc = r" Create an empty `BSTR`."]
                #[doc = r""]
                #[doc = r" This function does not allocate memory."]
                pub fn new() -> Self {
                    Self(std::ptr::null_mut())
                }
                #[doc = r" Returns `true` if the string is empty."]
                pub fn is_empty(&self) -> bool {
                    self.0.is_null()
                }
                #[doc = r" Returns the length of the string."]
                pub fn len(&self) -> usize {
                    if self.is_empty() {
                        return 0;
                    }
                    unsafe { SysStringLen(self) as usize }
                }
                #[doc = r" Create a `BSTR` from a slice of 16-bit characters."]
                pub fn from_wide(value: &[u16]) -> Self {
                    if value.len() == 0 {
                        return Self(::std::ptr::null_mut());
                    }
                    unsafe { SysAllocStringLen(PWSTR(value.as_ptr() as _), value.len() as u32) }
                }
                #[doc = r" Get the string as 16-bit characters."]
                pub fn as_wide(&self) -> &[u16] {
                    if self.0.is_null() {
                        return &[];
                    }
                    unsafe { ::std::slice::from_raw_parts(self.0 as *const u16, self.len()) }
                }
            }
            impl ::std::clone::Clone for BSTR {
                fn clone(&self) -> Self {
                    Self::from_wide(self.as_wide())
                }
            }
            impl ::std::convert::From<&str> for BSTR {
                fn from(value: &str) -> Self {
                    let value: ::std::vec::Vec<u16> = value.encode_utf16().collect();
                    Self::from_wide(&value)
                }
            }
            impl ::std::convert::From<::std::string::String> for BSTR {
                fn from(value: ::std::string::String) -> Self {
                    value.as_str().into()
                }
            }
            impl ::std::convert::From<&::std::string::String> for BSTR {
                fn from(value: &::std::string::String) -> Self {
                    value.as_str().into()
                }
            }
            impl<'a> ::std::convert::TryFrom<&'a BSTR> for ::std::string::String {
                type Error = ::std::string::FromUtf16Error;
                fn try_from(value: &BSTR) -> ::std::result::Result<Self, Self::Error> {
                    ::std::string::String::from_utf16(value.as_wide())
                }
            }
            impl ::std::convert::TryFrom<BSTR> for ::std::string::String {
                type Error = ::std::string::FromUtf16Error;
                fn try_from(value: BSTR) -> ::std::result::Result<Self, Self::Error> {
                    ::std::string::String::try_from(&value)
                }
            }
            impl ::std::default::Default for BSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::fmt::Display for BSTR {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    use std::fmt::Write;
                    for c in ::std::char::decode_utf16(self.as_wide().iter().cloned()) {
                        f.write_char(c.map_err(|_| ::std::fmt::Error)?)?
                    }
                    Ok(())
                }
            }
            impl ::std::fmt::Debug for BSTR {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::std::write!(f, "{}", self)
                }
            }
            impl ::std::cmp::PartialEq for BSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.as_wide() == other.as_wide()
                }
            }
            impl ::std::cmp::PartialEq<::std::string::String> for BSTR {
                fn eq(&self, other: &::std::string::String) -> bool {
                    self == other.as_str()
                }
            }
            impl ::std::cmp::PartialEq<str> for BSTR {
                fn eq(&self, other: &str) -> bool {
                    self == other
                }
            }
            impl ::std::cmp::PartialEq<&str> for BSTR {
                fn eq(&self, other: &&str) -> bool {
                    self.as_wide().iter().copied().eq(other.encode_utf16())
                }
            }
            impl ::std::cmp::PartialEq<BSTR> for &str {
                fn eq(&self, other: &BSTR) -> bool {
                    other == self
                }
            }
            impl ::std::ops::Drop for BSTR {
                fn drop(&mut self) {
                    if !self.0.is_null() {
                        unsafe { SysFreeString(self as &Self) }
                    }
                }
            }
            unsafe impl ::windows::Abi for BSTR {
                type Abi = ::std::mem::ManuallyDrop<Self>;
                type DefaultType = Self;
                fn set_abi(&mut self) -> *mut Self::Abi {
                    debug_assert!(self.0.is_null());
                    &mut self.0 as *mut _ as _
                }
            }
            pub type BSTR_abi = *mut u16;
            pub const CO_E_NOTINITIALIZED: ::windows::HRESULT =
                ::windows::HRESULT(-2147221008i32 as _);
            pub unsafe fn CloseHandle<'a>(hobject: impl ::windows::IntoParam<'a, HANDLE>) -> BOOL {
                #[cfg(windows)]
                {
                    #[link(name = "KERNEL32")]
                    extern "system" {
                        fn CloseHandle(hobject: HANDLE) -> BOOL;
                    }
                    ::std::mem::transmute(CloseHandle(hobject.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            pub const E_NOINTERFACE: ::windows::HRESULT = ::windows::HRESULT(-2147467262i32 as _);
            pub type FARPROC = unsafe extern "system" fn() -> isize;
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(transparent)]
            pub struct HANDLE(pub isize);
            impl HANDLE {}
            impl ::std::default::Default for HANDLE {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl HANDLE {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for HANDLE {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("HANDLE").field("Value", &self.0).finish()
                }
            }
            impl ::std::cmp::PartialEq for HANDLE {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for HANDLE {}
            unsafe impl ::windows::Abi for HANDLE {
                type Abi = Self;
                type DefaultType = Self;
            }
            impl HANDLE {
                pub const INVALID: Self = Self(-1);
                pub fn is_invalid(&self) -> bool {
                    self.0 == -1
                }
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(transparent)]
            pub struct HINSTANCE(pub isize);
            impl HINSTANCE {}
            impl ::std::default::Default for HINSTANCE {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl HINSTANCE {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for HINSTANCE {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("HINSTANCE")
                        .field("Value", &self.0)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for HINSTANCE {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for HINSTANCE {}
            unsafe impl ::windows::Abi for HINSTANCE {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[repr(transparent)]
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub const NULL: Self = Self(::std::ptr::null_mut());
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::cmp::PartialEq for PSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            unsafe impl ::windows::Abi for PSTR {
                type Abi = Self;
                type DefaultType = Self;
                fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[repr(transparent)]
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct PWSTR(pub *mut u16);
            impl PWSTR {
                pub const NULL: Self = Self(::std::ptr::null_mut());
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PWSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::cmp::PartialEq for PWSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            unsafe impl ::windows::Abi for PWSTR {
                type Abi = Self;
                type DefaultType = Self;
                fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            pub const S_OK: ::windows::HRESULT = ::windows::HRESULT(0i32 as _);
            pub unsafe fn SysAllocStringLen<'a>(
                strin: impl ::windows::IntoParam<'a, PWSTR>,
                ui: u32,
            ) -> BSTR {
                #[cfg(windows)]
                {
                    #[link(name = "OLEAUT32")]
                    extern "system" {
                        fn SysAllocStringLen(strin: PWSTR, ui: u32) -> BSTR;
                    }
                    ::std::mem::transmute(SysAllocStringLen(
                        strin.into_param().abi(),
                        ::std::mem::transmute(ui),
                    ))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            pub unsafe fn SysFreeString<'a>(bstrstring: impl ::windows::IntoParam<'a, BSTR>) {
                #[cfg(windows)]
                {
                    #[link(name = "OLEAUT32")]
                    extern "system" {
                        fn SysFreeString(bstrstring: ::std::mem::ManuallyDrop<BSTR>);
                    }
                    ::std::mem::transmute(SysFreeString(bstrstring.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            pub unsafe fn SysStringLen<'a>(pbstr: impl ::windows::IntoParam<'a, BSTR>) -> u32 {
                #[cfg(windows)]
                {
                    #[link(name = "OLEAUT32")]
                    extern "system" {
                        fn SysStringLen(pbstr: ::std::mem::ManuallyDrop<BSTR>) -> u32;
                    }
                    ::std::mem::transmute(SysStringLen(pbstr.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Security {
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct SECURITY_ATTRIBUTES {
                pub nLength: u32,
                pub lpSecurityDescriptor: *mut ::std::ffi::c_void,
                pub bInheritHandle: super::Foundation::BOOL,
            }
            impl SECURITY_ATTRIBUTES {}
            impl ::std::default::Default for SECURITY_ATTRIBUTES {
                fn default() -> Self {
                    Self {
                        nLength: 0,
                        lpSecurityDescriptor: ::std::ptr::null_mut(),
                        bInheritHandle: ::std::default::Default::default(),
                    }
                }
            }
            impl ::std::fmt::Debug for SECURITY_ATTRIBUTES {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("SECURITY_ATTRIBUTES")
                        .field("nLength", &self.nLength)
                        .field("lpSecurityDescriptor", &self.lpSecurityDescriptor)
                        .field("bInheritHandle", &self.bInheritHandle)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for SECURITY_ATTRIBUTES {
                fn eq(&self, other: &Self) -> bool {
                    self.nLength == other.nLength
                        && self.lpSecurityDescriptor == other.lpSecurityDescriptor
                        && self.bInheritHandle == other.bInheritHandle
                }
            }
            impl ::std::cmp::Eq for SECURITY_ATTRIBUTES {}
            unsafe impl ::windows::Abi for SECURITY_ATTRIBUTES {
                type Abi = Self;
                type DefaultType = Self;
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Com {
                pub unsafe fn CoCreateGuid() -> ::windows::Result<::windows::Guid> {
                    #[cfg(windows)]
                    {
                        #[link(name = "OLE32")]
                        extern "system" {
                            fn CoCreateGuid(pguid: *mut ::windows::Guid) -> ::windows::HRESULT;
                        }
                        let mut result__: <::windows::Guid as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        CoCreateGuid(&mut result__).from_abi::<::windows::Guid>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::std::ffi::c_void {
                    #[cfg(windows)]
                    {
                        #[link(name = "OLE32")]
                        extern "system" {
                            fn CoTaskMemAlloc(cb: usize) -> *mut ::std::ffi::c_void;
                        }
                        ::std::mem::transmute(CoTaskMemAlloc(::std::mem::transmute(cb)))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn CoTaskMemFree(pv: *const ::std::ffi::c_void) {
                    #[cfg(windows)]
                    {
                        #[link(name = "OLE32")]
                        extern "system" {
                            fn CoTaskMemFree(pv: *const ::std::ffi::c_void);
                        }
                        ::std::mem::transmute(CoTaskMemFree(::std::mem::transmute(pv)))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IAgileObject(::windows::IUnknown);
                impl IAgileObject {}
                unsafe impl ::windows::Interface for IAgileObject {
                    type Vtable = IAgileObject_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2498374548,
                        59852,
                        18912,
                        [192, 255, 238, 100, 202, 143, 91, 144],
                    );
                }
                impl ::std::convert::From<IAgileObject> for ::windows::IUnknown {
                    fn from(value: IAgileObject) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IAgileObject> for ::windows::IUnknown {
                    fn from(value: &IAgileObject) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IAgileObject {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IAgileObject {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IAgileObject_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                );
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Diagnostics {
                #[allow(
                    unused_variables,
                    non_upper_case_globals,
                    non_snake_case,
                    unused_unsafe,
                    non_camel_case_types,
                    dead_code,
                    clippy::all
                )]
                pub mod Debug {
                    #[derive(
                        :: std :: cmp :: PartialEq,
                        :: std :: cmp :: Eq,
                        :: std :: marker :: Copy,
                        :: std :: clone :: Clone,
                        :: std :: default :: Default,
                        :: std :: fmt :: Debug,
                    )]
                    #[repr(transparent)]
                    pub struct FORMAT_MESSAGE_OPTIONS(pub u32);
                    pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS =
                        FORMAT_MESSAGE_OPTIONS(256u32);
                    pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: FORMAT_MESSAGE_OPTIONS =
                        FORMAT_MESSAGE_OPTIONS(8192u32);
                    pub const FORMAT_MESSAGE_FROM_HMODULE: FORMAT_MESSAGE_OPTIONS =
                        FORMAT_MESSAGE_OPTIONS(2048u32);
                    pub const FORMAT_MESSAGE_FROM_STRING: FORMAT_MESSAGE_OPTIONS =
                        FORMAT_MESSAGE_OPTIONS(1024u32);
                    pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS =
                        FORMAT_MESSAGE_OPTIONS(4096u32);
                    pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS =
                        FORMAT_MESSAGE_OPTIONS(512u32);
                    impl ::std::convert::From<u32> for FORMAT_MESSAGE_OPTIONS {
                        fn from(value: u32) -> Self {
                            Self(value)
                        }
                    }
                    unsafe impl ::windows::Abi for FORMAT_MESSAGE_OPTIONS {
                        type Abi = Self;
                        type DefaultType = Self;
                    }
                    impl ::std::ops::BitOr for FORMAT_MESSAGE_OPTIONS {
                        type Output = Self;
                        fn bitor(self, rhs: Self) -> Self {
                            Self(self.0 | rhs.0)
                        }
                    }
                    impl ::std::ops::BitAnd for FORMAT_MESSAGE_OPTIONS {
                        type Output = Self;
                        fn bitand(self, rhs: Self) -> Self {
                            Self(self.0 & rhs.0)
                        }
                    }
                    impl ::std::ops::BitOrAssign for FORMAT_MESSAGE_OPTIONS {
                        fn bitor_assign(&mut self, rhs: Self) {
                            self.0.bitor_assign(rhs.0)
                        }
                    }
                    impl ::std::ops::BitAndAssign for FORMAT_MESSAGE_OPTIONS {
                        fn bitand_assign(&mut self, rhs: Self) {
                            self.0.bitand_assign(rhs.0)
                        }
                    }
                    pub unsafe fn FormatMessageW(
                        dwflags: FORMAT_MESSAGE_OPTIONS,
                        lpsource: *const ::std::ffi::c_void,
                        dwmessageid: u32,
                        dwlanguageid: u32,
                        lpbuffer: super::super::super::Foundation::PWSTR,
                        nsize: u32,
                        arguments: *const *const i8,
                    ) -> u32 {
                        #[cfg(windows)]
                        {
                            #[link(name = "KERNEL32")]
                            extern "system" {
                                fn FormatMessageW(
                                    dwflags: FORMAT_MESSAGE_OPTIONS,
                                    lpsource: *const ::std::ffi::c_void,
                                    dwmessageid: u32,
                                    dwlanguageid: u32,
                                    lpbuffer: super::super::super::Foundation::PWSTR,
                                    nsize: u32,
                                    arguments: *const *const i8,
                                ) -> u32;
                            }
                            ::std::mem::transmute(FormatMessageW(
                                ::std::mem::transmute(dwflags),
                                ::std::mem::transmute(lpsource),
                                ::std::mem::transmute(dwmessageid),
                                ::std::mem::transmute(dwlanguageid),
                                ::std::mem::transmute(lpbuffer),
                                ::std::mem::transmute(nsize),
                                ::std::mem::transmute(arguments),
                            ))
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                    pub unsafe fn GetLastError() -> WIN32_ERROR {
                        #[cfg(windows)]
                        {
                            #[link(name = "KERNEL32")]
                            extern "system" {
                                fn GetLastError() -> WIN32_ERROR;
                            }
                            ::std::mem::transmute(GetLastError())
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                    #[derive(
                        :: std :: cmp :: PartialEq,
                        :: std :: cmp :: Eq,
                        :: std :: marker :: Copy,
                        :: std :: clone :: Clone,
                        :: std :: default :: Default,
                        :: std :: fmt :: Debug,
                    )]
                    #[repr(transparent)]
                    pub struct WIN32_ERROR(pub u32);
                    impl ::std::convert::From<u32> for WIN32_ERROR {
                        fn from(value: u32) -> Self {
                            Self(value)
                        }
                    }
                    unsafe impl ::windows::Abi for WIN32_ERROR {
                        type Abi = Self;
                        type DefaultType = Self;
                    }
                    impl ::std::ops::BitOr for WIN32_ERROR {
                        type Output = Self;
                        fn bitor(self, rhs: Self) -> Self {
                            Self(self.0 | rhs.0)
                        }
                    }
                    impl ::std::ops::BitAnd for WIN32_ERROR {
                        type Output = Self;
                        fn bitand(self, rhs: Self) -> Self {
                            Self(self.0 & rhs.0)
                        }
                    }
                    impl ::std::ops::BitOrAssign for WIN32_ERROR {
                        fn bitor_assign(&mut self, rhs: Self) {
                            self.0.bitor_assign(rhs.0)
                        }
                    }
                    impl ::std::ops::BitAndAssign for WIN32_ERROR {
                        fn bitand_assign(&mut self, rhs: Self) {
                            self.0.bitand_assign(rhs.0)
                        }
                    }
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod LibraryLoader {
                pub unsafe fn FreeLibrary<'a>(
                    hlibmodule: impl ::windows::IntoParam<'a, super::super::Foundation::HINSTANCE>,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn FreeLibrary(
                                hlibmodule: super::super::Foundation::HINSTANCE,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(FreeLibrary(hlibmodule.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetProcAddress<'a>(
                    hmodule: impl ::windows::IntoParam<'a, super::super::Foundation::HINSTANCE>,
                    lpprocname: impl ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                ) -> ::std::option::Option<super::super::Foundation::FARPROC> {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn GetProcAddress(
                                hmodule: super::super::Foundation::HINSTANCE,
                                lpprocname: super::super::Foundation::PSTR,
                            ) -> ::std::option::Option<super::super::Foundation::FARPROC>;
                        }
                        ::std::mem::transmute(GetProcAddress(
                            hmodule.into_param().abi(),
                            lpprocname.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn LoadLibraryA<'a>(
                    lplibfilename: impl ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                ) -> super::super::Foundation::HINSTANCE {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn LoadLibraryA(
                                lplibfilename: super::super::Foundation::PSTR,
                            ) -> super::super::Foundation::HINSTANCE;
                        }
                        ::std::mem::transmute(LoadLibraryA(lplibfilename.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Memory {
                pub unsafe fn GetProcessHeap() -> HeapHandle {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn GetProcessHeap() -> HeapHandle;
                        }
                        ::std::mem::transmute(GetProcessHeap())
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
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
                impl ::std::convert::From<u32> for HEAP_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for HEAP_FLAGS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for HEAP_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for HEAP_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for HEAP_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for HEAP_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub unsafe fn HeapAlloc<'a>(
                    hheap: impl ::windows::IntoParam<'a, HeapHandle>,
                    dwflags: HEAP_FLAGS,
                    dwbytes: usize,
                ) -> *mut ::std::ffi::c_void {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn HeapAlloc(
                                hheap: HeapHandle,
                                dwflags: HEAP_FLAGS,
                                dwbytes: usize,
                            ) -> *mut ::std::ffi::c_void;
                        }
                        ::std::mem::transmute(HeapAlloc(
                            hheap.into_param().abi(),
                            ::std::mem::transmute(dwflags),
                            ::std::mem::transmute(dwbytes),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn HeapFree<'a>(
                    hheap: impl ::windows::IntoParam<'a, HeapHandle>,
                    dwflags: HEAP_FLAGS,
                    lpmem: *const ::std::ffi::c_void,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn HeapFree(
                                hheap: HeapHandle,
                                dwflags: HEAP_FLAGS,
                                lpmem: *const ::std::ffi::c_void,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(HeapFree(
                            hheap.into_param().abi(),
                            ::std::mem::transmute(dwflags),
                            ::std::mem::transmute(lpmem),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(transparent)]
                pub struct HeapHandle(pub isize);
                impl HeapHandle {}
                impl ::std::default::Default for HeapHandle {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl HeapHandle {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for HeapHandle {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HeapHandle")
                            .field("Value", &self.0)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for HeapHandle {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for HeapHandle {}
                unsafe impl ::windows::Abi for HeapHandle {
                    type Abi = Self;
                    type DefaultType = Self;
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod OleAutomation {
                pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::Result<IErrorInfo> {
                    #[cfg(windows)]
                    {
                        #[link(name = "OLEAUT32")]
                        extern "system" {
                            fn GetErrorInfo(
                                dwreserved: u32,
                                pperrinfo: *mut ::windows::RawPtr,
                            ) -> ::windows::HRESULT;
                        }
                        let mut result__: <IErrorInfo as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        GetErrorInfo(::std::mem::transmute(dwreserved), &mut result__)
                            .from_abi::<IErrorInfo>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IErrorInfo(::windows::IUnknown);
                impl IErrorInfo {
                    pub unsafe fn GetGUID(&self) -> ::windows::Result<::windows::Guid> {
                        let mut result__: <::windows::Guid as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<::windows::Guid>(result__)
                    }
                    pub unsafe fn GetSource(
                        &self,
                    ) -> ::windows::Result<super::super::Foundation::BSTR> {
                        let mut result__: <super::super::Foundation::BSTR as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::BSTR>(result__)
                    }
                    pub unsafe fn GetDescription(
                        &self,
                    ) -> ::windows::Result<super::super::Foundation::BSTR> {
                        let mut result__: <super::super::Foundation::BSTR as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::BSTR>(result__)
                    }
                    pub unsafe fn GetHelpFile(
                        &self,
                    ) -> ::windows::Result<super::super::Foundation::BSTR> {
                        let mut result__: <super::super::Foundation::BSTR as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::BSTR>(result__)
                    }
                    pub unsafe fn GetHelpContext(&self) -> ::windows::Result<u32> {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                unsafe impl ::windows::Interface for IErrorInfo {
                    type Vtable = IErrorInfo_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        485667104,
                        21629,
                        4123,
                        [142, 101, 8, 0, 43, 43, 209, 25],
                    );
                }
                impl ::std::convert::From<IErrorInfo> for ::windows::IUnknown {
                    fn from(value: IErrorInfo) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IErrorInfo> for ::windows::IUnknown {
                    fn from(value: &IErrorInfo) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IErrorInfo {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IErrorInfo {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IErrorInfo_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pguid: *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pbstrsource: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pbstrdescription: *mut ::std::mem::ManuallyDrop<
                            super::super::Foundation::BSTR,
                        >,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pbstrhelpfile: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pdwhelpcontext: *mut u32,
                    ) -> ::windows::HRESULT,
                );
                pub unsafe fn SetErrorInfo<'a>(
                    dwreserved: u32,
                    perrinfo: impl ::windows::IntoParam<'a, IErrorInfo>,
                ) -> ::windows::Result<()> {
                    #[cfg(windows)]
                    {
                        #[link(name = "OLEAUT32")]
                        extern "system" {
                            fn SetErrorInfo(
                                dwreserved: u32,
                                perrinfo: ::windows::RawPtr,
                            ) -> ::windows::HRESULT;
                        }
                        SetErrorInfo(
                            ::std::mem::transmute(dwreserved),
                            perrinfo.into_param().abi(),
                        )
                        .ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Threading {
                pub unsafe fn CreateEventA<'a>(
                    lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                    bmanualreset: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    binitialstate: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    lpname: impl ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                ) -> super::super::Foundation::HANDLE {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn CreateEventA(
                                lpeventattributes : * const super::super::Security:: SECURITY_ATTRIBUTES,
                                bmanualreset: super::super::Foundation::BOOL,
                                binitialstate: super::super::Foundation::BOOL,
                                lpname: super::super::Foundation::PSTR,
                            ) -> super::super::Foundation::HANDLE;
                        }
                        ::std::mem::transmute(CreateEventA(
                            ::std::mem::transmute(lpeventattributes),
                            bmanualreset.into_param().abi(),
                            binitialstate.into_param().abi(),
                            lpname.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn SetEvent<'a>(
                    hevent: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn SetEvent(
                                hevent: super::super::Foundation::HANDLE,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(SetEvent(hevent.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WAIT_RETURN_CAUSE(pub u32);
                pub const WAIT_OBJECT_0: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(0u32);
                pub const WAIT_ABANDONED: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(128u32);
                pub const WAIT_ABANDONED_0: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(128u32);
                pub const WAIT_IO_COMPLETION: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(192u32);
                pub const WAIT_TIMEOUT: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(258u32);
                pub const WAIT_FAILED: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(4294967295u32);
                impl ::std::convert::From<u32> for WAIT_RETURN_CAUSE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WAIT_RETURN_CAUSE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for WAIT_RETURN_CAUSE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for WAIT_RETURN_CAUSE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for WAIT_RETURN_CAUSE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for WAIT_RETURN_CAUSE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub unsafe fn WaitForSingleObject<'a>(
                    hhandle: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    dwmilliseconds: u32,
                ) -> WAIT_RETURN_CAUSE {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn WaitForSingleObject(
                                hhandle: super::super::Foundation::HANDLE,
                                dwmilliseconds: u32,
                            ) -> WAIT_RETURN_CAUSE;
                        }
                        ::std::mem::transmute(WaitForSingleObject(
                            hhandle.into_param().abi(),
                            ::std::mem::transmute(dwmilliseconds),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WinRT {
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ILanguageExceptionErrorInfo(::windows::IUnknown);
                impl ILanguageExceptionErrorInfo {
                    pub unsafe fn GetLanguageException(
                        &self,
                    ) -> ::windows::Result<::windows::IUnknown> {
                        let mut result__: <::windows::IUnknown as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<::windows::IUnknown>(result__)
                    }
                }
                unsafe impl ::windows::Interface for ILanguageExceptionErrorInfo {
                    type Vtable = ILanguageExceptionErrorInfo_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        77782003,
                        57219,
                        4460,
                        [9, 70, 8, 18, 171, 246, 224, 125],
                    );
                }
                impl ::std::convert::From<ILanguageExceptionErrorInfo> for ::windows::IUnknown {
                    fn from(value: ILanguageExceptionErrorInfo) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ILanguageExceptionErrorInfo> for ::windows::IUnknown {
                    fn from(value: &ILanguageExceptionErrorInfo) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ILanguageExceptionErrorInfo {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &ILanguageExceptionErrorInfo {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ILanguageExceptionErrorInfo_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        languageexception: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ILanguageExceptionErrorInfo2(::windows::IUnknown);
                impl ILanguageExceptionErrorInfo2 {
                    pub unsafe fn GetLanguageException(
                        &self,
                    ) -> ::windows::Result<::windows::IUnknown> {
                        let mut result__: <::windows::IUnknown as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<::windows::IUnknown>(result__)
                    }
                    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(
                        &self,
                    ) -> ::windows::Result<ILanguageExceptionErrorInfo2> {
                        let mut result__: <ILanguageExceptionErrorInfo2 as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<ILanguageExceptionErrorInfo2>(result__)
                    }
                    pub unsafe fn CapturePropagationContext<'a>(
                        &self,
                        languageexception: impl ::windows::IntoParam<'a, ::windows::IUnknown>,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            languageexception.into_param().abi(),
                        )
                        .ok()
                    }
                    pub unsafe fn GetPropagationContextHead(
                        &self,
                    ) -> ::windows::Result<ILanguageExceptionErrorInfo2> {
                        let mut result__: <ILanguageExceptionErrorInfo2 as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<ILanguageExceptionErrorInfo2>(result__)
                    }
                }
                unsafe impl ::windows::Interface for ILanguageExceptionErrorInfo2 {
                    type Vtable = ILanguageExceptionErrorInfo2_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1464264132,
                        23447,
                        16972,
                        [182, 32, 40, 34, 145, 87, 52, 221],
                    );
                }
                impl ::std::convert::From<ILanguageExceptionErrorInfo2> for ::windows::IUnknown {
                    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ILanguageExceptionErrorInfo2> for ::windows::IUnknown {
                    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
                    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
                    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ILanguageExceptionErrorInfo> for ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::Param<'a, ILanguageExceptionErrorInfo> {
                        ::windows::Param::Owned(
                            ::std::convert::Into::<ILanguageExceptionErrorInfo>::into(self),
                        )
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ILanguageExceptionErrorInfo> for &ILanguageExceptionErrorInfo2 {
                    fn into_param(self) -> ::windows::Param<'a, ILanguageExceptionErrorInfo> {
                        ::windows::Param::Owned(
                            ::std::convert::Into::<ILanguageExceptionErrorInfo>::into(
                                ::std::clone::Clone::clone(self),
                            ),
                        )
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ILanguageExceptionErrorInfo2_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        languageexception: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        previouslanguageexceptionerrorinfo: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        languageexception: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        propagatedlanguageexceptionerrorinfohead: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IRestrictedErrorInfo(::windows::IUnknown);
                impl IRestrictedErrorInfo {
                    pub unsafe fn GetErrorDetails(
                        &self,
                        description: *mut super::super::Foundation::BSTR,
                        error: *mut ::windows::HRESULT,
                        restricteddescription: *mut super::super::Foundation::BSTR,
                        capabilitysid: *mut super::super::Foundation::BSTR,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(description),
                            ::std::mem::transmute(error),
                            ::std::mem::transmute(restricteddescription),
                            ::std::mem::transmute(capabilitysid),
                        )
                        .ok()
                    }
                    pub unsafe fn GetReference(
                        &self,
                    ) -> ::windows::Result<super::super::Foundation::BSTR> {
                        let mut result__: <super::super::Foundation::BSTR as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::BSTR>(result__)
                    }
                }
                unsafe impl ::windows::Interface for IRestrictedErrorInfo {
                    type Vtable = IRestrictedErrorInfo_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2193256594,
                        19592,
                        17021,
                        [167, 188, 22, 221, 147, 254, 182, 126],
                    );
                }
                impl ::std::convert::From<IRestrictedErrorInfo> for ::windows::IUnknown {
                    fn from(value: IRestrictedErrorInfo) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IRestrictedErrorInfo> for ::windows::IUnknown {
                    fn from(value: &IRestrictedErrorInfo) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IRestrictedErrorInfo {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IRestrictedErrorInfo {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                unsafe impl ::std::marker::Send for IRestrictedErrorInfo {}
                unsafe impl ::std::marker::Sync for IRestrictedErrorInfo {}
                #[repr(C)]
                #[doc(hidden)]
                pub struct IRestrictedErrorInfo_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                        error: *mut ::windows::HRESULT,
                        restricteddescription: *mut ::std::mem::ManuallyDrop<
                            super::super::Foundation::BSTR,
                        >,
                        capabilitysid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        reference: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IWeakReference(::windows::IUnknown);
                impl IWeakReference {
                    pub unsafe fn Resolve<T: ::windows::Interface>(&self) -> ::windows::Result<T> {
                        let mut result__ = ::std::option::Option::None;
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            &<T as ::windows::Interface>::IID,
                            ::windows::Abi::set_abi(&mut result__),
                        )
                        .and_some(result__)
                    }
                }
                unsafe impl ::windows::Interface for IWeakReference {
                    type Vtable = IWeakReference_abi;
                    const IID: ::windows::Guid =
                        ::windows::Guid::from_values(55, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
                }
                impl ::std::convert::From<IWeakReference> for ::windows::IUnknown {
                    fn from(value: IWeakReference) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IWeakReference> for ::windows::IUnknown {
                    fn from(value: &IWeakReference) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IWeakReference {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IWeakReference {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWeakReference_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        riid: *const ::windows::Guid,
                        objectreference: *mut *mut ::std::ffi::c_void,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IWeakReferenceSource(::windows::IUnknown);
                impl IWeakReferenceSource {
                    pub unsafe fn GetWeakReference(&self) -> ::windows::Result<IWeakReference> {
                        let mut result__: <IWeakReference as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<IWeakReference>(result__)
                    }
                }
                unsafe impl ::windows::Interface for IWeakReferenceSource {
                    type Vtable = IWeakReferenceSource_abi;
                    const IID: ::windows::Guid =
                        ::windows::Guid::from_values(56, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
                }
                impl ::std::convert::From<IWeakReferenceSource> for ::windows::IUnknown {
                    fn from(value: IWeakReferenceSource) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IWeakReferenceSource> for ::windows::IUnknown {
                    fn from(value: &IWeakReferenceSource) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IWeakReferenceSource {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IWeakReferenceSource {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWeakReferenceSource_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        weakreference: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
            }
        }
    }
}
