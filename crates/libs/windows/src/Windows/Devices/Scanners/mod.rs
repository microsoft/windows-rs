#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScanner(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScanner {
    type Vtable = IImageScanner_Vtbl;
}
impl ::core::clone::Clone for IImageScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageScanner {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53a88f78_5298_48a0_8da3_8087519665e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScanner_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultScanSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerScanSource) -> ::windows_core::HRESULT,
    pub IsScanSourceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerScanSource, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FlatbedConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FeederConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AutoConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsPreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ScanPreviewToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, targetstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ScanPreviewToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ScanFilesToFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, storagefolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ScanFilesToFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerFeederConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerFeederConfiguration {
    type Vtable = IImageScannerFeederConfiguration_Vtbl;
}
impl ::core::clone::Clone for IImageScannerFeederConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageScannerFeederConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74bdacee_fa97_4c17_8280_40e39c6dcc67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerFeederConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanAutoDetectPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AutoDetectPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoDetectPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Printing::PrintMediaSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub PageOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Printing::PrintOrientation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageOrientation: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageOrientation: usize,
    #[cfg(feature = "Foundation")]
    pub PageSizeDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSizeDimensions: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub IsPageSizeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    IsPageSizeSupported: usize,
    pub MaxNumberOfPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxNumberOfPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CanScanDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Duplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanScanAhead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ScanAhead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetScanAhead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct IImageScannerFormatConfiguration(::windows_core::IUnknown);
