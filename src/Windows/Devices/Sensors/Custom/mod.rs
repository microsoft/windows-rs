#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CustomSensor(::windows::runtime::IInspectable);
impl CustomSensor {
    pub fn GetCurrentReading(&self) -> ::windows::runtime::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CustomSensorReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ReportInterval(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::TypedEventHandler<
                CustomSensor,
                CustomSensorReadingChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetDeviceSelector<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        interfaceid: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                interfaceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        sensorid: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<CustomSensor>>
    {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                sensorid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IAsyncOperation<CustomSensor>>(result__)
        })
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICustomSensor2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ReportLatency(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ICustomSensorStatics<
        R,
        F: FnOnce(&ICustomSensorStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CustomSensor, ICustomSensorStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CustomSensor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Sensors.Custom.CustomSensor;{a136f9ad-4034-4b4d-99dd-531aac649c09})",
    );
}
unsafe impl ::windows::runtime::Interface for CustomSensor {
    type Vtable = ICustomSensor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2704734637,
        16436,
        19277,
        [153, 221, 83, 26, 172, 100, 156, 9],
    );
}
impl ::windows::runtime::RuntimeName for CustomSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensor";
}
impl ::std::convert::From<CustomSensor> for ::windows::runtime::IUnknown {
    fn from(value: CustomSensor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CustomSensor> for ::windows::runtime::IUnknown {
    fn from(value: &CustomSensor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CustomSensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CustomSensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CustomSensor> for ::windows::runtime::IInspectable {
    fn from(value: CustomSensor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSensor> for ::windows::runtime::IInspectable {
    fn from(value: &CustomSensor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CustomSensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CustomSensor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CustomSensor {}
unsafe impl ::std::marker::Sync for CustomSensor {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CustomSensorReading(::windows::runtime::IInspectable);
impl CustomSensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            ::windows::runtime::IInspectable,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                ::windows::runtime::IInspectable,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>,
    > {
        let this = &::windows::runtime::Interface::cast::<ICustomSensorReading2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::TimeSpan,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CustomSensorReading {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Devices.Sensors.Custom.CustomSensorReading;{64004f4d-446a-4366-a87a-5f963268ec53})" ) ;
}
unsafe impl ::windows::runtime::Interface for CustomSensorReading {
    type Vtable = ICustomSensorReading_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1677741901,
        17514,
        17254,
        [168, 122, 95, 150, 50, 104, 236, 83],
    );
}
impl ::windows::runtime::RuntimeName for CustomSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReading";
}
impl ::std::convert::From<CustomSensorReading> for ::windows::runtime::IUnknown {
    fn from(value: CustomSensorReading) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CustomSensorReading> for ::windows::runtime::IUnknown {
    fn from(value: &CustomSensorReading) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CustomSensorReading {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CustomSensorReading {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CustomSensorReading> for ::windows::runtime::IInspectable {
    fn from(value: CustomSensorReading) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSensorReading> for ::windows::runtime::IInspectable {
    fn from(value: &CustomSensorReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CustomSensorReading
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CustomSensorReading
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CustomSensorReading {}
unsafe impl ::std::marker::Sync for CustomSensorReading {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CustomSensorReadingChangedEventArgs(::windows::runtime::IInspectable);
impl CustomSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows::runtime::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CustomSensorReading>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CustomSensorReadingChangedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs;{6b202023-cffd-4cc1-8ff0-e21823d76fcc})" ) ;
}
unsafe impl ::windows::runtime::Interface for CustomSensorReadingChangedEventArgs {
    type Vtable = ICustomSensorReadingChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1797267491,
        53245,
        19649,
        [143, 240, 226, 24, 35, 215, 111, 204],
    );
}
impl ::windows::runtime::RuntimeName for CustomSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs";
}
impl ::std::convert::From<CustomSensorReadingChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CustomSensorReadingChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CustomSensorReadingChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CustomSensorReadingChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CustomSensorReadingChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CustomSensorReadingChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CustomSensorReadingChangedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CustomSensorReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSensorReadingChangedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CustomSensorReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CustomSensorReadingChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CustomSensorReadingChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CustomSensorReadingChangedEventArgs {}
unsafe impl ::std::marker::Sync for CustomSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSensor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSensor {
    type Vtable = ICustomSensor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2704734637,
        16436,
        19277,
        [153, 221, 83, 26, 172, 100, 156, 9],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensor_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSensor2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSensor2 {
    type Vtable = ICustomSensor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        551235857,
        60504,
        19871,
        [191, 189, 231, 120, 37, 8, 133, 16],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensor2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSensorReading(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSensorReading {
    type Vtable = ICustomSensorReading_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1677741901,
        17514,
        17254,
        [168, 122, 95, 150, 50, 104, 236, 83],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReading_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSensorReading2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSensorReading2 {
    type Vtable = ICustomSensorReading2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        574396650,
        49011,
        18834,
        [154, 72, 211, 200, 151, 89, 76, 203],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReading2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSensorReadingChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSensorReadingChangedEventArgs {
    type Vtable = ICustomSensorReadingChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1797267491,
        53245,
        19649,
        [143, 240, 226, 24, 35, 215, 111, 204],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorReadingChangedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICustomSensorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomSensorStatics {
    type Vtable = ICustomSensorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2569032399,
        62498,
        19581,
        [131, 107, 231, 220, 116, 167, 18, 75],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSensorStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interfaceid: ::windows::runtime::GUID,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sensorid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
