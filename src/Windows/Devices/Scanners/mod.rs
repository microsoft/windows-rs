#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageScanner(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageScanner {
    type Vtable = IImageScanner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1403555704, 21144, 18592, [141, 163, 128, 135, 81, 150, 101, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScanner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerScanSource) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerScanSource, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scansource: ImageScannerScanSource, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scansource: ImageScannerScanSource, targetstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scansource: ImageScannerScanSource, storagefolder: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageScannerFeederConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageScannerFeederConfiguration {
    type Vtable = IImageScannerFeederConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1958587630, 64151, 19479, [130, 128, 64, 227, 156, 109, 204, 103]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerFeederConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Printing::PrintMediaSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))] usize,
    #[cfg(feature = "Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))] usize,
    #[cfg(feature = "Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Printing::PrintOrientation) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))] usize,
    #[cfg(feature = "Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_Scanners`*"]
pub struct IImageScannerFormatConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageScannerFormatConfiguration {
    type Vtable = IImageScannerFormatConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2921815313, 56031, 16400, [191, 16, 204, 165, 200, 61, 203, 176]);
}
impl IImageScannerFormatConfiguration {
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultFormat(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IImageScannerFormatConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ae275d11-dadf-4010-bf10-cca5c83dcbb0}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerFormatConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerFormat, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageScannerPreviewResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageScannerPreviewResult {
    type Vtable = IImageScannerPreviewResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(146275982, 34961, 17437, [190, 156, 23, 111, 161, 9, 200, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerPreviewResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerFormat) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageScannerScanResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageScannerScanResult {
    type Vtable = IImageScannerScanResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3373671629, 36919, 20040, [132, 193, 172, 9, 117, 7, 107, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerScanResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_Scanners`*"]
pub struct IImageScannerSourceConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageScannerSourceConfiguration {
    type Vtable = IImageScannerSourceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3216310357, 2884, 19586, [158, 137, 32, 95, 156, 35, 78, 89]);
}
impl IImageScannerSourceConfiguration {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn MinScanArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn MaxScanArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn SelectedScanRegion(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn SetSelectedScanRegion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn AutoCroppingMode(&self) -> ::windows::runtime::Result<ImageScannerAutoCroppingMode> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerAutoCroppingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerAutoCroppingMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn OpticalResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DesiredResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetDesiredResolution<'a, Param0: ::windows::runtime::IntoParam<'a, ImageScannerResolution>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ActualResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultColorMode(&self) -> ::windows::runtime::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerColorMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ColorMode(&self) -> ::windows::runtime::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerColorMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn BrightnessStep(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Brightness(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetBrightness(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ContrastStep(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Contrast(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetContrast(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultFormat(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IImageScannerSourceConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{bfb50055-0b44-4c82-9e89-205f9c234e59}");
}
impl ::std::convert::TryFrom<IImageScannerSourceConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IImageScannerSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IImageScannerSourceConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IImageScannerSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for &IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::std::convert::TryInto::<IImageScannerFormatConfiguration>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerSourceConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerAutoCroppingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerAutoCroppingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerColorMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ImageScannerColorMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerColorMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ImageScannerColorMode, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageScannerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageScannerStatics {
    type Vtable = IImageScannerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3159877390, 55300, 17527, [159, 181, 185, 17, 181, 71, 56, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_Scanners`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ImageScanner(::windows::runtime::IInspectable);
impl ImageScanner {
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultScanSource(&self) -> ::windows::runtime::Result<ImageScannerScanSource> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerScanSource = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerScanSource>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsScanSourceSupported(&self, value: ImageScannerScanSource) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn FlatbedConfiguration(&self) -> ::windows::runtime::Result<ImageScannerFlatbedConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFlatbedConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn FeederConfiguration(&self) -> ::windows::runtime::Result<ImageScannerFeederConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFeederConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn AutoConfiguration(&self) -> ::windows::runtime::Result<ImageScannerAutoConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerAutoConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsPreviewSupported(&self, scansource: ImageScannerScanSource) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), scansource, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`, `Storage_Streams`*"]
    pub fn ScanPreviewToStreamAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, scansource: ImageScannerScanSource, targetstream: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ImageScannerPreviewResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), scansource, targetstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageScannerPreviewResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`, `Storage`*"]
    pub fn ScanFilesToFolderAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFolder>>(&self, scansource: ImageScannerScanSource, storagefolder: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), scansource, storagefolder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ImageScanner>> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageScanner>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IImageScannerStatics<R, F: FnOnce(&IImageScannerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ImageScanner, IImageScannerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageScanner {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScanner;{53a88f78-5298-48a0-8da3-8087519665e0})");
}
unsafe impl ::windows::runtime::Interface for ImageScanner {
    type Vtable = IImageScanner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1403555704, 21144, 18592, [141, 163, 128, 135, 81, 150, 101, 224]);
}
impl ::windows::runtime::RuntimeName for ImageScanner {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScanner";
}
unsafe impl ::std::marker::Send for ImageScanner {}
unsafe impl ::std::marker::Sync for ImageScanner {}
#[doc = "*Required features: `Devices_Scanners`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ImageScannerAutoConfiguration(::windows::runtime::IInspectable);
impl ImageScannerAutoConfiguration {
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultFormat(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerAutoConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerAutoConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
}
unsafe impl ::windows::runtime::Interface for ImageScannerAutoConfiguration {
    type Vtable = IImageScannerFormatConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2921815313, 56031, 16400, [191, 16, 204, 165, 200, 61, 203, 176]);
}
impl ::windows::runtime::RuntimeName for ImageScannerAutoConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerAutoConfiguration";
}
impl ::std::convert::From<ImageScannerAutoConfiguration> for IImageScannerFormatConfiguration {
    fn from(value: ImageScannerAutoConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ImageScannerAutoConfiguration> for IImageScannerFormatConfiguration {
    fn from(value: &ImageScannerAutoConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IImageScannerFormatConfiguration>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for &ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IImageScannerFormatConfiguration>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for ImageScannerAutoConfiguration {}
unsafe impl ::std::marker::Sync for ImageScannerAutoConfiguration {}
#[doc = "*Required features: `Devices_Scanners`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ImageScannerAutoCroppingMode(pub i32);
impl ImageScannerAutoCroppingMode {
    pub const Disabled: ImageScannerAutoCroppingMode = ImageScannerAutoCroppingMode(0i32);
    pub const SingleRegion: ImageScannerAutoCroppingMode = ImageScannerAutoCroppingMode(1i32);
    pub const MultipleRegion: ImageScannerAutoCroppingMode = ImageScannerAutoCroppingMode(2i32);
}
impl ::std::convert::From<i32> for ImageScannerAutoCroppingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ImageScannerAutoCroppingMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerAutoCroppingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerAutoCroppingMode;i4)");
}
#[doc = "*Required features: `Devices_Scanners`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ImageScannerColorMode(pub i32);
impl ImageScannerColorMode {
    pub const Color: ImageScannerColorMode = ImageScannerColorMode(0i32);
    pub const Grayscale: ImageScannerColorMode = ImageScannerColorMode(1i32);
    pub const Monochrome: ImageScannerColorMode = ImageScannerColorMode(2i32);
    pub const AutoColor: ImageScannerColorMode = ImageScannerColorMode(3i32);
}
impl ::std::convert::From<i32> for ImageScannerColorMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ImageScannerColorMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerColorMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerColorMode;i4)");
}
#[doc = "*Required features: `Devices_Scanners`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ImageScannerFeederConfiguration(::windows::runtime::IInspectable);
impl ImageScannerFeederConfiguration {
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultFormat(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn CanAutoDetectPageSize(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn AutoDetectPageSize(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetAutoDetectPageSize(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Printing")]
    #[doc = "*Required features: `Devices_Scanners`, `Graphics_Printing`*"]
    pub fn PageSize(&self) -> ::windows::runtime::Result<super::super::Graphics::Printing::PrintMediaSize> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Printing::PrintMediaSize = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Printing::PrintMediaSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    #[doc = "*Required features: `Devices_Scanners`, `Graphics_Printing`*"]
    pub fn SetPageSize(&self, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Printing")]
    #[doc = "*Required features: `Devices_Scanners`, `Graphics_Printing`*"]
    pub fn PageOrientation(&self) -> ::windows::runtime::Result<super::super::Graphics::Printing::PrintOrientation> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Graphics::Printing::PrintOrientation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Printing::PrintOrientation>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    #[doc = "*Required features: `Devices_Scanners`, `Graphics_Printing`*"]
    pub fn SetPageOrientation(&self, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn PageSizeDimensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    #[doc = "*Required features: `Devices_Scanners`, `Graphics_Printing`*"]
    pub fn IsPageSizeSupported(&self, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), pagesize, pageorientation, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxNumberOfPages(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetMaxNumberOfPages(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn CanScanDuplex(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Duplex(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetDuplex(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn CanScanAhead(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ScanAhead(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetScanAhead(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn MinScanArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn MaxScanArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn SelectedScanRegion(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn SetSelectedScanRegion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn AutoCroppingMode(&self) -> ::windows::runtime::Result<ImageScannerAutoCroppingMode> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerAutoCroppingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerAutoCroppingMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn OpticalResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DesiredResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetDesiredResolution<'a, Param0: ::windows::runtime::IntoParam<'a, ImageScannerResolution>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ActualResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultColorMode(&self) -> ::windows::runtime::Result<ImageScannerColorMode> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerColorMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ColorMode(&self) -> ::windows::runtime::Result<ImageScannerColorMode> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerColorMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn BrightnessStep(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Brightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetBrightness(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ContrastStep(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Contrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetContrast(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerFeederConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerFeederConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
}
unsafe impl ::windows::runtime::Interface for ImageScannerFeederConfiguration {
    type Vtable = IImageScannerFormatConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2921815313, 56031, 16400, [191, 16, 204, 165, 200, 61, 203, 176]);
}
impl ::windows::runtime::RuntimeName for ImageScannerFeederConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFeederConfiguration";
}
impl ::std::convert::From<ImageScannerFeederConfiguration> for IImageScannerFormatConfiguration {
    fn from(value: ImageScannerFeederConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ImageScannerFeederConfiguration> for IImageScannerFormatConfiguration {
    fn from(value: &ImageScannerFeederConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IImageScannerFormatConfiguration>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for &ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IImageScannerFormatConfiguration>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<ImageScannerFeederConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ImageScannerFeederConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ImageScannerFeederConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ImageScannerFeederConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerSourceConfiguration> for ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerSourceConfiguration> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerSourceConfiguration> for &ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerSourceConfiguration> {
        ::std::convert::TryInto::<IImageScannerSourceConfiguration>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ImageScannerFeederConfiguration {}
unsafe impl ::std::marker::Sync for ImageScannerFeederConfiguration {}
#[doc = "*Required features: `Devices_Scanners`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ImageScannerFlatbedConfiguration(::windows::runtime::IInspectable);
impl ImageScannerFlatbedConfiguration {
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultFormat(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn MinScanArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn MaxScanArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn SelectedScanRegion(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation`*"]
    pub fn SetSelectedScanRegion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn AutoCroppingMode(&self) -> ::windows::runtime::Result<ImageScannerAutoCroppingMode> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerAutoCroppingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerAutoCroppingMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn OpticalResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DesiredResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetDesiredResolution<'a, Param0: ::windows::runtime::IntoParam<'a, ImageScannerResolution>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ActualResolution(&self) -> ::windows::runtime::Result<ImageScannerResolution> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerResolution>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultColorMode(&self) -> ::windows::runtime::Result<ImageScannerColorMode> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerColorMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ColorMode(&self) -> ::windows::runtime::Result<ImageScannerColorMode> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: ImageScannerColorMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn BrightnessStep(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultBrightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Brightness(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetBrightness(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MinContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn MaxContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn ContrastStep(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn DefaultContrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Contrast(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn SetContrast(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerFlatbedConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerFlatbedConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
}
unsafe impl ::windows::runtime::Interface for ImageScannerFlatbedConfiguration {
    type Vtable = IImageScannerFormatConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2921815313, 56031, 16400, [191, 16, 204, 165, 200, 61, 203, 176]);
}
impl ::windows::runtime::RuntimeName for ImageScannerFlatbedConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFlatbedConfiguration";
}
impl ::std::convert::From<ImageScannerFlatbedConfiguration> for IImageScannerFormatConfiguration {
    fn from(value: ImageScannerFlatbedConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ImageScannerFlatbedConfiguration> for IImageScannerFormatConfiguration {
    fn from(value: &ImageScannerFlatbedConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IImageScannerFormatConfiguration>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerFormatConfiguration> for &ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerFormatConfiguration> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IImageScannerFormatConfiguration>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<ImageScannerFlatbedConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ImageScannerFlatbedConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ImageScannerFlatbedConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ImageScannerFlatbedConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerSourceConfiguration> for ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerSourceConfiguration> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageScannerSourceConfiguration> for &ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageScannerSourceConfiguration> {
        ::std::convert::TryInto::<IImageScannerSourceConfiguration>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ImageScannerFlatbedConfiguration {}
unsafe impl ::std::marker::Sync for ImageScannerFlatbedConfiguration {}
#[doc = "*Required features: `Devices_Scanners`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ImageScannerFormat(pub i32);
impl ImageScannerFormat {
    pub const Jpeg: ImageScannerFormat = ImageScannerFormat(0i32);
    pub const Png: ImageScannerFormat = ImageScannerFormat(1i32);
    pub const DeviceIndependentBitmap: ImageScannerFormat = ImageScannerFormat(2i32);
    pub const Tiff: ImageScannerFormat = ImageScannerFormat(3i32);
    pub const Xps: ImageScannerFormat = ImageScannerFormat(4i32);
    pub const OpenXps: ImageScannerFormat = ImageScannerFormat(5i32);
    pub const Pdf: ImageScannerFormat = ImageScannerFormat(6i32);
}
impl ::std::convert::From<i32> for ImageScannerFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ImageScannerFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerFormat;i4)");
}
#[doc = "*Required features: `Devices_Scanners`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ImageScannerPreviewResult(::windows::runtime::IInspectable);
impl ImageScannerPreviewResult {
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Scanners`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__: ImageScannerFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageScannerFormat>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerPreviewResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerPreviewResult;{08b7fe8e-8891-441d-be9c-176fa109c8bb})");
}
unsafe impl ::windows::runtime::Interface for ImageScannerPreviewResult {
    type Vtable = IImageScannerPreviewResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(146275982, 34961, 17437, [190, 156, 23, 111, 161, 9, 200, 187]);
}
impl ::windows::runtime::RuntimeName for ImageScannerPreviewResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerPreviewResult";
}
unsafe impl ::std::marker::Send for ImageScannerPreviewResult {}
unsafe impl ::std::marker::Sync for ImageScannerPreviewResult {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Devices_Scanners`*"]
pub struct ImageScannerResolution {
    pub DpiX: f32,
    pub DpiY: f32,
}
impl ImageScannerResolution {}
impl ::std::default::Default for ImageScannerResolution {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ImageScannerResolution {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ImageScannerResolution").field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).finish()
    }
}
impl ::std::cmp::PartialEq for ImageScannerResolution {
    fn eq(&self, other: &Self) -> bool {
        self.DpiX == other.DpiX && self.DpiY == other.DpiY
    }
}
impl ::std::cmp::Eq for ImageScannerResolution {}
unsafe impl ::windows::runtime::Abi for ImageScannerResolution {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerResolution {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Devices.Scanners.ImageScannerResolution;f4;f4)");
}
#[doc = "*Required features: `Devices_Scanners`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ImageScannerScanResult(::windows::runtime::IInspectable);
impl ImageScannerScanResult {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Devices_Scanners`, `Foundation_Collections`, `Storage`*"]
    pub fn ScannedFiles(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerScanResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerScanResult;{c91624cd-9037-4e48-84c1-ac0975076bc5})");
}
unsafe impl ::windows::runtime::Interface for ImageScannerScanResult {
    type Vtable = IImageScannerScanResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3373671629, 36919, 20040, [132, 193, 172, 9, 117, 7, 107, 197]);
}
impl ::windows::runtime::RuntimeName for ImageScannerScanResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerScanResult";
}
unsafe impl ::std::marker::Send for ImageScannerScanResult {}
unsafe impl ::std::marker::Sync for ImageScannerScanResult {}
#[doc = "*Required features: `Devices_Scanners`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ImageScannerScanSource(pub i32);
impl ImageScannerScanSource {
    pub const Default: ImageScannerScanSource = ImageScannerScanSource(0i32);
    pub const Flatbed: ImageScannerScanSource = ImageScannerScanSource(1i32);
    pub const Feeder: ImageScannerScanSource = ImageScannerScanSource(2i32);
    pub const AutoConfigured: ImageScannerScanSource = ImageScannerScanSource(3i32);
}
impl ::std::convert::From<i32> for ImageScannerScanSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ImageScannerScanSource {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ImageScannerScanSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerScanSource;i4)");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct ScannerDeviceContract(pub u8);