impl IImageScannerFormatConfiguration {
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IImageScannerFormatConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IImageScannerFormatConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageScannerFormatConfiguration {}
impl ::core::fmt::Debug for IImageScannerFormatConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageScannerFormatConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IImageScannerFormatConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ae275d11-dadf-4010-bf10-cca5c83dcbb0}");
}
unsafe impl ::windows_core::Interface for IImageScannerFormatConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
}
impl ::core::clone::Clone for IImageScannerFormatConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageScannerFormatConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae275d11_dadf_4010_bf10_cca5c83dcbb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerFormatConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DefaultFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerFormat) -> ::windows_core::HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerFormat, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerPreviewResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerPreviewResult {
    type Vtable = IImageScannerPreviewResult_Vtbl;
}
impl ::core::clone::Clone for IImageScannerPreviewResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageScannerPreviewResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08b7fe8e_8891_441d_be9c_176fa109c8bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerPreviewResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerScanResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerScanResult {
    type Vtable = IImageScannerScanResult_Vtbl;
}
impl ::core::clone::Clone for IImageScannerScanResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageScannerScanResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc91624cd_9037_4e48_84c1_ac0975076bc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerScanResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ScannedFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ScannedFiles: usize,
}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct IImageScannerSourceConfiguration(::windows_core::IUnknown);
impl IImageScannerSourceConfiguration {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinScanArea(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinScanArea)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxScanArea(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxScanArea)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedScanRegion(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedScanRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSelectedScanRegion(&self, value: super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedScanRegion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoCroppingMode(&self) -> ::windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoCroppingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoCroppingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn MinResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OpticalResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpticalResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DesiredResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredResolution(&self, value: ImageScannerResolution) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredResolution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultColorMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsColorModeSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn MinBrightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBrightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BrightnessStep(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultBrightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxContrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContrastStep(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContrastStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultContrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContrast)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IImageScannerSourceConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IImageScannerFormatConfiguration> for IImageScannerSourceConfiguration {}
impl ::core::cmp::PartialEq for IImageScannerSourceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageScannerSourceConfiguration {}
impl ::core::fmt::Debug for IImageScannerSourceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageScannerSourceConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IImageScannerSourceConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{bfb50055-0b44-4c82-9e89-205f9c234e59}");
}
unsafe impl ::windows_core::Interface for IImageScannerSourceConfiguration {
    type Vtable = IImageScannerSourceConfiguration_Vtbl;
}
impl ::core::clone::Clone for IImageScannerSourceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageScannerSourceConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfb50055_0b44_4c82_9e89_205f9c234e59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerSourceConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MinScanArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinScanArea: usize,
    #[cfg(feature = "Foundation")]
    pub MaxScanArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxScanArea: usize,
    #[cfg(feature = "Foundation")]
    pub SelectedScanRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedScanRegion: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectedScanRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectedScanRegion: usize,
    pub AutoCroppingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerAutoCroppingMode) -> ::windows_core::HRESULT,
    pub SetAutoCroppingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode) -> ::windows_core::HRESULT,
    pub IsAutoCroppingModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MinResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub OpticalResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub DesiredResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub SetDesiredResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerResolution) -> ::windows_core::HRESULT,
    pub ActualResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub DefaultColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows_core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows_core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode) -> ::windows_core::HRESULT,
    pub IsColorModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MinBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub BrightnessStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DefaultBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Brightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub MinContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ContrastStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DefaultContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Contrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerStatics {
    type Vtable = IImageScannerStatics_Vtbl;
}
impl ::core::clone::Clone for IImageScannerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageScannerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc57e70e_d804_4477_9fb5_b911b5473897);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScanner(::windows_core::IUnknown);
impl ImageScanner {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultScanSource(&self) -> ::windows_core::Result<ImageScannerScanSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultScanSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsScanSourceSupported(&self, value: ImageScannerScanSource) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsScanSourceSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FlatbedConfiguration(&self) -> ::windows_core::Result<ImageScannerFlatbedConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlatbedConfiguration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FeederConfiguration(&self) -> ::windows_core::Result<ImageScannerFeederConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FeederConfiguration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AutoConfiguration(&self) -> ::windows_core::Result<ImageScannerAutoConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoConfiguration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPreviewSupported(&self, scansource: ImageScannerScanSource) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPreviewSupported)(::windows_core::Interface::as_raw(this), scansource, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ScanPreviewToStreamAsync<P0>(&self, scansource: ImageScannerScanSource, targetstream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageScannerPreviewResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScanPreviewToStreamAsync)(::windows_core::Interface::as_raw(this), scansource, targetstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ScanFilesToFolderAsync<P0>(&self, scansource: ImageScannerScanSource, storagefolder: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScanFilesToFolderAsync)(::windows_core::Interface::as_raw(this), scansource, storagefolder.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageScanner>> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IImageScannerStatics<R, F: FnOnce(&IImageScannerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageScanner, IImageScannerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ImageScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScanner {}
impl ::core::fmt::Debug for ImageScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScanner").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScanner {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScanner;{53a88f78-5298-48a0-8da3-8087519665e0})");
}
impl ::core::clone::Clone for ImageScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageScanner {
    type Vtable = IImageScanner_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageScanner {
    const IID: ::windows_core::GUID = <IImageScanner as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageScanner {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScanner";
}
::windows_core::imp::interface_hierarchy!(ImageScanner, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ImageScanner {}
unsafe impl ::core::marker::Sync for ImageScanner {}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerAutoConfiguration(::windows_core::IUnknown);
impl ImageScannerAutoConfiguration {
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ImageScannerAutoConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerAutoConfiguration {}
impl ::core::fmt::Debug for ImageScannerAutoConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerAutoConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerAutoConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerAutoConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
}
impl ::core::clone::Clone for ImageScannerAutoConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageScannerAutoConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageScannerAutoConfiguration {
    const IID: ::windows_core::GUID = <IImageScannerFormatConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerAutoConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerAutoConfiguration";
}
::windows_core::imp::interface_hierarchy!(ImageScannerAutoConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IImageScannerFormatConfiguration> for ImageScannerAutoConfiguration {}
unsafe impl ::core::marker::Send for ImageScannerAutoConfiguration {}
unsafe impl ::core::marker::Sync for ImageScannerAutoConfiguration {}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerFeederConfiguration(::windows_core::IUnknown);
impl ImageScannerFeederConfiguration {
    pub fn CanAutoDetectPageSize(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanAutoDetectPageSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AutoDetectPageSize(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoDetectPageSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoDetectPageSize(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoDetectPageSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn PageSize(&self) -> ::windows_core::Result<super::super::Graphics::Printing::PrintMediaSize> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn SetPageSize(&self, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPageSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn PageOrientation(&self) -> ::windows_core::Result<super::super::Graphics::Printing::PrintOrientation> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageOrientation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn SetPageOrientation(&self, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPageOrientation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PageSizeDimensions(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageSizeDimensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn IsPageSizeSupported(&self, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPageSizeSupported)(::windows_core::Interface::as_raw(this), pagesize, pageorientation, &mut result__).from_abi(result__)
        }
    }
    pub fn MaxNumberOfPages(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxNumberOfPages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxNumberOfPages(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxNumberOfPages)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanScanDuplex(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanScanDuplex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Duplex(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duplex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDuplex(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuplex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanScanAhead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanScanAhead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ScanAhead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScanAhead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScanAhead(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetScanAhead)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinScanArea(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinScanArea)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxScanArea(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxScanArea)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedScanRegion(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedScanRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSelectedScanRegion(&self, value: super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedScanRegion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoCroppingMode(&self) -> ::windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoCroppingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoCroppingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn MinResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OpticalResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpticalResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DesiredResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredResolution(&self, value: ImageScannerResolution) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredResolution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultColorMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetColorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsColorModeSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn MinBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BrightnessStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContrastStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContrastStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetContrast)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ImageScannerFeederConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerFeederConfiguration {}
impl ::core::fmt::Debug for ImageScannerFeederConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerFeederConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerFeederConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerFeederConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
}
impl ::core::clone::Clone for ImageScannerFeederConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageScannerFeederConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageScannerFeederConfiguration {
    const IID: ::windows_core::GUID = <IImageScannerFormatConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerFeederConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFeederConfiguration";
}
::windows_core::imp::interface_hierarchy!(ImageScannerFeederConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IImageScannerFormatConfiguration> for ImageScannerFeederConfiguration {}
impl ::windows_core::CanTryInto<IImageScannerSourceConfiguration> for ImageScannerFeederConfiguration {}
unsafe impl ::core::marker::Send for ImageScannerFeederConfiguration {}
unsafe impl ::core::marker::Sync for ImageScannerFeederConfiguration {}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerFlatbedConfiguration(::windows_core::IUnknown);
impl ImageScannerFlatbedConfiguration {
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinScanArea(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinScanArea)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxScanArea(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxScanArea)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedScanRegion(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedScanRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSelectedScanRegion(&self, value: super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedScanRegion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoCroppingMode(&self) -> ::windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoCroppingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoCroppingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn MinResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OpticalResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpticalResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DesiredResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredResolution(&self, value: ImageScannerResolution) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredResolution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultColorMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetColorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsColorModeSupported)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn MinBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BrightnessStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBrightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContrastStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContrastStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetContrast)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ImageScannerFlatbedConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerFlatbedConfiguration {}
impl ::core::fmt::Debug for ImageScannerFlatbedConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerFlatbedConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerFlatbedConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerFlatbedConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
}
impl ::core::clone::Clone for ImageScannerFlatbedConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageScannerFlatbedConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageScannerFlatbedConfiguration {
    const IID: ::windows_core::GUID = <IImageScannerFormatConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerFlatbedConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFlatbedConfiguration";
}
::windows_core::imp::interface_hierarchy!(ImageScannerFlatbedConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IImageScannerFormatConfiguration> for ImageScannerFlatbedConfiguration {}
impl ::windows_core::CanTryInto<IImageScannerSourceConfiguration> for ImageScannerFlatbedConfiguration {}
unsafe impl ::core::marker::Send for ImageScannerFlatbedConfiguration {}
unsafe impl ::core::marker::Sync for ImageScannerFlatbedConfiguration {}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerPreviewResult(::windows_core::IUnknown);
impl ImageScannerPreviewResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ImageScannerPreviewResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerPreviewResult {}
impl ::core::fmt::Debug for ImageScannerPreviewResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerPreviewResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerPreviewResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerPreviewResult;{08b7fe8e-8891-441d-be9c-176fa109c8bb})");
}
impl ::core::clone::Clone for ImageScannerPreviewResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageScannerPreviewResult {
    type Vtable = IImageScannerPreviewResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageScannerPreviewResult {
    const IID: ::windows_core::GUID = <IImageScannerPreviewResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerPreviewResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerPreviewResult";
}
::windows_core::imp::interface_hierarchy!(ImageScannerPreviewResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ImageScannerPreviewResult {}
unsafe impl ::core::marker::Sync for ImageScannerPreviewResult {}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerScanResult(::windows_core::IUnknown);
impl ImageScannerScanResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ScannedFiles(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScannedFiles)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ImageScannerScanResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerScanResult {}
impl ::core::fmt::Debug for ImageScannerScanResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerScanResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerScanResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerScanResult;{c91624cd-9037-4e48-84c1-ac0975076bc5})");
}
impl ::core::clone::Clone for ImageScannerScanResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageScannerScanResult {
    type Vtable = IImageScannerScanResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageScannerScanResult {
    const IID: ::windows_core::GUID = <IImageScannerScanResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerScanResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerScanResult";
}
::windows_core::imp::interface_hierarchy!(ImageScannerScanResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ImageScannerScanResult {}
unsafe impl ::core::marker::Sync for ImageScannerScanResult {}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ImageScannerAutoCroppingMode(pub i32);
impl ImageScannerAutoCroppingMode {
    pub const Disabled: Self = Self(0i32);
    pub const SingleRegion: Self = Self(1i32);
    pub const MultipleRegion: Self = Self(2i32);
}
impl ::core::marker::Copy for ImageScannerAutoCroppingMode {}
impl ::core::clone::Clone for ImageScannerAutoCroppingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerAutoCroppingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ImageScannerAutoCroppingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ImageScannerAutoCroppingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerAutoCroppingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerAutoCroppingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerAutoCroppingMode;i4)");
}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ImageScannerColorMode(pub i32);
impl ImageScannerColorMode {
    pub const Color: Self = Self(0i32);
    pub const Grayscale: Self = Self(1i32);
    pub const Monochrome: Self = Self(2i32);
    pub const AutoColor: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerColorMode {}
impl ::core::clone::Clone for ImageScannerColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerColorMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ImageScannerColorMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ImageScannerColorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerColorMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerColorMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerColorMode;i4)");
}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ImageScannerFormat(pub i32);
impl ImageScannerFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const DeviceIndependentBitmap: Self = Self(2i32);
    pub const Tiff: Self = Self(3i32);
    pub const Xps: Self = Self(4i32);
    pub const OpenXps: Self = Self(5i32);
    pub const Pdf: Self = Self(6i32);
}
impl ::core::marker::Copy for ImageScannerFormat {}
impl ::core::clone::Clone for ImageScannerFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ImageScannerFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ImageScannerFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerFormat;i4)");
}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ImageScannerScanSource(pub i32);
impl ImageScannerScanSource {
    pub const Default: Self = Self(0i32);
    pub const Flatbed: Self = Self(1i32);
    pub const Feeder: Self = Self(2i32);
    pub const AutoConfigured: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerScanSource {}
impl ::core::clone::Clone for ImageScannerScanSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerScanSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ImageScannerScanSource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ImageScannerScanSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerScanSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageScannerScanSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerScanSource;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
pub struct ImageScannerResolution {
    pub DpiX: f32,
    pub DpiY: f32,
}
impl ::core::marker::Copy for ImageScannerResolution {}
impl ::core::clone::Clone for ImageScannerResolution {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ImageScannerResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ImageScannerResolution").field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).finish()
    }
}
impl ::windows_core::TypeKind for ImageScannerResolution {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for ImageScannerResolution {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Scanners.ImageScannerResolution;f4;f4)");
}
impl ::core::cmp::PartialEq for ImageScannerResolution {
    fn eq(&self, other: &Self) -> bool {
        self.DpiX == other.DpiX && self.DpiY == other.DpiY
    }
}
impl ::core::cmp::Eq for ImageScannerResolution {}
impl ::core::default::Default for ImageScannerResolution {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
