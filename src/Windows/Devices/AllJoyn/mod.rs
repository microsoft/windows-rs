#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynAboutData(pub ::windows::core::IInspectable);
impl AllJoynAboutData {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn DefaultAppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetDefaultAppName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`*"]
    pub fn AppNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn DateOfManufacture(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn SetDateOfManufacture<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn DefaultDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetDefaultDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`*"]
    pub fn Descriptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn DefaultManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetDefaultManufacturer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`*"]
    pub fn Manufacturers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetModelNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetSoftwareVersion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn SetSupportUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetAppId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynAboutData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAboutData;{e5a9bf00-1fa2-4839-93ef-f9df404890f7})");
}
unsafe impl ::windows::core::Interface for AllJoynAboutData {
    type Vtable = IAllJoynAboutData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5a9bf00_1fa2_4839_93ef_f9df404890f7);
}
impl ::windows::core::RuntimeName for AllJoynAboutData {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAboutData";
}
impl ::core::convert::From<AllJoynAboutData> for ::windows::core::IUnknown {
    fn from(value: AllJoynAboutData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynAboutData> for ::windows::core::IUnknown {
    fn from(value: &AllJoynAboutData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynAboutData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynAboutData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynAboutData> for ::windows::core::IInspectable {
    fn from(value: AllJoynAboutData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynAboutData> for ::windows::core::IInspectable {
    fn from(value: &AllJoynAboutData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynAboutData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynAboutData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynAboutData {}
unsafe impl ::core::marker::Sync for AllJoynAboutData {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynAboutDataView(pub ::windows::core::IInspectable);
impl AllJoynAboutDataView {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Status(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AJSoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn DateOfManufacture(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Globalization`*"]
    pub fn DefaultLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn HardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`, `Globalization`*"]
    pub fn SupportedLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn DeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn GetDataBySessionPortAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, AllJoynBusAttachment>>(uniquename: Param0, busattachment: Param1, sessionport: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        Self::IAllJoynAboutDataViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), busattachment.into_param().abi(), sessionport, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`, `Globalization`*"]
    pub fn GetDataBySessionPortWithLanguageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, AllJoynBusAttachment>, Param3: ::windows::core::IntoParam<'a, super::super::Globalization::Language>>(uniquename: Param0, busattachment: Param1, sessionport: u16, language: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        Self::IAllJoynAboutDataViewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), busattachment.into_param().abi(), sessionport, language.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        })
    }
    pub fn IAllJoynAboutDataViewStatics<R, F: FnOnce(&IAllJoynAboutDataViewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynAboutDataView, IAllJoynAboutDataViewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynAboutDataView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAboutDataView;{6823111f-6212-4934-9c48-e19ca4984288})");
}
unsafe impl ::windows::core::Interface for AllJoynAboutDataView {
    type Vtable = IAllJoynAboutDataView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6823111f_6212_4934_9c48_e19ca4984288);
}
impl ::windows::core::RuntimeName for AllJoynAboutDataView {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAboutDataView";
}
impl ::core::convert::From<AllJoynAboutDataView> for ::windows::core::IUnknown {
    fn from(value: AllJoynAboutDataView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynAboutDataView> for ::windows::core::IUnknown {
    fn from(value: &AllJoynAboutDataView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynAboutDataView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynAboutDataView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynAboutDataView> for ::windows::core::IInspectable {
    fn from(value: AllJoynAboutDataView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynAboutDataView> for ::windows::core::IInspectable {
    fn from(value: &AllJoynAboutDataView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynAboutDataView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynAboutDataView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynAboutDataView {}
unsafe impl ::core::marker::Sync for AllJoynAboutDataView {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynAcceptSessionJoinerEventArgs(pub ::windows::core::IInspectable);
impl AllJoynAcceptSessionJoinerEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SessionPort(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn TrafficType(&self) -> ::windows::core::Result<AllJoynTrafficType> {
        let this = self;
        unsafe {
            let mut result__: AllJoynTrafficType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynTrafficType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SamePhysicalNode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SameNetwork(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, IAllJoynAcceptSessionJoiner>>(uniquename: Param0, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: Param4) -> ::windows::core::Result<AllJoynAcceptSessionJoinerEventArgs> {
        Self::IAllJoynAcceptSessionJoinerEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), sessionport, traffictype, proximity, acceptsessionjoiner.into_param().abi(), &mut result__).from_abi::<AllJoynAcceptSessionJoinerEventArgs>(result__)
        })
    }
    pub fn IAllJoynAcceptSessionJoinerEventArgsFactory<R, F: FnOnce(&IAllJoynAcceptSessionJoinerEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynAcceptSessionJoinerEventArgs, IAllJoynAcceptSessionJoinerEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynAcceptSessionJoinerEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAcceptSessionJoinerEventArgs;{4efb5365-3e8a-4257-8f10-539ce0d56c0f})");
}
unsafe impl ::windows::core::Interface for AllJoynAcceptSessionJoinerEventArgs {
    type Vtable = IAllJoynAcceptSessionJoinerEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4efb5365_3e8a_4257_8f10_539ce0d56c0f);
}
impl ::windows::core::RuntimeName for AllJoynAcceptSessionJoinerEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAcceptSessionJoinerEventArgs";
}
impl ::core::convert::From<AllJoynAcceptSessionJoinerEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynAcceptSessionJoinerEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynAcceptSessionJoinerEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynAcceptSessionJoinerEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynAcceptSessionJoinerEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynAcceptSessionJoinerEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynAcceptSessionJoinerEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynAcceptSessionJoinerEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynAcceptSessionJoinerEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynAcceptSessionJoinerEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynAuthenticationCompleteEventArgs(pub ::windows::core::IInspectable);
impl AllJoynAuthenticationCompleteEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism> {
        let this = self;
        unsafe {
            let mut result__: AllJoynAuthenticationMechanism = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynAuthenticationMechanism>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynAuthenticationCompleteEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAuthenticationCompleteEventArgs;{97b4701c-15dc-4b53-b6a4-7d134300d7bf})");
}
unsafe impl ::windows::core::Interface for AllJoynAuthenticationCompleteEventArgs {
    type Vtable = IAllJoynAuthenticationCompleteEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97b4701c_15dc_4b53_b6a4_7d134300d7bf);
}
impl ::windows::core::RuntimeName for AllJoynAuthenticationCompleteEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAuthenticationCompleteEventArgs";
}
impl ::core::convert::From<AllJoynAuthenticationCompleteEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynAuthenticationCompleteEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynAuthenticationCompleteEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynAuthenticationCompleteEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynAuthenticationCompleteEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynAuthenticationCompleteEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynAuthenticationCompleteEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynAuthenticationCompleteEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynAuthenticationCompleteEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynAuthenticationCompleteEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AllJoynAuthenticationMechanism(pub i32);
impl AllJoynAuthenticationMechanism {
    pub const None: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(0i32);
    pub const SrpAnonymous: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(1i32);
    pub const SrpLogon: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(2i32);
    pub const EcdheNull: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(3i32);
    pub const EcdhePsk: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(4i32);
    pub const EcdheEcdsa: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(5i32);
    pub const EcdheSpeke: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(6i32);
}
impl ::core::convert::From<i32> for AllJoynAuthenticationMechanism {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AllJoynAuthenticationMechanism {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AllJoynAuthenticationMechanism {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynAuthenticationMechanism;i4)");
}
impl ::windows::core::DefaultType for AllJoynAuthenticationMechanism {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynBusAttachment(pub ::windows::core::IInspectable);
impl AllJoynBusAttachment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynBusAttachment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AboutData(&self) -> ::windows::core::Result<AllJoynAboutData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynAboutData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn ConnectionSpecification(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn State(&self) -> ::windows::core::Result<AllJoynBusAttachmentState> {
        let this = self;
        unsafe {
            let mut result__: AllJoynBusAttachmentState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynBusAttachmentState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn PingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, uniquename: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Connect(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Disconnect(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynBusAttachmentStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`*"]
    pub fn AuthenticationMechanisms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<AllJoynAuthenticationMechanism>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<AllJoynAuthenticationMechanism>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn CredentialsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveCredentialsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn CredentialsVerificationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsVerificationRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveCredentialsVerificationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn AuthenticationComplete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAuthenticationCompleteEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveAuthenticationComplete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(connectionspecification: Param0) -> ::windows::core::Result<AllJoynBusAttachment> {
        Self::IAllJoynBusAttachmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectionspecification.into_param().abi(), &mut result__).from_abi::<AllJoynBusAttachment>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn GetAboutDataAsync<'a, Param0: ::windows::core::IntoParam<'a, AllJoynServiceInfo>>(&self, serviceinfo: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        let this = &::windows::core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), serviceinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`, `Globalization`*"]
    pub fn GetAboutDataWithLanguageAsync<'a, Param0: ::windows::core::IntoParam<'a, AllJoynServiceInfo>, Param1: ::windows::core::IntoParam<'a, super::super::Globalization::Language>>(&self, serviceinfo: Param0, language: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        let this = &::windows::core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), serviceinfo.into_param().abi(), language.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn AcceptSessionJoinerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAcceptSessionJoinerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveAcceptSessionJoinerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn SessionJoined<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynSessionJoinedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveSessionJoined<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn GetDefault() -> ::windows::core::Result<AllJoynBusAttachment> {
        Self::IAllJoynBusAttachmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynBusAttachment>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_AllJoyn`, `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn GetWatcher<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(requiredinterfaces: Param0) -> ::windows::core::Result<super::Enumeration::DeviceWatcher> {
        Self::IAllJoynBusAttachmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), requiredinterfaces.into_param().abi(), &mut result__).from_abi::<super::Enumeration::DeviceWatcher>(result__)
        })
    }
    pub fn IAllJoynBusAttachmentFactory<R, F: FnOnce(&IAllJoynBusAttachmentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynBusAttachment, IAllJoynBusAttachmentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAllJoynBusAttachmentStatics<R, F: FnOnce(&IAllJoynBusAttachmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynBusAttachment, IAllJoynBusAttachmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynBusAttachment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusAttachment;{f309f153-1eed-42c3-a20e-436d41fe62f6})");
}
unsafe impl ::windows::core::Interface for AllJoynBusAttachment {
    type Vtable = IAllJoynBusAttachment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf309f153_1eed_42c3_a20e_436d41fe62f6);
}
impl ::windows::core::RuntimeName for AllJoynBusAttachment {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusAttachment";
}
impl ::core::convert::From<AllJoynBusAttachment> for ::windows::core::IUnknown {
    fn from(value: AllJoynBusAttachment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynBusAttachment> for ::windows::core::IUnknown {
    fn from(value: &AllJoynBusAttachment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynBusAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynBusAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynBusAttachment> for ::windows::core::IInspectable {
    fn from(value: AllJoynBusAttachment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynBusAttachment> for ::windows::core::IInspectable {
    fn from(value: &AllJoynBusAttachment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynBusAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynBusAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynBusAttachment {}
unsafe impl ::core::marker::Sync for AllJoynBusAttachment {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AllJoynBusAttachmentState(pub i32);
impl AllJoynBusAttachmentState {
    pub const Disconnected: AllJoynBusAttachmentState = AllJoynBusAttachmentState(0i32);
    pub const Connecting: AllJoynBusAttachmentState = AllJoynBusAttachmentState(1i32);
    pub const Connected: AllJoynBusAttachmentState = AllJoynBusAttachmentState(2i32);
    pub const Disconnecting: AllJoynBusAttachmentState = AllJoynBusAttachmentState(3i32);
}
impl ::core::convert::From<i32> for AllJoynBusAttachmentState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AllJoynBusAttachmentState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AllJoynBusAttachmentState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynBusAttachmentState;i4)");
}
impl ::windows::core::DefaultType for AllJoynBusAttachmentState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynBusAttachmentStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynBusAttachmentStateChangedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn State(&self) -> ::windows::core::Result<AllJoynBusAttachmentState> {
        let this = self;
        unsafe {
            let mut result__: AllJoynBusAttachmentState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynBusAttachmentState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Status(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynBusAttachmentStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusAttachmentStateChangedEventArgs;{d82e75f4-c02a-41ec-a8d5-eab1558953aa})");
}
unsafe impl ::windows::core::Interface for AllJoynBusAttachmentStateChangedEventArgs {
    type Vtable = IAllJoynBusAttachmentStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd82e75f4_c02a_41ec_a8d5_eab1558953aa);
}
impl ::windows::core::RuntimeName for AllJoynBusAttachmentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusAttachmentStateChangedEventArgs";
}
impl ::core::convert::From<AllJoynBusAttachmentStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynBusAttachmentStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynBusAttachmentStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynBusAttachmentStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynBusAttachmentStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynBusAttachmentStateChangedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynBusObject(pub ::windows::core::IInspectable);
impl AllJoynBusObject {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynBusObject, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AddProducer<'a, Param0: ::windows::core::IntoParam<'a, IAllJoynProducer>>(&self, producer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), producer.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn BusAttachment(&self) -> ::windows::core::Result<AllJoynBusAttachment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynBusAttachment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Session(&self) -> ::windows::core::Result<AllJoynSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynSession>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusObject, AllJoynBusObjectStoppedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(objectpath: Param0) -> ::windows::core::Result<AllJoynBusObject> {
        Self::IAllJoynBusObjectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), objectpath.into_param().abi(), &mut result__).from_abi::<AllJoynBusObject>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn CreateWithBusAttachment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, AllJoynBusAttachment>>(objectpath: Param0, busattachment: Param1) -> ::windows::core::Result<AllJoynBusObject> {
        Self::IAllJoynBusObjectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), objectpath.into_param().abi(), busattachment.into_param().abi(), &mut result__).from_abi::<AllJoynBusObject>(result__)
        })
    }
    pub fn IAllJoynBusObjectFactory<R, F: FnOnce(&IAllJoynBusObjectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynBusObject, IAllJoynBusObjectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynBusObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusObject;{e8fd825e-f73a-490c-8804-04e026643047})");
}
unsafe impl ::windows::core::Interface for AllJoynBusObject {
    type Vtable = IAllJoynBusObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8fd825e_f73a_490c_8804_04e026643047);
}
impl ::windows::core::RuntimeName for AllJoynBusObject {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusObject";
}
impl ::core::convert::From<AllJoynBusObject> for ::windows::core::IUnknown {
    fn from(value: AllJoynBusObject) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynBusObject> for ::windows::core::IUnknown {
    fn from(value: &AllJoynBusObject) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynBusObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynBusObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynBusObject> for ::windows::core::IInspectable {
    fn from(value: AllJoynBusObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynBusObject> for ::windows::core::IInspectable {
    fn from(value: &AllJoynBusObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynBusObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynBusObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynBusObject {}
unsafe impl ::core::marker::Sync for AllJoynBusObject {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynBusObjectStoppedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynBusObjectStoppedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Status(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create(status: i32) -> ::windows::core::Result<AllJoynBusObjectStoppedEventArgs> {
        Self::IAllJoynBusObjectStoppedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), status, &mut result__).from_abi::<AllJoynBusObjectStoppedEventArgs>(result__)
        })
    }
    pub fn IAllJoynBusObjectStoppedEventArgsFactory<R, F: FnOnce(&IAllJoynBusObjectStoppedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynBusObjectStoppedEventArgs, IAllJoynBusObjectStoppedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynBusObjectStoppedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusObjectStoppedEventArgs;{de102115-ef8e-4d42-b93b-a2ae74519766})");
}
unsafe impl ::windows::core::Interface for AllJoynBusObjectStoppedEventArgs {
    type Vtable = IAllJoynBusObjectStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde102115_ef8e_4d42_b93b_a2ae74519766);
}
impl ::windows::core::RuntimeName for AllJoynBusObjectStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusObjectStoppedEventArgs";
}
impl ::core::convert::From<AllJoynBusObjectStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynBusObjectStoppedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynBusObjectStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynBusObjectStoppedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynBusObjectStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynBusObjectStoppedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynBusObjectStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynBusObjectStoppedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynBusObjectStoppedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynBusObjectStoppedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynCredentials(pub ::windows::core::IInspectable);
impl AllJoynCredentials {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism> {
        let this = self;
        unsafe {
            let mut result__: AllJoynAuthenticationMechanism = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynAuthenticationMechanism>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Security_Cryptography_Certificates`*"]
    pub fn Certificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Security_Cryptography_Certificates`*"]
    pub fn SetCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Cryptography::Certificates::Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Security_Credentials`*"]
    pub fn PasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Security_Credentials`*"]
    pub fn SetPasswordCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn Timeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn SetTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynCredentials {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynCredentials;{824650f2-a190-40b1-abab-349ec244dfaa})");
}
unsafe impl ::windows::core::Interface for AllJoynCredentials {
    type Vtable = IAllJoynCredentials_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x824650f2_a190_40b1_abab_349ec244dfaa);
}
impl ::windows::core::RuntimeName for AllJoynCredentials {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynCredentials";
}
impl ::core::convert::From<AllJoynCredentials> for ::windows::core::IUnknown {
    fn from(value: AllJoynCredentials) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynCredentials> for ::windows::core::IUnknown {
    fn from(value: &AllJoynCredentials) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynCredentials> for ::windows::core::IInspectable {
    fn from(value: AllJoynCredentials) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynCredentials> for ::windows::core::IInspectable {
    fn from(value: &AllJoynCredentials) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynCredentials {}
unsafe impl ::core::marker::Sync for AllJoynCredentials {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynCredentialsRequestedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynCredentialsRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AttemptCount(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Credentials(&self) -> ::windows::core::Result<AllJoynCredentials> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynCredentials>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn RequestedUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynCredentialsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynCredentialsRequestedEventArgs;{6a87e34e-b069-4b80-9e1a-41bc837c65d2})");
}
unsafe impl ::windows::core::Interface for AllJoynCredentialsRequestedEventArgs {
    type Vtable = IAllJoynCredentialsRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a87e34e_b069_4b80_9e1a_41bc837c65d2);
}
impl ::windows::core::RuntimeName for AllJoynCredentialsRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynCredentialsRequestedEventArgs";
}
impl ::core::convert::From<AllJoynCredentialsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynCredentialsRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynCredentialsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynCredentialsRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynCredentialsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynCredentialsRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynCredentialsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynCredentialsRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynCredentialsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynCredentialsRequestedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynCredentialsVerificationRequestedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynCredentialsVerificationRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism> {
        let this = self;
        unsafe {
            let mut result__: AllJoynAuthenticationMechanism = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynAuthenticationMechanism>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Security_Cryptography_Certificates`*"]
    pub fn PeerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Networking_Sockets`*"]
    pub fn PeerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: super::super::Networking::Sockets::SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Sockets::SocketSslErrorSeverity>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`, `Security_Cryptography_Certificates`*"]
    pub fn PeerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation_Collections`, `Security_Cryptography_Certificates`*"]
    pub fn PeerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynCredentialsVerificationRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynCredentialsVerificationRequestedEventArgs;{800a7612-b805-44af-a2e1-792ab655a2d0})");
}
unsafe impl ::windows::core::Interface for AllJoynCredentialsVerificationRequestedEventArgs {
    type Vtable = IAllJoynCredentialsVerificationRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x800a7612_b805_44af_a2e1_792ab655a2d0);
}
impl ::windows::core::RuntimeName for AllJoynCredentialsVerificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynCredentialsVerificationRequestedEventArgs";
}
impl ::core::convert::From<AllJoynCredentialsVerificationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynCredentialsVerificationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynCredentialsVerificationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynCredentialsVerificationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynCredentialsVerificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynCredentialsVerificationRequestedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynMessageInfo(pub ::windows::core::IInspectable);
impl AllJoynMessageInfo {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SenderUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(senderuniquename: Param0) -> ::windows::core::Result<AllJoynMessageInfo> {
        Self::IAllJoynMessageInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), senderuniquename.into_param().abi(), &mut result__).from_abi::<AllJoynMessageInfo>(result__)
        })
    }
    pub fn IAllJoynMessageInfoFactory<R, F: FnOnce(&IAllJoynMessageInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynMessageInfo, IAllJoynMessageInfoFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynMessageInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynMessageInfo;{ff2b0127-2c12-4859-aa3a-c74461ee814c})");
}
unsafe impl ::windows::core::Interface for AllJoynMessageInfo {
    type Vtable = IAllJoynMessageInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2b0127_2c12_4859_aa3a_c74461ee814c);
}
impl ::windows::core::RuntimeName for AllJoynMessageInfo {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynMessageInfo";
}
impl ::core::convert::From<AllJoynMessageInfo> for ::windows::core::IUnknown {
    fn from(value: AllJoynMessageInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynMessageInfo> for ::windows::core::IUnknown {
    fn from(value: &AllJoynMessageInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynMessageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynMessageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynMessageInfo> for ::windows::core::IInspectable {
    fn from(value: AllJoynMessageInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynMessageInfo> for ::windows::core::IInspectable {
    fn from(value: &AllJoynMessageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynMessageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynMessageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynMessageInfo {}
unsafe impl ::core::marker::Sync for AllJoynMessageInfo {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynProducerStoppedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynProducerStoppedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Status(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create(status: i32) -> ::windows::core::Result<AllJoynProducerStoppedEventArgs> {
        Self::IAllJoynProducerStoppedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), status, &mut result__).from_abi::<AllJoynProducerStoppedEventArgs>(result__)
        })
    }
    pub fn IAllJoynProducerStoppedEventArgsFactory<R, F: FnOnce(&IAllJoynProducerStoppedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynProducerStoppedEventArgs, IAllJoynProducerStoppedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynProducerStoppedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynProducerStoppedEventArgs;{51309770-4937-492d-8080-236439987ceb})");
}
unsafe impl ::windows::core::Interface for AllJoynProducerStoppedEventArgs {
    type Vtable = IAllJoynProducerStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51309770_4937_492d_8080_236439987ceb);
}
impl ::windows::core::RuntimeName for AllJoynProducerStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynProducerStoppedEventArgs";
}
impl ::core::convert::From<AllJoynProducerStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynProducerStoppedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynProducerStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynProducerStoppedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynProducerStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynProducerStoppedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynProducerStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynProducerStoppedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynProducerStoppedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynProducerStoppedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynServiceInfo(pub ::windows::core::IInspectable);
impl AllJoynServiceInfo {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn ObjectPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SessionPort(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uniquename: Param0, objectpath: Param1, sessionport: u16) -> ::windows::core::Result<AllJoynServiceInfo> {
        Self::IAllJoynServiceInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), objectpath.into_param().abi(), sessionport, &mut result__).from_abi::<AllJoynServiceInfo>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynServiceInfo>> {
        Self::IAllJoynServiceInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynServiceInfo>>(result__)
        })
    }
    pub fn IAllJoynServiceInfoFactory<R, F: FnOnce(&IAllJoynServiceInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynServiceInfo, IAllJoynServiceInfoFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAllJoynServiceInfoStatics<R, F: FnOnce(&IAllJoynServiceInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynServiceInfo, IAllJoynServiceInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynServiceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynServiceInfo;{4cbe8209-b93e-4182-999b-ddd000f9c575})");
}
unsafe impl ::windows::core::Interface for AllJoynServiceInfo {
    type Vtable = IAllJoynServiceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cbe8209_b93e_4182_999b_ddd000f9c575);
}
impl ::windows::core::RuntimeName for AllJoynServiceInfo {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynServiceInfo";
}
impl ::core::convert::From<AllJoynServiceInfo> for ::windows::core::IUnknown {
    fn from(value: AllJoynServiceInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynServiceInfo> for ::windows::core::IUnknown {
    fn from(value: &AllJoynServiceInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynServiceInfo> for ::windows::core::IInspectable {
    fn from(value: AllJoynServiceInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynServiceInfo> for ::windows::core::IInspectable {
    fn from(value: &AllJoynServiceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynServiceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynServiceInfo {}
unsafe impl ::core::marker::Sync for AllJoynServiceInfo {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynServiceInfoRemovedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynServiceInfoRemovedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uniquename: Param0) -> ::windows::core::Result<AllJoynServiceInfoRemovedEventArgs> {
        Self::IAllJoynServiceInfoRemovedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), &mut result__).from_abi::<AllJoynServiceInfoRemovedEventArgs>(result__)
        })
    }
    pub fn IAllJoynServiceInfoRemovedEventArgsFactory<R, F: FnOnce(&IAllJoynServiceInfoRemovedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynServiceInfoRemovedEventArgs, IAllJoynServiceInfoRemovedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynServiceInfoRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynServiceInfoRemovedEventArgs;{3057a95f-1d3f-41f3-8969-e32792627396})");
}
unsafe impl ::windows::core::Interface for AllJoynServiceInfoRemovedEventArgs {
    type Vtable = IAllJoynServiceInfoRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3057a95f_1d3f_41f3_8969_e32792627396);
}
impl ::windows::core::RuntimeName for AllJoynServiceInfoRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynServiceInfoRemovedEventArgs";
}
impl ::core::convert::From<AllJoynServiceInfoRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynServiceInfoRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynServiceInfoRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynServiceInfoRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynServiceInfoRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynServiceInfoRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynServiceInfoRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynServiceInfoRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynServiceInfoRemovedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynServiceInfoRemovedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynSession(pub ::windows::core::IInspectable);
impl AllJoynSession {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Status(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveMemberAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, uniquename: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn MemberAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberAddedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveMemberAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn MemberRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveMemberRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn Lost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionLostEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn RemoveLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn GetFromServiceInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, AllJoynServiceInfo>>(serviceinfo: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>> {
        Self::IAllJoynSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), serviceinfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynSession>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_AllJoyn`, `Foundation`*"]
    pub fn GetFromServiceInfoAndBusAttachmentAsync<'a, Param0: ::windows::core::IntoParam<'a, AllJoynServiceInfo>, Param1: ::windows::core::IntoParam<'a, AllJoynBusAttachment>>(serviceinfo: Param0, busattachment: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>> {
        Self::IAllJoynSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), serviceinfo.into_param().abi(), busattachment.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynSession>>(result__)
        })
    }
    pub fn IAllJoynSessionStatics<R, F: FnOnce(&IAllJoynSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynSession, IAllJoynSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSession;{e8d11b0c-c0d4-406c-88a9-a93efa85d4b1})");
}
unsafe impl ::windows::core::Interface for AllJoynSession {
    type Vtable = IAllJoynSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d11b0c_c0d4_406c_88a9_a93efa85d4b1);
}
impl ::windows::core::RuntimeName for AllJoynSession {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSession";
}
impl ::core::convert::From<AllJoynSession> for ::windows::core::IUnknown {
    fn from(value: AllJoynSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynSession> for ::windows::core::IUnknown {
    fn from(value: &AllJoynSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynSession> for ::windows::core::IInspectable {
    fn from(value: AllJoynSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynSession> for ::windows::core::IInspectable {
    fn from(value: &AllJoynSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynSession {}
unsafe impl ::core::marker::Sync for AllJoynSession {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynSessionJoinedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynSessionJoinedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Session(&self) -> ::windows::core::Result<AllJoynSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynSession>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AllJoynSession>>(session: Param0) -> ::windows::core::Result<AllJoynSessionJoinedEventArgs> {
        Self::IAllJoynSessionJoinedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), session.into_param().abi(), &mut result__).from_abi::<AllJoynSessionJoinedEventArgs>(result__)
        })
    }
    pub fn IAllJoynSessionJoinedEventArgsFactory<R, F: FnOnce(&IAllJoynSessionJoinedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynSessionJoinedEventArgs, IAllJoynSessionJoinedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynSessionJoinedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionJoinedEventArgs;{9e9f5bd0-b5d7-47c5-8dab-b040cc192871})");
}
unsafe impl ::windows::core::Interface for AllJoynSessionJoinedEventArgs {
    type Vtable = IAllJoynSessionJoinedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e9f5bd0_b5d7_47c5_8dab_b040cc192871);
}
impl ::windows::core::RuntimeName for AllJoynSessionJoinedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionJoinedEventArgs";
}
impl ::core::convert::From<AllJoynSessionJoinedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynSessionJoinedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynSessionJoinedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynSessionJoinedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynSessionJoinedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynSessionJoinedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynSessionJoinedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynSessionJoinedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynSessionJoinedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynSessionJoinedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynSessionLostEventArgs(pub ::windows::core::IInspectable);
impl AllJoynSessionLostEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Reason(&self) -> ::windows::core::Result<AllJoynSessionLostReason> {
        let this = self;
        unsafe {
            let mut result__: AllJoynSessionLostReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AllJoynSessionLostReason>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create(reason: AllJoynSessionLostReason) -> ::windows::core::Result<AllJoynSessionLostEventArgs> {
        Self::IAllJoynSessionLostEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), reason, &mut result__).from_abi::<AllJoynSessionLostEventArgs>(result__)
        })
    }
    pub fn IAllJoynSessionLostEventArgsFactory<R, F: FnOnce(&IAllJoynSessionLostEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynSessionLostEventArgs, IAllJoynSessionLostEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynSessionLostEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionLostEventArgs;{e766a48a-8bb8-4954-ae67-d2fa43d1f96b})");
}
unsafe impl ::windows::core::Interface for AllJoynSessionLostEventArgs {
    type Vtable = IAllJoynSessionLostEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe766a48a_8bb8_4954_ae67_d2fa43d1f96b);
}
impl ::windows::core::RuntimeName for AllJoynSessionLostEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionLostEventArgs";
}
impl ::core::convert::From<AllJoynSessionLostEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynSessionLostEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynSessionLostEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynSessionLostEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynSessionLostEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynSessionLostEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynSessionLostEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynSessionLostEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynSessionLostEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynSessionLostEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AllJoynSessionLostReason(pub i32);
impl AllJoynSessionLostReason {
    pub const None: AllJoynSessionLostReason = AllJoynSessionLostReason(0i32);
    pub const ProducerLeftSession: AllJoynSessionLostReason = AllJoynSessionLostReason(1i32);
    pub const ProducerClosedAbruptly: AllJoynSessionLostReason = AllJoynSessionLostReason(2i32);
    pub const RemovedByProducer: AllJoynSessionLostReason = AllJoynSessionLostReason(3i32);
    pub const LinkTimeout: AllJoynSessionLostReason = AllJoynSessionLostReason(4i32);
    pub const Other: AllJoynSessionLostReason = AllJoynSessionLostReason(5i32);
}
impl ::core::convert::From<i32> for AllJoynSessionLostReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AllJoynSessionLostReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AllJoynSessionLostReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynSessionLostReason;i4)");
}
impl ::windows::core::DefaultType for AllJoynSessionLostReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynSessionMemberAddedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynSessionMemberAddedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uniquename: Param0) -> ::windows::core::Result<AllJoynSessionMemberAddedEventArgs> {
        Self::IAllJoynSessionMemberAddedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), &mut result__).from_abi::<AllJoynSessionMemberAddedEventArgs>(result__)
        })
    }
    pub fn IAllJoynSessionMemberAddedEventArgsFactory<R, F: FnOnce(&IAllJoynSessionMemberAddedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynSessionMemberAddedEventArgs, IAllJoynSessionMemberAddedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynSessionMemberAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionMemberAddedEventArgs;{49a2798a-0dd1-46c1-9cd6-27190e503a5e})");
}
unsafe impl ::windows::core::Interface for AllJoynSessionMemberAddedEventArgs {
    type Vtable = IAllJoynSessionMemberAddedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49a2798a_0dd1_46c1_9cd6_27190e503a5e);
}
impl ::windows::core::RuntimeName for AllJoynSessionMemberAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionMemberAddedEventArgs";
}
impl ::core::convert::From<AllJoynSessionMemberAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynSessionMemberAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynSessionMemberAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynSessionMemberAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynSessionMemberAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynSessionMemberAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynSessionMemberAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynSessionMemberAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynSessionMemberAddedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynSessionMemberAddedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynSessionMemberRemovedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynSessionMemberRemovedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uniquename: Param0) -> ::windows::core::Result<AllJoynSessionMemberRemovedEventArgs> {
        Self::IAllJoynSessionMemberRemovedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uniquename.into_param().abi(), &mut result__).from_abi::<AllJoynSessionMemberRemovedEventArgs>(result__)
        })
    }
    pub fn IAllJoynSessionMemberRemovedEventArgsFactory<R, F: FnOnce(&IAllJoynSessionMemberRemovedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynSessionMemberRemovedEventArgs, IAllJoynSessionMemberRemovedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynSessionMemberRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionMemberRemovedEventArgs;{409a219f-aa4a-4893-b430-baa1b63c6219})");
}
unsafe impl ::windows::core::Interface for AllJoynSessionMemberRemovedEventArgs {
    type Vtable = IAllJoynSessionMemberRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x409a219f_aa4a_4893_b430_baa1b63c6219);
}
impl ::windows::core::RuntimeName for AllJoynSessionMemberRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionMemberRemovedEventArgs";
}
impl ::core::convert::From<AllJoynSessionMemberRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynSessionMemberRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynSessionMemberRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynSessionMemberRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynSessionMemberRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynSessionMemberRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynSessionMemberRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynSessionMemberRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynSessionMemberRemovedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynSessionMemberRemovedEventArgs {}
#[doc = "*Required features: `Devices_AllJoyn`*"]
pub struct AllJoynStatus {}
impl AllJoynStatus {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Ok() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Fail() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn OperationTimedOut() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn OtherEndClosed() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn ConnectionRefused() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AuthenticationFailed() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn AuthenticationRejectedByUser() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SslConnectFailed() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SslIdentityVerificationFailed() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InsufficientSecurity() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument1() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument2() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument3() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument4() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument5() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument6() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument7() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn InvalidArgument8() -> ::windows::core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn IAllJoynStatusStatics<R, F: FnOnce(&IAllJoynStatusStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynStatus, IAllJoynStatusStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AllJoynStatus {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynStatus";
}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AllJoynTrafficType(pub i32);
impl AllJoynTrafficType {
    pub const Unknown: AllJoynTrafficType = AllJoynTrafficType(0i32);
    pub const Messages: AllJoynTrafficType = AllJoynTrafficType(1i32);
    pub const RawUnreliable: AllJoynTrafficType = AllJoynTrafficType(2i32);
    pub const RawReliable: AllJoynTrafficType = AllJoynTrafficType(4i32);
}
impl ::core::convert::From<i32> for AllJoynTrafficType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AllJoynTrafficType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AllJoynTrafficType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynTrafficType;i4)");
}
impl ::windows::core::DefaultType for AllJoynTrafficType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_AllJoyn`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AllJoynWatcherStoppedEventArgs(pub ::windows::core::IInspectable);
impl AllJoynWatcherStoppedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Status(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Create(status: i32) -> ::windows::core::Result<AllJoynWatcherStoppedEventArgs> {
        Self::IAllJoynWatcherStoppedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), status, &mut result__).from_abi::<AllJoynWatcherStoppedEventArgs>(result__)
        })
    }
    pub fn IAllJoynWatcherStoppedEventArgsFactory<R, F: FnOnce(&IAllJoynWatcherStoppedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AllJoynWatcherStoppedEventArgs, IAllJoynWatcherStoppedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AllJoynWatcherStoppedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynWatcherStoppedEventArgs;{c9fca03b-701d-4aa8-97dd-a2bb0a8f5fa3})");
}
unsafe impl ::windows::core::Interface for AllJoynWatcherStoppedEventArgs {
    type Vtable = IAllJoynWatcherStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9fca03b_701d_4aa8_97dd_a2bb0a8f5fa3);
}
impl ::windows::core::RuntimeName for AllJoynWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynWatcherStoppedEventArgs";
}
impl ::core::convert::From<AllJoynWatcherStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AllJoynWatcherStoppedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AllJoynWatcherStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AllJoynWatcherStoppedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AllJoynWatcherStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AllJoynWatcherStoppedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AllJoynWatcherStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AllJoynWatcherStoppedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AllJoynWatcherStoppedEventArgs {}
unsafe impl ::core::marker::Sync for AllJoynWatcherStoppedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynAboutData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynAboutData {
    type Vtable = IAllJoynAboutData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5a9bf00_1fa2_4839_93ef_f9df404890f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAboutData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynAboutDataView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynAboutDataView {
    type Vtable = IAllJoynAboutDataView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6823111f_6212_4934_9c48_e19ca4984288);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAboutDataView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynAboutDataViewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynAboutDataViewStatics {
    type Vtable = IAllJoynAboutDataViewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57edb688_0c5e_416e_88b5_39b32d25c47d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAboutDataViewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, busattachment: ::windows::core::RawPtr, sessionport: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Globalization"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, busattachment: ::windows::core::RawPtr, sessionport: u16, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_AllJoyn`*"]
pub struct IAllJoynAcceptSessionJoiner(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynAcceptSessionJoiner {
    type Vtable = IAllJoynAcceptSessionJoiner_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4da817d2_cd1d_4023_a7c4_16def89c28df);
}
impl IAllJoynAcceptSessionJoiner {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IAllJoynAcceptSessionJoiner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4da817d2-cd1d-4023-a7c4-16def89c28df}");
}
impl ::core::convert::From<IAllJoynAcceptSessionJoiner> for ::windows::core::IUnknown {
    fn from(value: IAllJoynAcceptSessionJoiner) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAllJoynAcceptSessionJoiner> for ::windows::core::IUnknown {
    fn from(value: &IAllJoynAcceptSessionJoiner) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAllJoynAcceptSessionJoiner> for ::windows::core::IInspectable {
    fn from(value: IAllJoynAcceptSessionJoiner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAllJoynAcceptSessionJoiner> for ::windows::core::IInspectable {
    fn from(value: &IAllJoynAcceptSessionJoiner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoiner_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoinerEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynAcceptSessionJoinerEventArgs {
    type Vtable = IAllJoynAcceptSessionJoinerEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4efb5365_3e8a_4257_8f10_539ce0d56c0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoinerEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AllJoynTrafficType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoinerEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynAcceptSessionJoinerEventArgsFactory {
    type Vtable = IAllJoynAcceptSessionJoinerEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4435bc0_6145_429e_84db_d5bfe772b14f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoinerEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynAuthenticationCompleteEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynAuthenticationCompleteEventArgs {
    type Vtable = IAllJoynAuthenticationCompleteEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97b4701c_15dc_4b53_b6a4_7d134300d7bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAuthenticationCompleteEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AllJoynAuthenticationMechanism) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusAttachment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusAttachment {
    type Vtable = IAllJoynBusAttachment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf309f153_1eed_42c3_a20e_436d41fe62f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AllJoynBusAttachmentState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusAttachment2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusAttachment2 {
    type Vtable = IAllJoynBusAttachment2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3474cb1e_2368_43b2_b43e_6a3ac1278d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachment2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Globalization"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceinfo: ::windows::core::RawPtr, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusAttachmentFactory {
    type Vtable = IAllJoynBusAttachmentFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x642ef1a4_ad85_4ddf_90ae_604452b22288);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionspecification: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusAttachmentStateChangedEventArgs {
    type Vtable = IAllJoynBusAttachmentStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd82e75f4_c02a_41ec_a8d5_eab1558953aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AllJoynBusAttachmentState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusAttachmentStatics {
    type Vtable = IAllJoynBusAttachmentStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x839d4d3d_1051_40d7_872a_8d0141115b1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requiredinterfaces: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusObject(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusObject {
    type Vtable = IAllJoynBusObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8fd825e_f73a_490c_8804_04e026643047);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, producer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusObjectFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusObjectFactory {
    type Vtable = IAllJoynBusObjectFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c2f9f0b_8e02_4f9c_ac27_ea6dad5d3b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObjectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, objectpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, objectpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, busattachment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusObjectStoppedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusObjectStoppedEventArgs {
    type Vtable = IAllJoynBusObjectStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde102115_ef8e_4d42_b93b_a2ae74519766);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObjectStoppedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynBusObjectStoppedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynBusObjectStoppedEventArgsFactory {
    type Vtable = IAllJoynBusObjectStoppedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b22fd48_d0a3_4255_953a_4772b4028073);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObjectStoppedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynCredentials(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynCredentials {
    type Vtable = IAllJoynCredentials_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x824650f2_a190_40b1_abab_349ec244dfaa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynCredentials_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AllJoynAuthenticationMechanism) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynCredentialsRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynCredentialsRequestedEventArgs {
    type Vtable = IAllJoynCredentialsRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a87e34e_b069_4b80_9e1a_41bc837c65d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynCredentialsRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynCredentialsVerificationRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynCredentialsVerificationRequestedEventArgs {
    type Vtable = IAllJoynCredentialsVerificationRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x800a7612_b805_44af_a2e1_792ab655a2d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynCredentialsVerificationRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AllJoynAuthenticationMechanism) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynMessageInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynMessageInfo {
    type Vtable = IAllJoynMessageInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2b0127_2c12_4859_aa3a_c74461ee814c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynMessageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynMessageInfoFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynMessageInfoFactory {
    type Vtable = IAllJoynMessageInfoFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34664c2a_8289_43d4_b4a8_3f4de359f043);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynMessageInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, senderuniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_AllJoyn`*"]
pub struct IAllJoynProducer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynProducer {
    type Vtable = IAllJoynProducer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d084679_469b_495a_a710_ac50f123069f);
}
impl IAllJoynProducer {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_AllJoyn`*"]
    pub fn SetBusObject<'a, Param0: ::windows::core::IntoParam<'a, AllJoynBusObject>>(&self, busobject: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), busobject.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IAllJoynProducer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9d084679-469b-495a-a710-ac50f123069f}");
}
impl ::core::convert::From<IAllJoynProducer> for ::windows::core::IUnknown {
    fn from(value: IAllJoynProducer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAllJoynProducer> for ::windows::core::IUnknown {
    fn from(value: &IAllJoynProducer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAllJoynProducer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAllJoynProducer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAllJoynProducer> for ::windows::core::IInspectable {
    fn from(value: IAllJoynProducer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAllJoynProducer> for ::windows::core::IInspectable {
    fn from(value: &IAllJoynProducer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAllJoynProducer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAllJoynProducer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynProducer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, busobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynProducerStoppedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynProducerStoppedEventArgs {
    type Vtable = IAllJoynProducerStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51309770_4937_492d_8080_236439987ceb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynProducerStoppedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynProducerStoppedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynProducerStoppedEventArgsFactory {
    type Vtable = IAllJoynProducerStoppedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56529961_b219_4d6e_9f78_fa3f99fa8fe5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynProducerStoppedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynServiceInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynServiceInfo {
    type Vtable = IAllJoynServiceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cbe8209_b93e_4182_999b_ddd000f9c575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynServiceInfoFactory {
    type Vtable = IAllJoynServiceInfoFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7581dabd_fe03_4f4b_94a4_f02fdcbd11b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, objectpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionport: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoRemovedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynServiceInfoRemovedEventArgs {
    type Vtable = IAllJoynServiceInfoRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3057a95f_1d3f_41f3_8969_e32792627396);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoRemovedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynServiceInfoRemovedEventArgsFactory {
    type Vtable = IAllJoynServiceInfoRemovedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dbf8627_9aff_4955_9227_6953baf41569);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoRemovedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynServiceInfoStatics {
    type Vtable = IAllJoynServiceInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5678570a_603a_49fc_b750_0ef13609213c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSession {
    type Vtable = IAllJoynSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d11b0c_c0d4_406c_88a9_a93efa85d4b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionJoinedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionJoinedEventArgs {
    type Vtable = IAllJoynSessionJoinedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e9f5bd0_b5d7_47c5_8dab_b040cc192871);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionJoinedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionJoinedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionJoinedEventArgsFactory {
    type Vtable = IAllJoynSessionJoinedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6824d689_d6cb_4d9e_a09e_35806870b17f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionJoinedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionLostEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionLostEventArgs {
    type Vtable = IAllJoynSessionLostEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe766a48a_8bb8_4954_ae67_d2fa43d1f96b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionLostEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AllJoynSessionLostReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionLostEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionLostEventArgsFactory {
    type Vtable = IAllJoynSessionLostEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13bbfd32_d2f4_49c9_980e_2805e13586b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionLostEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: AllJoynSessionLostReason, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberAddedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionMemberAddedEventArgs {
    type Vtable = IAllJoynSessionMemberAddedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49a2798a_0dd1_46c1_9cd6_27190e503a5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberAddedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberAddedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionMemberAddedEventArgsFactory {
    type Vtable = IAllJoynSessionMemberAddedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x341de352_1d33_40a1_a1d3_e5777020e1f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberAddedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberRemovedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionMemberRemovedEventArgs {
    type Vtable = IAllJoynSessionMemberRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x409a219f_aa4a_4893_b430_baa1b63c6219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberRemovedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionMemberRemovedEventArgsFactory {
    type Vtable = IAllJoynSessionMemberRemovedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d355e8_42b8_4b67_b757_d0cfcad59280);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberRemovedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynSessionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynSessionStatics {
    type Vtable = IAllJoynSessionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e05d604_a06c_46d4_b46c_0b0b54105b44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceinfo: ::windows::core::RawPtr, busattachment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynStatusStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynStatusStatics {
    type Vtable = IAllJoynStatusStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0b7a17e_0d29_4da9_8ac6_54c554bedbc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynStatusStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynWatcherStoppedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynWatcherStoppedEventArgs {
    type Vtable = IAllJoynWatcherStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9fca03b_701d_4aa8_97dd_a2bb0a8f5fa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynWatcherStoppedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAllJoynWatcherStoppedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAllJoynWatcherStoppedEventArgsFactory {
    type Vtable = IAllJoynWatcherStoppedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x878fa5a8_2d50_47e1_904a_20bf0d48c782);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynWatcherStoppedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
