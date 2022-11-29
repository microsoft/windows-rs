#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IHdmiDisplayInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x130b3c0a_f565_476e_abd5_ea05aee74c69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedDisplayModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedDisplayModes: usize,
    pub GetCurrentDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultDisplayModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultDisplayModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeWithHdrAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut ::core::ffi::c_void, hdroption: HdmiDisplayHdrOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeWithHdrAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut ::core::ffi::c_void, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisplayModesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayModesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayModesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayModesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHdmiDisplayInformationStatics {
    type Vtable = IHdmiDisplayInformationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHdmiDisplayInformationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ce6b260_f42a_4a15_914c_7b8e2a5a65df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayMode(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_Vtbl;
}
unsafe impl ::windows::core::Interface for IHdmiDisplayMode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c06d5ad_1b90_4f51_9981_ef5a1c0ddf66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResolutionWidthInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ResolutionHeightInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub StereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayColorSpace) -> ::windows::core::HRESULT,
    pub PixelEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayPixelEncoding) -> ::windows::core::HRESULT,
    pub IsSdrLuminanceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSmpte2084Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Is2086MetadataSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayMode2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHdmiDisplayMode2 {
    type Vtable = IHdmiDisplayMode2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHdmiDisplayMode2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07cd4e9f_4b3c_42b8_84e7_895368718af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsDolbyVisionLowLatencySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
pub struct HdmiDisplayInformation(::windows::core::IUnknown);
impl HdmiDisplayInformation {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedDisplayModes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HdmiDisplayMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSupportedDisplayModes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetCurrentDisplayMode(&self) -> ::windows::core::Result<HdmiDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentDisplayMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultDisplayModeAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDefaultDisplayModeAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetCurrentDisplayModeAsync(&self, mode: &HdmiDisplayMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestSetCurrentDisplayModeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mode), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetCurrentDisplayModeWithHdrAsync(&self, mode: &HdmiDisplayMode, hdroption: HdmiDisplayHdrOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestSetCurrentDisplayModeWithHdrAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mode), hdroption, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(&self, mode: &HdmiDisplayMode, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestSetCurrentDisplayModeWithHdrAndMetadataAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mode), hdroption, hdrmetadata, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayModesChanged(&self, value: &super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayModesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisplayModesChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDisplayModesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<HdmiDisplayInformation> {
        Self::IHdmiDisplayInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHdmiDisplayInformationStatics<R, F: FnOnce(&IHdmiDisplayInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HdmiDisplayInformation, IHdmiDisplayInformationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HdmiDisplayInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdmiDisplayInformation {}
impl ::core::fmt::Debug for HdmiDisplayInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayInformation;{130b3c0a-f565-476e-abd5-ea05aee74c69})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for HdmiDisplayInformation {
    const IID: ::windows::core::GUID = <IHdmiDisplayInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HdmiDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayInformation";
}
::windows::core::interface_hierarchy!(HdmiDisplayInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HdmiDisplayInformation {}
unsafe impl ::core::marker::Sync for HdmiDisplayInformation {}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
pub struct HdmiDisplayMode(::windows::core::IUnknown);
impl HdmiDisplayMode {
    pub fn ResolutionWidthInRawPixels(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResolutionWidthInRawPixels)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ResolutionHeightInRawPixels(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResolutionHeightInRawPixels)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RefreshRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RefreshRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StereoEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StereoEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitsPerPixel(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitsPerPixel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEqual(&self, mode: &HdmiDisplayMode) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEqual)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mode), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ColorSpace(&self) -> ::windows::core::Result<HdmiDisplayColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorSpace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelEncoding(&self) -> ::windows::core::Result<HdmiDisplayPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSdrLuminanceSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSdrLuminanceSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSmpte2084Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSmpte2084Supported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Is2086MetadataSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Is2086MetadataSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDolbyVisionLowLatencySupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHdmiDisplayMode2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDolbyVisionLowLatencySupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HdmiDisplayMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdmiDisplayMode {}
impl ::core::fmt::Debug for HdmiDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayMode;{0c06d5ad-1b90-4f51-9981-ef5a1c0ddf66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_Vtbl;
}
unsafe impl ::windows::core::Interface for HdmiDisplayMode {
    const IID: ::windows::core::GUID = <IHdmiDisplayMode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HdmiDisplayMode {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayMode";
}
::windows::core::interface_hierarchy!(HdmiDisplayMode, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HdmiDisplayMode {}
unsafe impl ::core::marker::Sync for HdmiDisplayMode {}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: Self = Self(0i32);
    pub const RgbFull: Self = Self(1i32);
    pub const BT2020: Self = Self(2i32);
    pub const BT709: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayColorSpace {}
impl ::core::clone::Clone for HdmiDisplayColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayColorSpace {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayColorSpace {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayColorSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayColorSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayColorSpace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayColorSpace;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: Self = Self(0i32);
    pub const EotfSdr: Self = Self(1i32);
    pub const Eotf2084: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayHdrOption {}
impl ::core::clone::Clone for HdmiDisplayHdrOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayHdrOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayHdrOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayHdrOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayHdrOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayHdrOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayHdrOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayPixelEncoding {}
impl ::core::clone::Clone for HdmiDisplayPixelEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayPixelEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayPixelEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayPixelEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayPixelEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayPixelEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayPixelEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
pub struct HdmiDisplayHdr2086Metadata {
    pub RedPrimaryX: u16,
    pub RedPrimaryY: u16,
    pub GreenPrimaryX: u16,
    pub GreenPrimaryY: u16,
    pub BluePrimaryX: u16,
    pub BluePrimaryY: u16,
    pub WhitePointX: u16,
    pub WhitePointY: u16,
    pub MaxMasteringLuminance: u16,
    pub MinMasteringLuminance: u16,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for HdmiDisplayHdr2086Metadata {}
impl ::core::clone::Clone for HdmiDisplayHdr2086Metadata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HdmiDisplayHdr2086Metadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HdmiDisplayHdr2086Metadata")
            .field("RedPrimaryX", &self.RedPrimaryX)
            .field("RedPrimaryY", &self.RedPrimaryY)
            .field("GreenPrimaryX", &self.GreenPrimaryX)
            .field("GreenPrimaryY", &self.GreenPrimaryY)
            .field("BluePrimaryX", &self.BluePrimaryX)
            .field("BluePrimaryY", &self.BluePrimaryY)
            .field("WhitePointX", &self.WhitePointX)
            .field("WhitePointY", &self.WhitePointY)
            .field("MaxMasteringLuminance", &self.MaxMasteringLuminance)
            .field("MinMasteringLuminance", &self.MinMasteringLuminance)
            .field("MaxContentLightLevel", &self.MaxContentLightLevel)
            .field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayHdr2086Metadata {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayHdr2086Metadata {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.Core.HdmiDisplayHdr2086Metadata;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayHdr2086Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.RedPrimaryX == other.RedPrimaryX && self.RedPrimaryY == other.RedPrimaryY && self.GreenPrimaryX == other.GreenPrimaryX && self.GreenPrimaryY == other.GreenPrimaryY && self.BluePrimaryX == other.BluePrimaryX && self.BluePrimaryY == other.BluePrimaryY && self.WhitePointX == other.WhitePointX && self.WhitePointY == other.WhitePointY && self.MaxMasteringLuminance == other.MaxMasteringLuminance && self.MinMasteringLuminance == other.MinMasteringLuminance && self.MaxContentLightLevel == other.MaxContentLightLevel && self.MaxFrameAverageLightLevel == other.MaxFrameAverageLightLevel
    }
}
impl ::core::cmp::Eq for HdmiDisplayHdr2086Metadata {}
impl ::core::default::Default for HdmiDisplayHdr2086Metadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
