#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AdvancedPhotoCaptureSettings(pub ::windows::core::IInspectable);
impl AdvancedPhotoCaptureSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AdvancedPhotoCaptureSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__: AdvancedPhotoMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdvancedPhotoMode>(result__)
        }
    }
    pub fn SetMode(&self, value: AdvancedPhotoMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AdvancedPhotoCaptureSettings;{08f3863a-0018-445b-93d2-646d1c5ed05c})");
}
unsafe impl ::windows::core::Interface for AdvancedPhotoCaptureSettings {
    type Vtable = IAdvancedPhotoCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f3863a_0018_445b_93d2_646d1c5ed05c);
}
impl ::windows::core::RuntimeName for AdvancedPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoCaptureSettings";
}
impl ::core::convert::From<AdvancedPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: AdvancedPhotoCaptureSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AdvancedPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoCaptureSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdvancedPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdvancedPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AdvancedPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: AdvancedPhotoCaptureSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AdvancedPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoCaptureSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdvancedPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdvancedPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AdvancedPhotoCaptureSettings {}
unsafe impl ::core::marker::Sync for AdvancedPhotoCaptureSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AdvancedPhotoControl(pub ::windows::core::IInspectable);
impl AdvancedPhotoControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__: AdvancedPhotoMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdvancedPhotoMode>(result__)
        }
    }
    pub fn Configure<'a, Param0: ::windows::core::IntoParam<'a, AdvancedPhotoCaptureSettings>>(&self, settings: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), settings.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AdvancedPhotoControl;{c5b15486-9001-4682-9309-68eae0080eec})");
}
unsafe impl ::windows::core::Interface for AdvancedPhotoControl {
    type Vtable = IAdvancedPhotoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b15486_9001_4682_9309_68eae0080eec);
}
impl ::windows::core::RuntimeName for AdvancedPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoControl";
}
impl ::core::convert::From<AdvancedPhotoControl> for ::windows::core::IUnknown {
    fn from(value: AdvancedPhotoControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AdvancedPhotoControl> for ::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdvancedPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdvancedPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AdvancedPhotoControl> for ::windows::core::IInspectable {
    fn from(value: AdvancedPhotoControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AdvancedPhotoControl> for ::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdvancedPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdvancedPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AdvancedPhotoControl {}
unsafe impl ::core::marker::Sync for AdvancedPhotoControl {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdvancedPhotoMode(pub i32);
impl AdvancedPhotoMode {
    pub const Auto: AdvancedPhotoMode = AdvancedPhotoMode(0i32);
    pub const Standard: AdvancedPhotoMode = AdvancedPhotoMode(1i32);
    pub const Hdr: AdvancedPhotoMode = AdvancedPhotoMode(2i32);
    pub const LowLight: AdvancedPhotoMode = AdvancedPhotoMode(3i32);
}
impl ::core::convert::From<i32> for AdvancedPhotoMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AdvancedPhotoMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AdvancedPhotoMode;i4)");
}
impl ::windows::core::DefaultType for AdvancedPhotoMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioDeviceController(pub ::windows::core::IInspectable);
impl AudioDeviceController {
    pub fn SetMuted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Muted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetVolumePercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VolumePercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>(result__)
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::MediaProperties::IMediaEncodingProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mediastreamtype, mediaencodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceController;{edd4a388-79c7-4f7c-90e8-ef934b21580a})");
}
unsafe impl ::windows::core::Interface for AudioDeviceController {
    type Vtable = IAudioDeviceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedd4a388_79c7_4f7c_90e8_ef934b21580a);
}
impl ::windows::core::RuntimeName for AudioDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceController";
}
impl ::core::convert::From<AudioDeviceController> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioDeviceController> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioDeviceController> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioDeviceController> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<AudioDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaDeviceController> for AudioDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaDeviceController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaDeviceController> for &AudioDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaDeviceController> {
        ::core::convert::TryInto::<IMediaDeviceController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioDeviceModule(pub ::windows::core::IInspectable);
impl AudioDeviceModule {
    pub fn ClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MajorVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MinorVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendCommandAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, command: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ModuleCommandResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), command.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceModule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModule;{86cfac36-47c1-4b33-9852-8773ec4be123})");
}
unsafe impl ::windows::core::Interface for AudioDeviceModule {
    type Vtable = IAudioDeviceModule_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86cfac36_47c1_4b33_9852_8773ec4be123);
}
impl ::windows::core::RuntimeName for AudioDeviceModule {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModule";
}
impl ::core::convert::From<AudioDeviceModule> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceModule) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioDeviceModule> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceModule) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceModule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceModule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioDeviceModule> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceModule) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioDeviceModule> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceModule) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceModule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceModule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioDeviceModuleNotificationEventArgs(pub ::windows::core::IInspectable);
impl AudioDeviceModuleNotificationEventArgs {
    pub fn Module(&self) -> ::windows::core::Result<AudioDeviceModule> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceModule>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn NotificationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceModuleNotificationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs;{e3e3ccaf-224c-48be-956b-9a13134e96e8})");
}
unsafe impl ::windows::core::Interface for AudioDeviceModuleNotificationEventArgs {
    type Vtable = IAudioDeviceModuleNotificationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3e3ccaf_224c_48be_956b_9a13134e96e8);
}
impl ::windows::core::RuntimeName for AudioDeviceModuleNotificationEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs";
}
impl ::core::convert::From<AudioDeviceModuleNotificationEventArgs> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceModuleNotificationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioDeviceModuleNotificationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceModuleNotificationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceModuleNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceModuleNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioDeviceModuleNotificationEventArgs> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceModuleNotificationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioDeviceModuleNotificationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceModuleNotificationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceModuleNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceModuleNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioDeviceModuleNotificationEventArgs {}
unsafe impl ::core::marker::Sync for AudioDeviceModuleNotificationEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioDeviceModulesManager(pub ::windows::core::IInspectable);
impl AudioDeviceModulesManager {
    #[cfg(feature = "Foundation")]
    pub fn ModuleNotificationReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveModuleNotificationReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, moduleid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), moduleid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<AudioDeviceModulesManager> {
        Self::IAudioDeviceModulesManagerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<AudioDeviceModulesManager>(result__)
        })
    }
    pub fn IAudioDeviceModulesManagerFactory<R, F: FnOnce(&IAudioDeviceModulesManagerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioDeviceModulesManager, IAudioDeviceModulesManagerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceModulesManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModulesManager;{6aa40c4d-960a-4d1c-b318-0022604547ed})");
}
unsafe impl ::windows::core::Interface for AudioDeviceModulesManager {
    type Vtable = IAudioDeviceModulesManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aa40c4d_960a_4d1c_b318_0022604547ed);
}
impl ::windows::core::RuntimeName for AudioDeviceModulesManager {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModulesManager";
}
impl ::core::convert::From<AudioDeviceModulesManager> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceModulesManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioDeviceModulesManager> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceModulesManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceModulesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceModulesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioDeviceModulesManager> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceModulesManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioDeviceModulesManager> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceModulesManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceModulesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceModulesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioDeviceModulesManager {}
unsafe impl ::core::marker::Sync for AudioDeviceModulesManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioDeviceRole(pub i32);
impl AudioDeviceRole {
    pub const Default: AudioDeviceRole = AudioDeviceRole(0i32);
    pub const Communications: AudioDeviceRole = AudioDeviceRole(1i32);
}
impl ::core::convert::From<i32> for AudioDeviceRole {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioDeviceRole {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceRole {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AudioDeviceRole;i4)");
}
impl ::windows::core::DefaultType for AudioDeviceRole {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: AutoFocusRange = AutoFocusRange(0i32);
    pub const Macro: AutoFocusRange = AutoFocusRange(1i32);
    pub const Normal: AutoFocusRange = AutoFocusRange(2i32);
}
impl ::core::convert::From<i32> for AutoFocusRange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutoFocusRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutoFocusRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AutoFocusRange;i4)");
}
impl ::windows::core::DefaultType for AutoFocusRange {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CallControl(pub ::windows::core::IInspectable);
impl CallControl {
    pub fn IndicateNewIncomingCall<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, enableringer: bool, callerid: Param1) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), enableringer, callerid.into_param().abi(), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn IndicateNewOutgoingCall(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn IndicateActiveCall(&self, calltoken: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), calltoken).ok() }
    }
    pub fn EndCall(&self, calltoken: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), calltoken).ok() }
    }
    pub fn HasRinger(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AnswerRequested<'a, Param0: ::windows::core::IntoParam<'a, CallControlEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAnswerRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HangUpRequested<'a, Param0: ::windows::core::IntoParam<'a, CallControlEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHangUpRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DialRequested<'a, Param0: ::windows::core::IntoParam<'a, DialRequestedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDialRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RedialRequested<'a, Param0: ::windows::core::IntoParam<'a, RedialRequestedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRedialRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeypadPressed<'a, Param0: ::windows::core::IntoParam<'a, KeypadPressedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeypadPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AudioTransferRequested<'a, Param0: ::windows::core::IntoParam<'a, CallControlEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioTransferRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CallControl>(result__)
        })
    }
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<CallControl>(result__)
        })
    }
    pub fn ICallControlStatics<R, F: FnOnce(&ICallControlStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CallControl, ICallControlStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CallControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CallControl;{a520d0d6-ae8d-45db-8011-ca49d3b3e578})");
}
unsafe impl ::windows::core::Interface for CallControl {
    type Vtable = ICallControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa520d0d6_ae8d_45db_8011_ca49d3b3e578);
}
impl ::windows::core::RuntimeName for CallControl {
    const NAME: &'static str = "Windows.Media.Devices.CallControl";
}
impl ::core::convert::From<CallControl> for ::windows::core::IUnknown {
    fn from(value: CallControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CallControl> for ::windows::core::IUnknown {
    fn from(value: &CallControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CallControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CallControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CallControl> for ::windows::core::IInspectable {
    fn from(value: CallControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CallControl> for ::windows::core::IInspectable {
    fn from(value: &CallControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CallControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CallControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CallControl {}
unsafe impl ::core::marker::Sync for CallControl {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CallControlEventHandler(::windows::core::IUnknown);
impl CallControlEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = CallControlEventHandler_box::<F> { vtable: &CallControlEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, CallControl>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CallControlEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({596f759f-50df-4454-bc63-4d3d01b61958})");
}
unsafe impl ::windows::core::Interface for CallControlEventHandler {
    type Vtable = CallControlEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x596f759f_50df_4454_bc63_4d3d01b61958);
}
#[repr(C)]
#[doc(hidden)]
pub struct CallControlEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct CallControlEventHandler_box<F: FnMut(&::core::option::Option<CallControl>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const CallControlEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>) -> ::windows::core::Result<()> + 'static> CallControlEventHandler_box<F> {
    const VTABLE: CallControlEventHandler_abi = CallControlEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<CallControlEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <CallControl as ::windows::core::Abi>::Abi as *const <CallControl as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraOcclusionInfo(pub ::windows::core::IInspectable);
impl CameraOcclusionInfo {
    pub fn GetState(&self) -> ::windows::core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraOcclusionState>(result__)
        }
    }
    pub fn IsOcclusionKindSupported(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), occlusionkind, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionInfo;{af6c4ad0-a84d-5db6-be58-a5da21cfe011})");
}
unsafe impl ::windows::core::Interface for CameraOcclusionInfo {
    type Vtable = ICameraOcclusionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6c4ad0_a84d_5db6_be58_a5da21cfe011);
}
impl ::windows::core::RuntimeName for CameraOcclusionInfo {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionInfo";
}
impl ::core::convert::From<CameraOcclusionInfo> for ::windows::core::IUnknown {
    fn from(value: CameraOcclusionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraOcclusionInfo> for ::windows::core::IUnknown {
    fn from(value: &CameraOcclusionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraOcclusionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraOcclusionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraOcclusionInfo> for ::windows::core::IInspectable {
    fn from(value: CameraOcclusionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraOcclusionInfo> for ::windows::core::IInspectable {
    fn from(value: &CameraOcclusionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraOcclusionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraOcclusionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CameraOcclusionInfo {}
unsafe impl ::core::marker::Sync for CameraOcclusionInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: CameraOcclusionKind = CameraOcclusionKind(0i32);
    pub const CameraHardware: CameraOcclusionKind = CameraOcclusionKind(1i32);
}
impl ::core::convert::From<i32> for CameraOcclusionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CameraOcclusionKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraOcclusionKind;i4)");
}
impl ::windows::core::DefaultType for CameraOcclusionKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraOcclusionState(pub ::windows::core::IInspectable);
impl CameraOcclusionState {
    pub fn IsOccluded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsOcclusionKind(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), occlusionkind, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionState;{430adeb8-6842-5e55-9bde-04b4ef3a8a57})");
}
unsafe impl ::windows::core::Interface for CameraOcclusionState {
    type Vtable = ICameraOcclusionState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x430adeb8_6842_5e55_9bde_04b4ef3a8a57);
}
impl ::windows::core::RuntimeName for CameraOcclusionState {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionState";
}
impl ::core::convert::From<CameraOcclusionState> for ::windows::core::IUnknown {
    fn from(value: CameraOcclusionState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraOcclusionState> for ::windows::core::IUnknown {
    fn from(value: &CameraOcclusionState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraOcclusionState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraOcclusionState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraOcclusionState> for ::windows::core::IInspectable {
    fn from(value: CameraOcclusionState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraOcclusionState> for ::windows::core::IInspectable {
    fn from(value: &CameraOcclusionState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraOcclusionState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraOcclusionState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CameraOcclusionState {}
unsafe impl ::core::marker::Sync for CameraOcclusionState {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraOcclusionStateChangedEventArgs(pub ::windows::core::IInspectable);
impl CameraOcclusionStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraOcclusionState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CameraOcclusionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionStateChangedEventArgs;{8512d848-c0de-57ca-a1ca-fb2c3d23df55})");
}
unsafe impl ::windows::core::Interface for CameraOcclusionStateChangedEventArgs {
    type Vtable = ICameraOcclusionStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8512d848_c0de_57ca_a1ca_fb2c3d23df55);
}
impl ::windows::core::RuntimeName for CameraOcclusionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionStateChangedEventArgs";
}
impl ::core::convert::From<CameraOcclusionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CameraOcclusionStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraOcclusionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CameraOcclusionStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraOcclusionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraOcclusionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraOcclusionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CameraOcclusionStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraOcclusionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CameraOcclusionStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraOcclusionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraOcclusionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CameraOcclusionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for CameraOcclusionStateChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: CameraStreamState = CameraStreamState(0i32);
    pub const Streaming: CameraStreamState = CameraStreamState(1i32);
    pub const BlockedForPrivacy: CameraStreamState = CameraStreamState(2i32);
    pub const Shutdown: CameraStreamState = CameraStreamState(3i32);
}
impl ::core::convert::From<i32> for CameraStreamState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CameraStreamState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CameraStreamState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraStreamState;i4)");
}
impl ::windows::core::DefaultType for CameraStreamState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CaptureSceneMode(pub i32);
impl CaptureSceneMode {
    pub const Auto: CaptureSceneMode = CaptureSceneMode(0i32);
    pub const Manual: CaptureSceneMode = CaptureSceneMode(1i32);
    pub const Macro: CaptureSceneMode = CaptureSceneMode(2i32);
    pub const Portrait: CaptureSceneMode = CaptureSceneMode(3i32);
    pub const Sport: CaptureSceneMode = CaptureSceneMode(4i32);
    pub const Snow: CaptureSceneMode = CaptureSceneMode(5i32);
    pub const Night: CaptureSceneMode = CaptureSceneMode(6i32);
    pub const Beach: CaptureSceneMode = CaptureSceneMode(7i32);
    pub const Sunset: CaptureSceneMode = CaptureSceneMode(8i32);
    pub const Candlelight: CaptureSceneMode = CaptureSceneMode(9i32);
    pub const Landscape: CaptureSceneMode = CaptureSceneMode(10i32);
    pub const NightPortrait: CaptureSceneMode = CaptureSceneMode(11i32);
    pub const Backlit: CaptureSceneMode = CaptureSceneMode(12i32);
}
impl ::core::convert::From<i32> for CaptureSceneMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CaptureSceneMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CaptureSceneMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureSceneMode;i4)");
}
impl ::windows::core::DefaultType for CaptureSceneMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: CaptureUse = CaptureUse(0i32);
    pub const Photo: CaptureUse = CaptureUse(1i32);
    pub const Video: CaptureUse = CaptureUse(2i32);
}
impl ::core::convert::From<i32> for CaptureUse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CaptureUse {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CaptureUse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureUse;i4)");
}
impl ::windows::core::DefaultType for CaptureUse {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ColorTemperaturePreset(pub i32);
impl ColorTemperaturePreset {
    pub const Auto: ColorTemperaturePreset = ColorTemperaturePreset(0i32);
    pub const Manual: ColorTemperaturePreset = ColorTemperaturePreset(1i32);
    pub const Cloudy: ColorTemperaturePreset = ColorTemperaturePreset(2i32);
    pub const Daylight: ColorTemperaturePreset = ColorTemperaturePreset(3i32);
    pub const Flash: ColorTemperaturePreset = ColorTemperaturePreset(4i32);
    pub const Fluorescent: ColorTemperaturePreset = ColorTemperaturePreset(5i32);
    pub const Tungsten: ColorTemperaturePreset = ColorTemperaturePreset(6i32);
    pub const Candlelight: ColorTemperaturePreset = ColorTemperaturePreset(7i32);
}
impl ::core::convert::From<i32> for ColorTemperaturePreset {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ColorTemperaturePreset {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ColorTemperaturePreset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ColorTemperaturePreset;i4)");
}
impl ::windows::core::DefaultType for ColorTemperaturePreset {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DefaultAudioCaptureDeviceChangedEventArgs(pub ::windows::core::IInspectable);
impl DefaultAudioCaptureDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Role(&self) -> ::windows::core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceRole = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceRole>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DefaultAudioCaptureDeviceChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs;{110f882f-1c05-4657-a18e-47c9b69f07ab})");
}
unsafe impl ::windows::core::Interface for DefaultAudioCaptureDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110f882f_1c05_4657_a18e_47c9b69f07ab);
}
impl ::windows::core::RuntimeName for DefaultAudioCaptureDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs";
}
impl ::core::convert::From<DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DefaultAudioCaptureDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DefaultAudioCaptureDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DefaultAudioCaptureDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DefaultAudioCaptureDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DefaultAudioCaptureDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DefaultAudioCaptureDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    fn from(value: DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultAudioCaptureDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    fn from(value: &DefaultAudioCaptureDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDefaultAudioDeviceChangedEventArgs> for DefaultAudioCaptureDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDefaultAudioDeviceChangedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDefaultAudioDeviceChangedEventArgs> for &DefaultAudioCaptureDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDefaultAudioDeviceChangedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DefaultAudioCaptureDeviceChangedEventArgs {}
unsafe impl ::core::marker::Sync for DefaultAudioCaptureDeviceChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DefaultAudioRenderDeviceChangedEventArgs(pub ::windows::core::IInspectable);
impl DefaultAudioRenderDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Role(&self) -> ::windows::core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceRole = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceRole>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DefaultAudioRenderDeviceChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs;{110f882f-1c05-4657-a18e-47c9b69f07ab})");
}
unsafe impl ::windows::core::Interface for DefaultAudioRenderDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110f882f_1c05_4657_a18e_47c9b69f07ab);
}
impl ::windows::core::RuntimeName for DefaultAudioRenderDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs";
}
impl ::core::convert::From<DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DefaultAudioRenderDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DefaultAudioRenderDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DefaultAudioRenderDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DefaultAudioRenderDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DefaultAudioRenderDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DefaultAudioRenderDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    fn from(value: DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultAudioRenderDeviceChangedEventArgs> for IDefaultAudioDeviceChangedEventArgs {
    fn from(value: &DefaultAudioRenderDeviceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDefaultAudioDeviceChangedEventArgs> for DefaultAudioRenderDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDefaultAudioDeviceChangedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDefaultAudioDeviceChangedEventArgs> for &DefaultAudioRenderDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDefaultAudioDeviceChangedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DefaultAudioRenderDeviceChangedEventArgs {}
unsafe impl ::core::marker::Sync for DefaultAudioRenderDeviceChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialRequestedEventArgs(pub ::windows::core::IInspectable);
impl DialRequestedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Contact(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DialRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DialRequestedEventArgs;{037b929e-953c-4286-8866-4f0f376c855a})");
}
unsafe impl ::windows::core::Interface for DialRequestedEventArgs {
    type Vtable = IDialRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x037b929e_953c_4286_8866_4f0f376c855a);
}
impl ::windows::core::RuntimeName for DialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DialRequestedEventArgs";
}
impl ::core::convert::From<DialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DialRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DialRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DialRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DialRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DialRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialRequestedEventHandler(::windows::core::IUnknown);
impl DialRequestedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<DialRequestedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DialRequestedEventHandler_box::<F> { vtable: &DialRequestedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, CallControl>, Param1: ::windows::core::IntoParam<'a, DialRequestedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DialRequestedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({5abbffdb-c21f-4bc4-891b-257e28c1b1a4})");
}
unsafe impl ::windows::core::Interface for DialRequestedEventHandler {
    type Vtable = DialRequestedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5abbffdb_c21f_4bc4_891b_257e28c1b1a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct DialRequestedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct DialRequestedEventHandler_box<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<DialRequestedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const DialRequestedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<DialRequestedEventArgs>) -> ::windows::core::Result<()> + 'static> DialRequestedEventHandler_box<F> {
    const VTABLE: DialRequestedEventHandler_abi = DialRequestedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DialRequestedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <CallControl as ::windows::core::Abi>::Abi as *const <CallControl as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <DialRequestedEventArgs as ::windows::core::Abi>::Abi as *const <DialRequestedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DigitalWindowBounds(pub ::windows::core::IInspectable);
impl DigitalWindowBounds {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DigitalWindowBounds, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NormalizedOriginTop(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetNormalizedOriginTop(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NormalizedOriginLeft(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetNormalizedOriginLeft(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowBounds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowBounds;{dd4f21dd-d173-5c6b-8c25-bdd26d5122b1})");
}
unsafe impl ::windows::core::Interface for DigitalWindowBounds {
    type Vtable = IDigitalWindowBounds_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4f21dd_d173_5c6b_8c25_bdd26d5122b1);
}
impl ::windows::core::RuntimeName for DigitalWindowBounds {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowBounds";
}
impl ::core::convert::From<DigitalWindowBounds> for ::windows::core::IUnknown {
    fn from(value: DigitalWindowBounds) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DigitalWindowBounds> for ::windows::core::IUnknown {
    fn from(value: &DigitalWindowBounds) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DigitalWindowBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DigitalWindowBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DigitalWindowBounds> for ::windows::core::IInspectable {
    fn from(value: DigitalWindowBounds) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DigitalWindowBounds> for ::windows::core::IInspectable {
    fn from(value: &DigitalWindowBounds) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DigitalWindowBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DigitalWindowBounds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DigitalWindowBounds {}
unsafe impl ::core::marker::Sync for DigitalWindowBounds {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DigitalWindowCapability(pub ::windows::core::IInspectable);
impl DigitalWindowCapability {
    pub fn Width(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MinScaleValue(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn MaxScaleValue(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn MinScaleValueWithoutUpsampling(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NormalizedFieldOfViewLimit(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowCapability;{d78bad2c-f721-5244-a196-b56ccbec606c})");
}
unsafe impl ::windows::core::Interface for DigitalWindowCapability {
    type Vtable = IDigitalWindowCapability_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78bad2c_f721_5244_a196_b56ccbec606c);
}
impl ::windows::core::RuntimeName for DigitalWindowCapability {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowCapability";
}
impl ::core::convert::From<DigitalWindowCapability> for ::windows::core::IUnknown {
    fn from(value: DigitalWindowCapability) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DigitalWindowCapability> for ::windows::core::IUnknown {
    fn from(value: &DigitalWindowCapability) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DigitalWindowCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DigitalWindowCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DigitalWindowCapability> for ::windows::core::IInspectable {
    fn from(value: DigitalWindowCapability) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DigitalWindowCapability> for ::windows::core::IInspectable {
    fn from(value: &DigitalWindowCapability) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DigitalWindowCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DigitalWindowCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DigitalWindowCapability {}
unsafe impl ::core::marker::Sync for DigitalWindowCapability {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DigitalWindowControl(pub ::windows::core::IInspectable);
impl DigitalWindowControl {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SupportedModes(&self) -> ::windows::core::Result<::windows::core::Array<DigitalWindowMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<DigitalWindowMode> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::core::Array::<DigitalWindowMode>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn CurrentMode(&self) -> ::windows::core::Result<DigitalWindowMode> {
        let this = self;
        unsafe {
            let mut result__: DigitalWindowMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DigitalWindowMode>(result__)
        }
    }
    pub fn GetBounds(&self) -> ::windows::core::Result<DigitalWindowBounds> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DigitalWindowBounds>(result__)
        }
    }
    pub fn Configure(&self, digitalwindowmode: DigitalWindowMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), digitalwindowmode).ok() }
    }
    pub fn ConfigureWithBounds<'a, Param1: ::windows::core::IntoParam<'a, DigitalWindowBounds>>(&self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), digitalwindowmode, digitalwindowbounds.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCapabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>>(result__)
        }
    }
    pub fn GetCapabilityForSize(&self, width: i32, height: i32) -> ::windows::core::Result<DigitalWindowCapability> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), width, height, &mut result__).from_abi::<DigitalWindowCapability>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowControl;{23b69eff-65d2-53ea-8780-de582b48b544})");
}
unsafe impl ::windows::core::Interface for DigitalWindowControl {
    type Vtable = IDigitalWindowControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23b69eff_65d2_53ea_8780_de582b48b544);
}
impl ::windows::core::RuntimeName for DigitalWindowControl {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowControl";
}
impl ::core::convert::From<DigitalWindowControl> for ::windows::core::IUnknown {
    fn from(value: DigitalWindowControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DigitalWindowControl> for ::windows::core::IUnknown {
    fn from(value: &DigitalWindowControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DigitalWindowControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DigitalWindowControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DigitalWindowControl> for ::windows::core::IInspectable {
    fn from(value: DigitalWindowControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DigitalWindowControl> for ::windows::core::IInspectable {
    fn from(value: &DigitalWindowControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DigitalWindowControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DigitalWindowControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DigitalWindowControl {}
unsafe impl ::core::marker::Sync for DigitalWindowControl {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DigitalWindowMode(pub i32);
impl DigitalWindowMode {
    pub const Off: DigitalWindowMode = DigitalWindowMode(0i32);
    pub const On: DigitalWindowMode = DigitalWindowMode(1i32);
    pub const Auto: DigitalWindowMode = DigitalWindowMode(2i32);
}
impl ::core::convert::From<i32> for DigitalWindowMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DigitalWindowMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DigitalWindowMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.DigitalWindowMode;i4)");
}
impl ::windows::core::DefaultType for DigitalWindowMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExposureCompensationControl(pub ::windows::core::IInspectable);
impl ExposureCompensationControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, value: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ExposureCompensationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposureCompensationControl;{81c8e834-dcec-4011-a610-1f3847e64aca})");
}
unsafe impl ::windows::core::Interface for ExposureCompensationControl {
    type Vtable = IExposureCompensationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c8e834_dcec_4011_a610_1f3847e64aca);
}
impl ::windows::core::RuntimeName for ExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureCompensationControl";
}
impl ::core::convert::From<ExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: ExposureCompensationControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: &ExposureCompensationControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: ExposureCompensationControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: &ExposureCompensationControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExposureCompensationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExposureControl(pub ::windows::core::IInspectable);
impl ExposureControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetAutoAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Min(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Max(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Step(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, shutterduration: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), shutterduration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ExposureControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposureControl;{09e8cbe2-ad96-4f28-a0e0-96ed7e1b5fd2})");
}
unsafe impl ::windows::core::Interface for ExposureControl {
    type Vtable = IExposureControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e8cbe2_ad96_4f28_a0e0_96ed7e1b5fd2);
}
impl ::windows::core::RuntimeName for ExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureControl";
}
impl ::core::convert::From<ExposureControl> for ::windows::core::IUnknown {
    fn from(value: ExposureControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExposureControl> for ::windows::core::IUnknown {
    fn from(value: &ExposureControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExposureControl> for ::windows::core::IInspectable {
    fn from(value: ExposureControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExposureControl> for ::windows::core::IInspectable {
    fn from(value: &ExposureControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExposureControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExposurePriorityVideoControl(pub ::windows::core::IInspectable);
impl ExposurePriorityVideoControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ExposurePriorityVideoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposurePriorityVideoControl;{2cb240a3-5168-4271-9ea5-47621a98a352})");
}
unsafe impl ::windows::core::Interface for ExposurePriorityVideoControl {
    type Vtable = IExposurePriorityVideoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cb240a3_5168_4271_9ea5_47621a98a352);
}
impl ::windows::core::RuntimeName for ExposurePriorityVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposurePriorityVideoControl";
}
impl ::core::convert::From<ExposurePriorityVideoControl> for ::windows::core::IUnknown {
    fn from(value: ExposurePriorityVideoControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExposurePriorityVideoControl> for ::windows::core::IUnknown {
    fn from(value: &ExposurePriorityVideoControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExposurePriorityVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExposurePriorityVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExposurePriorityVideoControl> for ::windows::core::IInspectable {
    fn from(value: ExposurePriorityVideoControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExposurePriorityVideoControl> for ::windows::core::IInspectable {
    fn from(value: &ExposurePriorityVideoControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExposurePriorityVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExposurePriorityVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExposurePriorityVideoControl {}
unsafe impl ::core::marker::Sync for ExposurePriorityVideoControl {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FlashControl(pub ::windows::core::IInspectable);
impl FlashControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RedEyeReduction(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AssistantLightSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AssistantLightEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAssistantLightEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFlashControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FlashControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FlashControl;{def41dbe-7d68-45e3-8c0f-be7bb32837d0})");
}
unsafe impl ::windows::core::Interface for FlashControl {
    type Vtable = IFlashControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef41dbe_7d68_45e3_8c0f_be7bb32837d0);
}
impl ::windows::core::RuntimeName for FlashControl {
    const NAME: &'static str = "Windows.Media.Devices.FlashControl";
}
impl ::core::convert::From<FlashControl> for ::windows::core::IUnknown {
    fn from(value: FlashControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FlashControl> for ::windows::core::IUnknown {
    fn from(value: &FlashControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FlashControl> for ::windows::core::IInspectable {
    fn from(value: FlashControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FlashControl> for ::windows::core::IInspectable {
    fn from(value: &FlashControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FlashControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusControl(pub ::windows::core::IInspectable);
impl FocusControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusPreset>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<FocusPreset>>(result__)
        }
    }
    pub fn Preset(&self) -> ::windows::core::Result<FocusPreset> {
        let this = self;
        unsafe {
            let mut result__: FocusPreset = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusPreset>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPresetAsync(&self, preset: FocusPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), preset, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPresetWithCompletionOptionAsync(&self, preset: FocusPreset, completebeforefocus: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), preset, completebeforefocus, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, focus: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), focus, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FocusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn FocusChangedSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn WaitForFocusSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusMode>> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<FocusMode>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusDistances(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AutoFocusRange>> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AutoFocusRange>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<FocusMode> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: FocusMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusMode>(result__)
        }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<MediaCaptureFocusState> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: MediaCaptureFocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureFocusState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnlockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Configure<'a, Param0: ::windows::core::IntoParam<'a, FocusSettings>>(&self, settings: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFocusControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), settings.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FocusControl;{c0d889f6-5228-4453-b153-85606592b238})");
}
unsafe impl ::windows::core::Interface for FocusControl {
    type Vtable = IFocusControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d889f6_5228_4453_b153_85606592b238);
}
impl ::windows::core::RuntimeName for FocusControl {
    const NAME: &'static str = "Windows.Media.Devices.FocusControl";
}
impl ::core::convert::From<FocusControl> for ::windows::core::IUnknown {
    fn from(value: FocusControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusControl> for ::windows::core::IUnknown {
    fn from(value: &FocusControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusControl> for ::windows::core::IInspectable {
    fn from(value: FocusControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusControl> for ::windows::core::IInspectable {
    fn from(value: &FocusControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FocusMode(pub i32);
impl FocusMode {
    pub const Auto: FocusMode = FocusMode(0i32);
    pub const Single: FocusMode = FocusMode(1i32);
    pub const Continuous: FocusMode = FocusMode(2i32);
    pub const Manual: FocusMode = FocusMode(3i32);
}
impl ::core::convert::From<i32> for FocusMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FocusMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FocusMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusMode;i4)");
}
impl ::windows::core::DefaultType for FocusMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FocusPreset(pub i32);
impl FocusPreset {
    pub const Auto: FocusPreset = FocusPreset(0i32);
    pub const Manual: FocusPreset = FocusPreset(1i32);
    pub const AutoMacro: FocusPreset = FocusPreset(2i32);
    pub const AutoNormal: FocusPreset = FocusPreset(3i32);
    pub const AutoInfinity: FocusPreset = FocusPreset(4i32);
    pub const AutoHyperfocal: FocusPreset = FocusPreset(5i32);
}
impl ::core::convert::From<i32> for FocusPreset {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FocusPreset {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FocusPreset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusPreset;i4)");
}
impl ::windows::core::DefaultType for FocusPreset {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusSettings(pub ::windows::core::IInspectable);
impl FocusSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Mode(&self) -> ::windows::core::Result<FocusMode> {
        let this = self;
        unsafe {
            let mut result__: FocusMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusMode>(result__)
        }
    }
    pub fn SetMode(&self, value: FocusMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AutoFocusRange(&self) -> ::windows::core::Result<AutoFocusRange> {
        let this = self;
        unsafe {
            let mut result__: AutoFocusRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutoFocusRange>(result__)
        }
    }
    pub fn SetAutoFocusRange(&self, value: AutoFocusRange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Distance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<ManualFocusDistance>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<ManualFocusDistance>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDistance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<ManualFocusDistance>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn WaitForFocus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetWaitForFocus(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DisableDriverFallback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDisableDriverFallback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FocusSettings;{79958f6b-3263-4275-85d6-aeae891c96ee})");
}
unsafe impl ::windows::core::Interface for FocusSettings {
    type Vtable = IFocusSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79958f6b_3263_4275_85d6_aeae891c96ee);
}
impl ::windows::core::RuntimeName for FocusSettings {
    const NAME: &'static str = "Windows.Media.Devices.FocusSettings";
}
impl ::core::convert::From<FocusSettings> for ::windows::core::IUnknown {
    fn from(value: FocusSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusSettings> for ::windows::core::IUnknown {
    fn from(value: &FocusSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusSettings> for ::windows::core::IInspectable {
    fn from(value: FocusSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusSettings> for ::windows::core::IInspectable {
    fn from(value: &FocusSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusSettings {}
unsafe impl ::core::marker::Sync for FocusSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HdrVideoControl(pub ::windows::core::IInspectable);
impl HdrVideoControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HdrVideoMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HdrVideoMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<HdrVideoMode> {
        let this = self;
        unsafe {
            let mut result__: HdrVideoMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HdrVideoMode>(result__)
        }
    }
    pub fn SetMode(&self, value: HdrVideoMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for HdrVideoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.HdrVideoControl;{55d8e2d0-30c0-43bf-9b9a-9799d70ced94})");
}
unsafe impl ::windows::core::Interface for HdrVideoControl {
    type Vtable = IHdrVideoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55d8e2d0_30c0_43bf_9b9a_9799d70ced94);
}
impl ::windows::core::RuntimeName for HdrVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.HdrVideoControl";
}
impl ::core::convert::From<HdrVideoControl> for ::windows::core::IUnknown {
    fn from(value: HdrVideoControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HdrVideoControl> for ::windows::core::IUnknown {
    fn from(value: &HdrVideoControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HdrVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HdrVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HdrVideoControl> for ::windows::core::IInspectable {
    fn from(value: HdrVideoControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HdrVideoControl> for ::windows::core::IInspectable {
    fn from(value: &HdrVideoControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HdrVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HdrVideoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HdrVideoControl {}
unsafe impl ::core::marker::Sync for HdrVideoControl {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: HdrVideoMode = HdrVideoMode(0i32);
    pub const On: HdrVideoMode = HdrVideoMode(1i32);
    pub const Auto: HdrVideoMode = HdrVideoMode(2i32);
}
impl ::core::convert::From<i32> for HdrVideoMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HdrVideoMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HdrVideoMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.HdrVideoMode;i4)");
}
impl ::windows::core::DefaultType for HdrVideoMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedPhotoCaptureSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedPhotoCaptureSettings {
    type Vtable = IAdvancedPhotoCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f3863a_0018_445b_93d2_646d1c5ed05c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AdvancedPhotoMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedPhotoControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedPhotoControl {
    type Vtable = IAdvancedPhotoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b15486_9001_4682_9309_68eae0080eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AdvancedPhotoMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController {
    type Vtable = IAdvancedVideoCaptureDeviceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde6ff4d3_2b96_4583_80ab_b5b01dc6a8d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController10(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController10 {
    type Vtable = IAdvancedVideoCaptureDeviceController10_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc621b82d_d6f0_5c1b_a388_a6e938407146);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController10_abi(
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
pub struct IAdvancedVideoCaptureDeviceController2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController2 {
    type Vtable = IAdvancedVideoCaptureDeviceController2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb94f8f_f11a_43db_b402_11930b80ae56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CaptureUse) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CaptureUse) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController3 {
    type Vtable = IAdvancedVideoCaptureDeviceController3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa98b8f34_ee0d_470c_b9f0_4229c4bbd089);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Devices_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController4 {
    type Vtable = IAdvancedVideoCaptureDeviceController4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9fbfaf_d371_41c3_9a17_824a87ebdfd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCaptureOptimization) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MediaCaptureOptimization) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController5 {
    type Vtable = IAdvancedVideoCaptureDeviceController5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33512b17_b9cb_4a23_b875_f9eaab535492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: ::windows::core::RawPtr, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController6 {
    type Vtable = IAdvancedVideoCaptureDeviceController6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6563a53_68a1_44b7_9f89_b5fa97ac0cbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController6_abi(
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
pub struct IAdvancedVideoCaptureDeviceController7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController7 {
    type Vtable = IAdvancedVideoCaptureDeviceController7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d2927f0_a054_50e7_b7df_7c04234d10f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController7_abi(
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
pub struct IAdvancedVideoCaptureDeviceController8(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController8 {
    type Vtable = IAdvancedVideoCaptureDeviceController8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd843f010_e7fb_595b_9a78_0e54c4532b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController8_abi(
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
pub struct IAdvancedVideoCaptureDeviceController9(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedVideoCaptureDeviceController9 {
    type Vtable = IAdvancedVideoCaptureDeviceController9_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bdca95d_0255_51bc_a10d_5a169ec1625a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController9_abi(
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
pub struct IAudioDeviceController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioDeviceController {
    type Vtable = IAudioDeviceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedd4a388_79c7_4f7c_90e8_ef934b21580a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioDeviceModule(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioDeviceModule {
    type Vtable = IAudioDeviceModule_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86cfac36_47c1_4b33_9852_8773ec4be123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModule_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, command: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioDeviceModuleNotificationEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioDeviceModuleNotificationEventArgs {
    type Vtable = IAudioDeviceModuleNotificationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3e3ccaf_224c_48be_956b_9a13134e96e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModuleNotificationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioDeviceModulesManager {
    type Vtable = IAudioDeviceModulesManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aa40c4d_960a_4d1c_b318_0022604547ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, moduleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManagerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioDeviceModulesManagerFactory {
    type Vtable = IAudioDeviceModulesManagerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8db03670_e64d_4773_96c0_bc7ebf0e063f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManagerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICallControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICallControl {
    type Vtable = ICallControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa520d0d6_ae8d_45db_8011_ca49d3b3e578);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enableringer: bool, callerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, calltoken: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, calltoken: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
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
pub struct ICallControlStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICallControlStatics {
    type Vtable = ICallControlStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03945ad5_85ab_40e1_af19_56c94303b019);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallControlStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraOcclusionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraOcclusionInfo {
    type Vtable = ICameraOcclusionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6c4ad0_a84d_5db6_be58_a5da21cfe011);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraOcclusionState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraOcclusionState {
    type Vtable = ICameraOcclusionState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x430adeb8_6842_5e55_9bde_04b4ef3a8a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraOcclusionStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraOcclusionStateChangedEventArgs {
    type Vtable = ICameraOcclusionStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8512d848_c0de_57ca_a1ca_fb2c3d23df55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDefaultAudioDeviceChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDefaultAudioDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110f882f_1c05_4657_a18e_47c9b69f07ab);
}
impl IDefaultAudioDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Role(&self) -> ::windows::core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceRole = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceRole>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IDefaultAudioDeviceChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{110f882f-1c05-4657-a18e-47c9b69f07ab}");
}
impl ::core::convert::From<IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDefaultAudioDeviceChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDefaultAudioDeviceChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDefaultAudioDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDefaultAudioDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDefaultAudioDeviceChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDefaultAudioDeviceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDefaultAudioDeviceChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDefaultAudioDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDefaultAudioDeviceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultAudioDeviceChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioDeviceRole) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialRequestedEventArgs {
    type Vtable = IDialRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x037b929e_953c_4286_8866_4f0f376c855a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDigitalWindowBounds(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDigitalWindowBounds {
    type Vtable = IDigitalWindowBounds_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4f21dd_d173_5c6b_8c25_bdd26d5122b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowBounds_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDigitalWindowCapability(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDigitalWindowCapability {
    type Vtable = IDigitalWindowCapability_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78bad2c_f721_5244_a196_b56ccbec606c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowCapability_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDigitalWindowControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDigitalWindowControl {
    type Vtable = IDigitalWindowControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23b69eff_65d2_53ea_8780_de582b48b544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut DigitalWindowMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DigitalWindowMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digitalwindowmode: DigitalWindowMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExposureCompensationControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExposureCompensationControl {
    type Vtable = IExposureCompensationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c8e834_dcec_4011_a610_1f3847e64aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposureCompensationControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExposureControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExposureControl {
    type Vtable = IExposureControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e8cbe2_ad96_4f28_a0e0_96ed7e1b5fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposureControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shutterduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExposurePriorityVideoControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExposurePriorityVideoControl {
    type Vtable = IExposurePriorityVideoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cb240a3_5168_4271_9ea5_47621a98a352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposurePriorityVideoControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlashControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlashControl {
    type Vtable = IFlashControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef41dbe_7d68_45e3_8c0f_be7bb32837d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlashControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlashControl2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlashControl2 {
    type Vtable = IFlashControl2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d29cc9e_75e1_4af7_bd7d_4e38e1c06cd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlashControl2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusControl {
    type Vtable = IFocusControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d889f6_5228_4453_b153_85606592b238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusPreset) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preset: FocusPreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preset: FocusPreset, completebeforefocus: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focus: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusControl2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusControl2 {
    type Vtable = IFocusControl2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f7cff48_c534_4e9e_94c3_52ef2afd5d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusControl2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCaptureFocusState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusSettings {
    type Vtable = IFocusSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79958f6b_3263_4275_85d6_aeae891c96ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: FocusMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AutoFocusRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AutoFocusRange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHdrVideoControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHdrVideoControl {
    type Vtable = IHdrVideoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55d8e2d0_30c0_43bf_9b9a_9799d70ced94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdrVideoControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut HdrVideoMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: HdrVideoMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInfraredTorchControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInfraredTorchControl {
    type Vtable = IInfraredTorchControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cba2c83_6cb6_5a04_a6fc_3be7b33ff056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfraredTorchControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InfraredTorchMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InfraredTorchMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsoSpeedControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsoSpeedControl {
    type Vtable = IIsoSpeedControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27b6c322_25ad_4f1b_aaab_524ab376ca33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsoSpeedControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IsoSpeedPreset) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preset: IsoSpeedPreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIsoSpeedControl2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIsoSpeedControl2 {
    type Vtable = IIsoSpeedControl2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f1578f2_6d77_4f8a_8c2f_6130b6395053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsoSpeedControl2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isospeed: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeypadPressedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeypadPressedEventArgs {
    type Vtable = IKeypadPressedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3a43900_b4fa_49cd_9442_89af6568f601);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeypadPressedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TelephonyKey) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagPhotoControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLagPhotoControl {
    type Vtable = ILowLagPhotoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5c4dd0_fadf_415d_aee6_3baa529300c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLagPhotoSequenceControl {
    type Vtable = ILowLagPhotoSequenceControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dcf909d_6d16_409c_bafe_b9a594c6fde6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaDeviceControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaDeviceControl {
    type Vtable = IMediaDeviceControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8dfa9_6f75_4863_ba0b_583f3036b4de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut f64, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut bool, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaDeviceControlCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaDeviceControlCapabilities {
    type Vtable = IMediaDeviceControlCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23005816_eb85_43e2_b92b_8240d5ee70ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceControlCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaDeviceController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaDeviceController {
    type Vtable = IMediaDeviceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6f8f5ce_209a_48fb_86fc_d44578f317e6);
}
impl IMediaDeviceController {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>(result__)
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::MediaProperties::IMediaEncodingProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mediastreamtype, mediaencodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaDeviceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f6f8f5ce-209a-48fb-86fc-d44578f317e6}");
}
impl ::core::convert::From<IMediaDeviceController> for ::windows::core::IUnknown {
    fn from(value: IMediaDeviceController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMediaDeviceController> for ::windows::core::IUnknown {
    fn from(value: &IMediaDeviceController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMediaDeviceController> for ::windows::core::IInspectable {
    fn from(value: IMediaDeviceController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaDeviceController> for ::windows::core::IInspectable {
    fn from(value: &IMediaDeviceController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaDeviceStatics {
    type Vtable = IMediaDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa2d9a40_909f_4bba_bf8b_0c0d296f14f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, role: AudioDeviceRole, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IModuleCommandResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IModuleCommandResult {
    type Vtable = IModuleCommandResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520d1eb4_1374_4c7d_b1e4_39dcdf3eae4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IModuleCommandResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SendCommandStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOpticalImageStabilizationControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOpticalImageStabilizationControl {
    type Vtable = IOpticalImageStabilizationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfad9c1d_00bc_423b_8eb2_a0178ca94247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpticalImageStabilizationControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut OpticalImageStabilizationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: OpticalImageStabilizationMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPanelBasedOptimizationControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPanelBasedOptimizationControl {
    type Vtable = IPanelBasedOptimizationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33323223_6247_5419_a5a4_3d808645d917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPanelBasedOptimizationControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoConfirmationControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoConfirmationControl {
    type Vtable = IPhotoConfirmationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8f3f363_ff5e_4582_a9a8_0550f85a4a76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRedialRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRedialRequestedEventArgs {
    type Vtable = IRedialRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eb55209_76ab_4c31_b40e_4b58379d580c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRedialRequestedEventArgs_abi(
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
pub struct IRegionOfInterest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRegionOfInterest {
    type Vtable = IRegionOfInterest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5ecc834_ce66_4e05_a78f_cf391a5ec2d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionOfInterest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRegionOfInterest2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRegionOfInterest2 {
    type Vtable = IRegionOfInterest2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19fe2a91_73aa_4d51_8a9d_56ccf7db7f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionOfInterest2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut RegionOfInterestType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: RegionOfInterestType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRegionsOfInterestControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRegionsOfInterestControl {
    type Vtable = IRegionsOfInterestControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc323f527_ab0b_4558_8b5b_df5693db0378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionsOfInterestControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, regions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, regions: ::windows::core::RawPtr, lockvalues: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneModeControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISceneModeControl {
    type Vtable = ISceneModeControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd48e5af7_8d59_4854_8c62_12c70ba89b7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModeControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CaptureSceneMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scenemode: CaptureSceneMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITorchControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITorchControl {
    type Vtable = ITorchControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6053665_8250_416c_919a_724296afa306);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITorchControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoDeviceController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoDeviceController {
    type Vtable = IVideoDeviceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99555575_2e2e_40b8_b6c7_f82d10013210);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDeviceController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoDeviceControllerGetDevicePropertyResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoDeviceControllerGetDevicePropertyResult {
    type Vtable = IVideoDeviceControllerGetDevicePropertyResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5d88395_6ed5_4790_8b5d_0ef13935d0f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDeviceControllerGetDevicePropertyResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VideoDeviceControllerGetDevicePropertyStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoTemporalDenoisingControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoTemporalDenoisingControl {
    type Vtable = IVideoTemporalDenoisingControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ab34735_3e2a_4a32_baff_4358c4fbdd57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTemporalDenoisingControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VideoTemporalDenoisingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VideoTemporalDenoisingMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWhiteBalanceControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWhiteBalanceControl {
    type Vtable = IWhiteBalanceControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781f047e_7162_49c8_a8f9_9481c565363e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWhiteBalanceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ColorTemperaturePreset) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preset: ColorTemperaturePreset, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, temperature: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IZoomControl(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IZoomControl {
    type Vtable = IZoomControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a1e0b12_32da_4c17_bfd7_8d0c73c8f5a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IZoomControl2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IZoomControl2 {
    type Vtable = IZoomControl2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69843db0_2e99_4641_8529_184f319d1671);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomControl2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IZoomSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IZoomSettings {
    type Vtable = IZoomSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ad66b24_14b4_4bfd_b18f_88fe24463b52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ZoomTransitionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ZoomTransitionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InfraredTorchControl(pub ::windows::core::IInspectable);
impl InfraredTorchControl {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>>(result__)
        }
    }
    pub fn CurrentMode(&self) -> ::windows::core::Result<InfraredTorchMode> {
        let this = self;
        unsafe {
            let mut result__: InfraredTorchMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InfraredTorchMode>(result__)
        }
    }
    pub fn SetCurrentMode(&self, value: InfraredTorchMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MinPower(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MaxPower(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn PowerStep(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Power(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetPower(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InfraredTorchControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.InfraredTorchControl;{1cba2c83-6cb6-5a04-a6fc-3be7b33ff056})");
}
unsafe impl ::windows::core::Interface for InfraredTorchControl {
    type Vtable = IInfraredTorchControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cba2c83_6cb6_5a04_a6fc_3be7b33ff056);
}
impl ::windows::core::RuntimeName for InfraredTorchControl {
    const NAME: &'static str = "Windows.Media.Devices.InfraredTorchControl";
}
impl ::core::convert::From<InfraredTorchControl> for ::windows::core::IUnknown {
    fn from(value: InfraredTorchControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InfraredTorchControl> for ::windows::core::IUnknown {
    fn from(value: &InfraredTorchControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InfraredTorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InfraredTorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InfraredTorchControl> for ::windows::core::IInspectable {
    fn from(value: InfraredTorchControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InfraredTorchControl> for ::windows::core::IInspectable {
    fn from(value: &InfraredTorchControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InfraredTorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InfraredTorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InfraredTorchControl {}
unsafe impl ::core::marker::Sync for InfraredTorchControl {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InfraredTorchMode(pub i32);
impl InfraredTorchMode {
    pub const Off: InfraredTorchMode = InfraredTorchMode(0i32);
    pub const On: InfraredTorchMode = InfraredTorchMode(1i32);
    pub const AlternatingFrameIllumination: InfraredTorchMode = InfraredTorchMode(2i32);
}
impl ::core::convert::From<i32> for InfraredTorchMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InfraredTorchMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InfraredTorchMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.InfraredTorchMode;i4)");
}
impl ::windows::core::DefaultType for InfraredTorchMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IsoSpeedControl(pub ::windows::core::IInspectable);
impl IsoSpeedControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Preset(&self) -> ::windows::core::Result<IsoSpeedPreset> {
        let this = self;
        unsafe {
            let mut result__: IsoSpeedPreset = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsoSpeedPreset>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetPresetAsync(&self, preset: IsoSpeedPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), preset, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, isospeed: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), isospeed, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetAutoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IsoSpeedControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.IsoSpeedControl;{27b6c322-25ad-4f1b-aaab-524ab376ca33})");
}
unsafe impl ::windows::core::Interface for IsoSpeedControl {
    type Vtable = IIsoSpeedControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27b6c322_25ad_4f1b_aaab_524ab376ca33);
}
impl ::windows::core::RuntimeName for IsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.IsoSpeedControl";
}
impl ::core::convert::From<IsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: IsoSpeedControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: &IsoSpeedControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: IsoSpeedControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: &IsoSpeedControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IsoSpeedControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsoSpeedPreset(pub i32);
impl IsoSpeedPreset {
    pub const Auto: IsoSpeedPreset = IsoSpeedPreset(0i32);
    pub const Iso50: IsoSpeedPreset = IsoSpeedPreset(1i32);
    pub const Iso80: IsoSpeedPreset = IsoSpeedPreset(2i32);
    pub const Iso100: IsoSpeedPreset = IsoSpeedPreset(3i32);
    pub const Iso200: IsoSpeedPreset = IsoSpeedPreset(4i32);
    pub const Iso400: IsoSpeedPreset = IsoSpeedPreset(5i32);
    pub const Iso800: IsoSpeedPreset = IsoSpeedPreset(6i32);
    pub const Iso1600: IsoSpeedPreset = IsoSpeedPreset(7i32);
    pub const Iso3200: IsoSpeedPreset = IsoSpeedPreset(8i32);
    pub const Iso6400: IsoSpeedPreset = IsoSpeedPreset(9i32);
    pub const Iso12800: IsoSpeedPreset = IsoSpeedPreset(10i32);
    pub const Iso25600: IsoSpeedPreset = IsoSpeedPreset(11i32);
}
impl ::core::convert::From<i32> for IsoSpeedPreset {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IsoSpeedPreset {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IsoSpeedPreset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.IsoSpeedPreset;i4)");
}
impl ::windows::core::DefaultType for IsoSpeedPreset {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeypadPressedEventArgs(pub ::windows::core::IInspectable);
impl KeypadPressedEventArgs {
    pub fn TelephonyKey(&self) -> ::windows::core::Result<TelephonyKey> {
        let this = self;
        unsafe {
            let mut result__: TelephonyKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TelephonyKey>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for KeypadPressedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.KeypadPressedEventArgs;{d3a43900-b4fa-49cd-9442-89af6568f601})");
}
unsafe impl ::windows::core::Interface for KeypadPressedEventArgs {
    type Vtable = IKeypadPressedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3a43900_b4fa_49cd_9442_89af6568f601);
}
impl ::windows::core::RuntimeName for KeypadPressedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.KeypadPressedEventArgs";
}
impl ::core::convert::From<KeypadPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeypadPressedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeypadPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeypadPressedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeypadPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeypadPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeypadPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeypadPressedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeypadPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeypadPressedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeypadPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeypadPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for KeypadPressedEventArgs {}
unsafe impl ::core::marker::Sync for KeypadPressedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeypadPressedEventHandler(::windows::core::IUnknown);
impl KeypadPressedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<KeypadPressedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = KeypadPressedEventHandler_box::<F> { vtable: &KeypadPressedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, CallControl>, Param1: ::windows::core::IntoParam<'a, KeypadPressedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for KeypadPressedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({e637a454-c527-422c-8926-c9af83b559a0})");
}
unsafe impl ::windows::core::Interface for KeypadPressedEventHandler {
    type Vtable = KeypadPressedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe637a454_c527_422c_8926_c9af83b559a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct KeypadPressedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct KeypadPressedEventHandler_box<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<KeypadPressedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const KeypadPressedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<KeypadPressedEventArgs>) -> ::windows::core::Result<()> + 'static> KeypadPressedEventHandler_box<F> {
    const VTABLE: KeypadPressedEventHandler_abi = KeypadPressedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<KeypadPressedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <CallControl as ::windows::core::Abi>::Abi as *const <CallControl as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <KeypadPressedEventArgs as ::windows::core::Abi>::Abi as *const <KeypadPressedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LowLagPhotoControl(pub ::windows::core::IInspectable);
impl LowLagPhotoControl {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>>(&self, captureproperties: Param0) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), captureproperties.into_param().abi(), &mut result__).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    pub fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__: super::MediaProperties::MediaThumbnailFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaThumbnailFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LowLagPhotoControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.LowLagPhotoControl;{6d5c4dd0-fadf-415d-aee6-3baa529300c9})");
}
unsafe impl ::windows::core::Interface for LowLagPhotoControl {
    type Vtable = ILowLagPhotoControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5c4dd0_fadf_415d_aee6_3baa529300c9);
}
impl ::windows::core::RuntimeName for LowLagPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoControl";
}
impl ::core::convert::From<LowLagPhotoControl> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LowLagPhotoControl> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LowLagPhotoControl> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LowLagPhotoControl> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagPhotoControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LowLagPhotoSequenceControl(pub ::windows::core::IInspectable);
impl LowLagPhotoSequenceControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MaxPastPhotos(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn PastPhotoLimit(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetPastPhotoLimit(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>>(&self, captureproperties: Param0) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), captureproperties.into_param().abi(), &mut result__).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    pub fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__: super::MediaProperties::MediaThumbnailFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaThumbnailFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LowLagPhotoSequenceControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.LowLagPhotoSequenceControl;{3dcf909d-6d16-409c-bafe-b9a594c6fde6})");
}
unsafe impl ::windows::core::Interface for LowLagPhotoSequenceControl {
    type Vtable = ILowLagPhotoSequenceControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dcf909d_6d16_409c_bafe_b9a594c6fde6);
}
impl ::windows::core::RuntimeName for LowLagPhotoSequenceControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoSequenceControl";
}
impl ::core::convert::From<LowLagPhotoSequenceControl> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoSequenceControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceControl> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoSequenceControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagPhotoSequenceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagPhotoSequenceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LowLagPhotoSequenceControl> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoSequenceControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceControl> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoSequenceControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagPhotoSequenceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagPhotoSequenceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ManualFocusDistance(pub i32);
impl ManualFocusDistance {
    pub const Infinity: ManualFocusDistance = ManualFocusDistance(0i32);
    pub const Hyperfocal: ManualFocusDistance = ManualFocusDistance(1i32);
    pub const Nearest: ManualFocusDistance = ManualFocusDistance(2i32);
}
impl ::core::convert::From<i32> for ManualFocusDistance {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ManualFocusDistance {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ManualFocusDistance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ManualFocusDistance;i4)");
}
impl ::windows::core::DefaultType for ManualFocusDistance {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureFocusState(pub i32);
impl MediaCaptureFocusState {
    pub const Uninitialized: MediaCaptureFocusState = MediaCaptureFocusState(0i32);
    pub const Lost: MediaCaptureFocusState = MediaCaptureFocusState(1i32);
    pub const Searching: MediaCaptureFocusState = MediaCaptureFocusState(2i32);
    pub const Focused: MediaCaptureFocusState = MediaCaptureFocusState(3i32);
    pub const Failed: MediaCaptureFocusState = MediaCaptureFocusState(4i32);
}
impl ::core::convert::From<i32> for MediaCaptureFocusState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureFocusState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureFocusState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureFocusState;i4)");
}
impl ::windows::core::DefaultType for MediaCaptureFocusState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureOptimization(pub i32);
impl MediaCaptureOptimization {
    pub const Default: MediaCaptureOptimization = MediaCaptureOptimization(0i32);
    pub const Quality: MediaCaptureOptimization = MediaCaptureOptimization(1i32);
    pub const Latency: MediaCaptureOptimization = MediaCaptureOptimization(2i32);
    pub const Power: MediaCaptureOptimization = MediaCaptureOptimization(3i32);
    pub const LatencyThenQuality: MediaCaptureOptimization = MediaCaptureOptimization(4i32);
    pub const LatencyThenPower: MediaCaptureOptimization = MediaCaptureOptimization(5i32);
    pub const PowerAndQuality: MediaCaptureOptimization = MediaCaptureOptimization(6i32);
}
impl ::core::convert::From<i32> for MediaCaptureOptimization {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureOptimization {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureOptimization {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureOptimization;i4)");
}
impl ::windows::core::DefaultType for MediaCaptureOptimization {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: MediaCapturePauseBehavior = MediaCapturePauseBehavior(0i32);
    pub const ReleaseHardwareResources: MediaCapturePauseBehavior = MediaCapturePauseBehavior(1i32);
}
impl ::core::convert::From<i32> for MediaCapturePauseBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCapturePauseBehavior {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCapturePauseBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCapturePauseBehavior;i4)");
}
impl ::windows::core::DefaultType for MediaCapturePauseBehavior {
    type DefaultType = Self;
}
pub struct MediaDevice {}
impl MediaDevice {
    pub fn GetAudioCaptureSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetAudioRenderSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetVideoCaptureSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDefaultAudioCaptureId(role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), role, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDefaultAudioRenderId(role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), role, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn DefaultAudioCaptureDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultAudioCaptureDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn DefaultAudioRenderDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultAudioRenderDeviceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    pub fn IMediaDeviceStatics<R, F: FnOnce(&IMediaDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaDevice, IMediaDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MediaDevice {
    const NAME: &'static str = "Windows.Media.Devices.MediaDevice";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaDeviceControl(pub ::windows::core::IInspectable);
impl MediaDeviceControl {
    pub fn Capabilities(&self) -> ::windows::core::Result<MediaDeviceControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControlCapabilities>(result__)
        }
    }
    pub fn TryGetValue(&self, value: &mut f64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetValue(&self, value: f64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TryGetAuto(&self, value: &mut bool) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetAuto(&self, value: bool) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaDeviceControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.MediaDeviceControl;{efa8dfa9-6f75-4863-ba0b-583f3036b4de})");
}
unsafe impl ::windows::core::Interface for MediaDeviceControl {
    type Vtable = IMediaDeviceControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8dfa9_6f75_4863_ba0b_583f3036b4de);
}
impl ::windows::core::RuntimeName for MediaDeviceControl {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControl";
}
impl ::core::convert::From<MediaDeviceControl> for ::windows::core::IUnknown {
    fn from(value: MediaDeviceControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaDeviceControl> for ::windows::core::IUnknown {
    fn from(value: &MediaDeviceControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaDeviceControl> for ::windows::core::IInspectable {
    fn from(value: MediaDeviceControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaDeviceControl> for ::windows::core::IInspectable {
    fn from(value: &MediaDeviceControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaDeviceControlCapabilities(pub ::windows::core::IInspectable);
impl MediaDeviceControlCapabilities {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn Default(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn AutoModeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaDeviceControlCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.MediaDeviceControlCapabilities;{23005816-eb85-43e2-b92b-8240d5ee70ec})");
}
unsafe impl ::windows::core::Interface for MediaDeviceControlCapabilities {
    type Vtable = IMediaDeviceControlCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23005816_eb85_43e2_b92b_8240d5ee70ec);
}
impl ::windows::core::RuntimeName for MediaDeviceControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControlCapabilities";
}
impl ::core::convert::From<MediaDeviceControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: MediaDeviceControlCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaDeviceControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: &MediaDeviceControlCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaDeviceControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaDeviceControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaDeviceControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: MediaDeviceControlCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaDeviceControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: &MediaDeviceControlCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaDeviceControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaDeviceControlCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ModuleCommandResult(pub ::windows::core::IInspectable);
impl ModuleCommandResult {
    pub fn Status(&self) -> ::windows::core::Result<SendCommandStatus> {
        let this = self;
        unsafe {
            let mut result__: SendCommandStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SendCommandStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ModuleCommandResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ModuleCommandResult;{520d1eb4-1374-4c7d-b1e4-39dcdf3eae4e})");
}
unsafe impl ::windows::core::Interface for ModuleCommandResult {
    type Vtable = IModuleCommandResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520d1eb4_1374_4c7d_b1e4_39dcdf3eae4e);
}
impl ::windows::core::RuntimeName for ModuleCommandResult {
    const NAME: &'static str = "Windows.Media.Devices.ModuleCommandResult";
}
impl ::core::convert::From<ModuleCommandResult> for ::windows::core::IUnknown {
    fn from(value: ModuleCommandResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ModuleCommandResult> for ::windows::core::IUnknown {
    fn from(value: &ModuleCommandResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ModuleCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ModuleCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ModuleCommandResult> for ::windows::core::IInspectable {
    fn from(value: ModuleCommandResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ModuleCommandResult> for ::windows::core::IInspectable {
    fn from(value: &ModuleCommandResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ModuleCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ModuleCommandResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OpticalImageStabilizationControl(pub ::windows::core::IInspectable);
impl OpticalImageStabilizationControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<OpticalImageStabilizationMode> {
        let this = self;
        unsafe {
            let mut result__: OpticalImageStabilizationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OpticalImageStabilizationMode>(result__)
        }
    }
    pub fn SetMode(&self, value: OpticalImageStabilizationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for OpticalImageStabilizationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.OpticalImageStabilizationControl;{bfad9c1d-00bc-423b-8eb2-a0178ca94247})");
}
unsafe impl ::windows::core::Interface for OpticalImageStabilizationControl {
    type Vtable = IOpticalImageStabilizationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfad9c1d_00bc_423b_8eb2_a0178ca94247);
}
impl ::windows::core::RuntimeName for OpticalImageStabilizationControl {
    const NAME: &'static str = "Windows.Media.Devices.OpticalImageStabilizationControl";
}
impl ::core::convert::From<OpticalImageStabilizationControl> for ::windows::core::IUnknown {
    fn from(value: OpticalImageStabilizationControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OpticalImageStabilizationControl> for ::windows::core::IUnknown {
    fn from(value: &OpticalImageStabilizationControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OpticalImageStabilizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OpticalImageStabilizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OpticalImageStabilizationControl> for ::windows::core::IInspectable {
    fn from(value: OpticalImageStabilizationControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OpticalImageStabilizationControl> for ::windows::core::IInspectable {
    fn from(value: &OpticalImageStabilizationControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OpticalImageStabilizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OpticalImageStabilizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OpticalImageStabilizationControl {}
unsafe impl ::core::marker::Sync for OpticalImageStabilizationControl {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OpticalImageStabilizationMode(pub i32);
impl OpticalImageStabilizationMode {
    pub const Off: OpticalImageStabilizationMode = OpticalImageStabilizationMode(0i32);
    pub const On: OpticalImageStabilizationMode = OpticalImageStabilizationMode(1i32);
    pub const Auto: OpticalImageStabilizationMode = OpticalImageStabilizationMode(2i32);
}
impl ::core::convert::From<i32> for OpticalImageStabilizationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OpticalImageStabilizationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for OpticalImageStabilizationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.OpticalImageStabilizationMode;i4)");
}
impl ::windows::core::DefaultType for OpticalImageStabilizationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PanelBasedOptimizationControl(pub ::windows::core::IInspectable);
impl PanelBasedOptimizationControl {
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Panel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Enumeration::Panel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::Panel>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPanel(&self, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PanelBasedOptimizationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.PanelBasedOptimizationControl;{33323223-6247-5419-a5a4-3d808645d917})");
}
unsafe impl ::windows::core::Interface for PanelBasedOptimizationControl {
    type Vtable = IPanelBasedOptimizationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33323223_6247_5419_a5a4_3d808645d917);
}
impl ::windows::core::RuntimeName for PanelBasedOptimizationControl {
    const NAME: &'static str = "Windows.Media.Devices.PanelBasedOptimizationControl";
}
impl ::core::convert::From<PanelBasedOptimizationControl> for ::windows::core::IUnknown {
    fn from(value: PanelBasedOptimizationControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PanelBasedOptimizationControl> for ::windows::core::IUnknown {
    fn from(value: &PanelBasedOptimizationControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PanelBasedOptimizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PanelBasedOptimizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PanelBasedOptimizationControl> for ::windows::core::IInspectable {
    fn from(value: PanelBasedOptimizationControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PanelBasedOptimizationControl> for ::windows::core::IInspectable {
    fn from(value: &PanelBasedOptimizationControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PanelBasedOptimizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PanelBasedOptimizationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PanelBasedOptimizationControl {}
unsafe impl ::core::marker::Sync for PanelBasedOptimizationControl {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoConfirmationControl(pub ::windows::core::IInspectable);
impl PhotoConfirmationControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PixelFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::MediaProperties::MediaPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetPixelFormat(&self, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), format).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoConfirmationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.PhotoConfirmationControl;{c8f3f363-ff5e-4582-a9a8-0550f85a4a76})");
}
unsafe impl ::windows::core::Interface for PhotoConfirmationControl {
    type Vtable = IPhotoConfirmationControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8f3f363_ff5e_4582_a9a8_0550f85a4a76);
}
impl ::windows::core::RuntimeName for PhotoConfirmationControl {
    const NAME: &'static str = "Windows.Media.Devices.PhotoConfirmationControl";
}
impl ::core::convert::From<PhotoConfirmationControl> for ::windows::core::IUnknown {
    fn from(value: PhotoConfirmationControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoConfirmationControl> for ::windows::core::IUnknown {
    fn from(value: &PhotoConfirmationControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoConfirmationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoConfirmationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoConfirmationControl> for ::windows::core::IInspectable {
    fn from(value: PhotoConfirmationControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoConfirmationControl> for ::windows::core::IInspectable {
    fn from(value: &PhotoConfirmationControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoConfirmationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoConfirmationControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RedialRequestedEventArgs(pub ::windows::core::IInspectable);
impl RedialRequestedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RedialRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RedialRequestedEventArgs;{7eb55209-76ab-4c31-b40e-4b58379d580c})");
}
unsafe impl ::windows::core::Interface for RedialRequestedEventArgs {
    type Vtable = IRedialRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eb55209_76ab_4c31_b40e_4b58379d580c);
}
impl ::windows::core::RuntimeName for RedialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.RedialRequestedEventArgs";
}
impl ::core::convert::From<RedialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RedialRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RedialRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RedialRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RedialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RedialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RedialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RedialRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RedialRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RedialRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RedialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RedialRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RedialRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RedialRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RedialRequestedEventHandler(::windows::core::IUnknown);
impl RedialRequestedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<RedialRequestedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = RedialRequestedEventHandler_box::<F> { vtable: &RedialRequestedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, CallControl>, Param1: ::windows::core::IntoParam<'a, RedialRequestedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RedialRequestedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({baf257d1-4ebd-4b84-9f47-6ec43d75d8b1})");
}
unsafe impl ::windows::core::Interface for RedialRequestedEventHandler {
    type Vtable = RedialRequestedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbaf257d1_4ebd_4b84_9f47_6ec43d75d8b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct RedialRequestedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct RedialRequestedEventHandler_box<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<RedialRequestedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const RedialRequestedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CallControl>, &::core::option::Option<RedialRequestedEventArgs>) -> ::windows::core::Result<()> + 'static> RedialRequestedEventHandler_box<F> {
    const VTABLE: RedialRequestedEventHandler_abi = RedialRequestedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RedialRequestedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <CallControl as ::windows::core::Abi>::Abi as *const <CallControl as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <RedialRequestedEventArgs as ::windows::core::Abi>::Abi as *const <RedialRequestedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RegionOfInterest(pub ::windows::core::IInspectable);
impl RegionOfInterest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RegionOfInterest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AutoFocusEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoFocusEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AutoWhiteBalanceEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoWhiteBalanceEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AutoExposureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoExposureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<RegionOfInterestType> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__: RegionOfInterestType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RegionOfInterestType>(result__)
        }
    }
    pub fn SetType(&self, value: RegionOfInterestType) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BoundsNormalized(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetBoundsNormalized(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Weight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetWeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RegionOfInterest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RegionOfInterest;{e5ecc834-ce66-4e05-a78f-cf391a5ec2d1})");
}
unsafe impl ::windows::core::Interface for RegionOfInterest {
    type Vtable = IRegionOfInterest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5ecc834_ce66_4e05_a78f_cf391a5ec2d1);
}
impl ::windows::core::RuntimeName for RegionOfInterest {
    const NAME: &'static str = "Windows.Media.Devices.RegionOfInterest";
}
impl ::core::convert::From<RegionOfInterest> for ::windows::core::IUnknown {
    fn from(value: RegionOfInterest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RegionOfInterest> for ::windows::core::IUnknown {
    fn from(value: &RegionOfInterest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RegionOfInterest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RegionOfInterest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RegionOfInterest> for ::windows::core::IInspectable {
    fn from(value: RegionOfInterest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RegionOfInterest> for ::windows::core::IInspectable {
    fn from(value: &RegionOfInterest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RegionOfInterest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RegionOfInterest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RegionOfInterest {}
unsafe impl ::core::marker::Sync for RegionOfInterest {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RegionOfInterestType(pub i32);
impl RegionOfInterestType {
    pub const Unknown: RegionOfInterestType = RegionOfInterestType(0i32);
    pub const Face: RegionOfInterestType = RegionOfInterestType(1i32);
}
impl ::core::convert::From<i32> for RegionOfInterestType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RegionOfInterestType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RegionOfInterestType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.RegionOfInterestType;i4)");
}
impl ::windows::core::DefaultType for RegionOfInterestType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RegionsOfInterestControl(pub ::windows::core::IInspectable);
impl RegionsOfInterestControl {
    pub fn MaxRegions(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetRegionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<RegionOfInterest>>>(&self, regions: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), regions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetRegionsWithLockAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<RegionOfInterest>>>(&self, regions: Param0, lockvalues: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), regions.into_param().abi(), lockvalues, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClearRegionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn AutoFocusSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AutoWhiteBalanceSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AutoExposureSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RegionsOfInterestControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RegionsOfInterestControl;{c323f527-ab0b-4558-8b5b-df5693db0378})");
}
unsafe impl ::windows::core::Interface for RegionsOfInterestControl {
    type Vtable = IRegionsOfInterestControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc323f527_ab0b_4558_8b5b_df5693db0378);
}
impl ::windows::core::RuntimeName for RegionsOfInterestControl {
    const NAME: &'static str = "Windows.Media.Devices.RegionsOfInterestControl";
}
impl ::core::convert::From<RegionsOfInterestControl> for ::windows::core::IUnknown {
    fn from(value: RegionsOfInterestControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RegionsOfInterestControl> for ::windows::core::IUnknown {
    fn from(value: &RegionsOfInterestControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RegionsOfInterestControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RegionsOfInterestControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RegionsOfInterestControl> for ::windows::core::IInspectable {
    fn from(value: RegionsOfInterestControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RegionsOfInterestControl> for ::windows::core::IInspectable {
    fn from(value: &RegionsOfInterestControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RegionsOfInterestControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RegionsOfInterestControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneModeControl(pub ::windows::core::IInspectable);
impl SceneModeControl {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<CaptureSceneMode> {
        let this = self;
        unsafe {
            let mut result__: CaptureSceneMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CaptureSceneMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, scenemode: CaptureSceneMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), scenemode, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SceneModeControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.SceneModeControl;{d48e5af7-8d59-4854-8c62-12c70ba89b7c})");
}
unsafe impl ::windows::core::Interface for SceneModeControl {
    type Vtable = ISceneModeControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd48e5af7_8d59_4854_8c62_12c70ba89b7c);
}
impl ::windows::core::RuntimeName for SceneModeControl {
    const NAME: &'static str = "Windows.Media.Devices.SceneModeControl";
}
impl ::core::convert::From<SceneModeControl> for ::windows::core::IUnknown {
    fn from(value: SceneModeControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneModeControl> for ::windows::core::IUnknown {
    fn from(value: &SceneModeControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SceneModeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SceneModeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneModeControl> for ::windows::core::IInspectable {
    fn from(value: SceneModeControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneModeControl> for ::windows::core::IInspectable {
    fn from(value: &SceneModeControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SceneModeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SceneModeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: SendCommandStatus = SendCommandStatus(0i32);
    pub const DeviceNotAvailable: SendCommandStatus = SendCommandStatus(1i32);
}
impl ::core::convert::From<i32> for SendCommandStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SendCommandStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SendCommandStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.SendCommandStatus;i4)");
}
impl ::windows::core::DefaultType for SendCommandStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TelephonyKey(pub i32);
impl TelephonyKey {
    pub const D0: TelephonyKey = TelephonyKey(0i32);
    pub const D1: TelephonyKey = TelephonyKey(1i32);
    pub const D2: TelephonyKey = TelephonyKey(2i32);
    pub const D3: TelephonyKey = TelephonyKey(3i32);
    pub const D4: TelephonyKey = TelephonyKey(4i32);
    pub const D5: TelephonyKey = TelephonyKey(5i32);
    pub const D6: TelephonyKey = TelephonyKey(6i32);
    pub const D7: TelephonyKey = TelephonyKey(7i32);
    pub const D8: TelephonyKey = TelephonyKey(8i32);
    pub const D9: TelephonyKey = TelephonyKey(9i32);
    pub const Star: TelephonyKey = TelephonyKey(10i32);
    pub const Pound: TelephonyKey = TelephonyKey(11i32);
    pub const A: TelephonyKey = TelephonyKey(12i32);
    pub const B: TelephonyKey = TelephonyKey(13i32);
    pub const C: TelephonyKey = TelephonyKey(14i32);
    pub const D: TelephonyKey = TelephonyKey(15i32);
}
impl ::core::convert::From<i32> for TelephonyKey {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TelephonyKey {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TelephonyKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.TelephonyKey;i4)");
}
impl ::windows::core::DefaultType for TelephonyKey {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TorchControl(pub ::windows::core::IInspectable);
impl TorchControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TorchControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.TorchControl;{a6053665-8250-416c-919a-724296afa306})");
}
unsafe impl ::windows::core::Interface for TorchControl {
    type Vtable = ITorchControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6053665_8250_416c_919a_724296afa306);
}
impl ::windows::core::RuntimeName for TorchControl {
    const NAME: &'static str = "Windows.Media.Devices.TorchControl";
}
impl ::core::convert::From<TorchControl> for ::windows::core::IUnknown {
    fn from(value: TorchControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TorchControl> for ::windows::core::IUnknown {
    fn from(value: &TorchControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TorchControl> for ::windows::core::IInspectable {
    fn from(value: TorchControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TorchControl> for ::windows::core::IInspectable {
    fn from(value: &TorchControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TorchControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoDeviceController(pub ::windows::core::IInspectable);
impl VideoDeviceController {
    pub fn Brightness(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Hue(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn WhiteBalance(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn BacklightCompensation(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Pan(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Tilt(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Zoom(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Roll(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Exposure(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    pub fn Focus(&self) -> ::windows::core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDeviceControl>(result__)
        }
    }
    #[cfg(feature = "Media_Capture")]
    pub fn TrySetPowerlineFrequency(&self, value: super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Media_Capture")]
    pub fn TryGetPowerlineFrequency(&self, value: &mut super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeviceProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, propertyid: Param0, propertyvalue: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), propertyvalue.into_param().abi()).ok() }
    }
    pub fn GetDeviceProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyid: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn LowLagPhotoSequence(&self) -> ::windows::core::Result<LowLagPhotoSequenceControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LowLagPhotoSequenceControl>(result__)
        }
    }
    pub fn LowLagPhoto(&self) -> ::windows::core::Result<LowLagPhotoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LowLagPhotoControl>(result__)
        }
    }
    pub fn SceneModeControl(&self) -> ::windows::core::Result<SceneModeControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneModeControl>(result__)
        }
    }
    pub fn TorchControl(&self) -> ::windows::core::Result<TorchControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TorchControl>(result__)
        }
    }
    pub fn FlashControl(&self) -> ::windows::core::Result<FlashControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FlashControl>(result__)
        }
    }
    pub fn WhiteBalanceControl(&self) -> ::windows::core::Result<WhiteBalanceControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WhiteBalanceControl>(result__)
        }
    }
    pub fn ExposureControl(&self) -> ::windows::core::Result<ExposureControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExposureControl>(result__)
        }
    }
    pub fn FocusControl(&self) -> ::windows::core::Result<FocusControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusControl>(result__)
        }
    }
    pub fn ExposureCompensationControl(&self) -> ::windows::core::Result<ExposureCompensationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExposureCompensationControl>(result__)
        }
    }
    pub fn IsoSpeedControl(&self) -> ::windows::core::Result<IsoSpeedControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IsoSpeedControl>(result__)
        }
    }
    pub fn RegionsOfInterestControl(&self) -> ::windows::core::Result<RegionsOfInterestControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RegionsOfInterestControl>(result__)
        }
    }
    pub fn PrimaryUse(&self) -> ::windows::core::Result<CaptureUse> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__: CaptureUse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CaptureUse>(result__)
        }
    }
    pub fn SetPrimaryUse(&self, value: CaptureUse) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Devices_Core")]
    pub fn VariablePhotoSequenceController(&self) -> ::windows::core::Result<Core::VariablePhotoSequenceController> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Core::VariablePhotoSequenceController>(result__)
        }
    }
    pub fn PhotoConfirmationControl(&self) -> ::windows::core::Result<PhotoConfirmationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoConfirmationControl>(result__)
        }
    }
    pub fn ZoomControl(&self) -> ::windows::core::Result<ZoomControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ZoomControl>(result__)
        }
    }
    pub fn ExposurePriorityVideoControl(&self) -> ::windows::core::Result<ExposurePriorityVideoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExposurePriorityVideoControl>(result__)
        }
    }
    pub fn DesiredOptimization(&self) -> ::windows::core::Result<MediaCaptureOptimization> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__: MediaCaptureOptimization = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureOptimization>(result__)
        }
    }
    pub fn SetDesiredOptimization(&self, value: MediaCaptureOptimization) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HdrVideoControl(&self) -> ::windows::core::Result<HdrVideoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HdrVideoControl>(result__)
        }
    }
    pub fn OpticalImageStabilizationControl(&self) -> ::windows::core::Result<OpticalImageStabilizationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OpticalImageStabilizationControl>(result__)
        }
    }
    pub fn AdvancedPhotoControl(&self) -> ::windows::core::Result<AdvancedPhotoControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdvancedPhotoControl>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>(result__)
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::MediaProperties::IMediaEncodingProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mediastreamtype, mediaencodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDevicePropertyById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, propertyid: Param0, maxpropertyvaluesize: Param1) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), maxpropertyvaluesize.into_param().abi(), &mut result__).from_abi::<VideoDeviceControllerGetDevicePropertyResult>(result__)
        }
    }
    pub fn SetDevicePropertyById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, propertyid: Param0, propertyvalue: Param1) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__: VideoDeviceControllerSetDevicePropertyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), propertyid.into_param().abi(), propertyvalue.into_param().abi(), &mut result__).from_abi::<VideoDeviceControllerSetDevicePropertyStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDevicePropertyByExtendedId<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: Param1) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), maxpropertyvaluesize.into_param().abi(), &mut result__).from_abi::<VideoDeviceControllerGetDevicePropertyResult>(result__)
        }
    }
    pub fn SetDevicePropertyByExtendedId(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__: VideoDeviceControllerSetDevicePropertyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), propertyvalue.len() as u32, ::core::mem::transmute(propertyvalue.as_ptr()), &mut result__).from_abi::<VideoDeviceControllerSetDevicePropertyStatus>(result__)
        }
    }
    pub fn VideoTemporalDenoisingControl(&self) -> ::windows::core::Result<VideoTemporalDenoisingControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoTemporalDenoisingControl>(result__)
        }
    }
    pub fn InfraredTorchControl(&self) -> ::windows::core::Result<InfraredTorchControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InfraredTorchControl>(result__)
        }
    }
    pub fn PanelBasedOptimizationControl(&self) -> ::windows::core::Result<PanelBasedOptimizationControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController8>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PanelBasedOptimizationControl>(result__)
        }
    }
    pub fn DigitalWindowControl(&self) -> ::windows::core::Result<DigitalWindowControl> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController9>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DigitalWindowControl>(result__)
        }
    }
    pub fn CameraOcclusionInfo(&self) -> ::windows::core::Result<CameraOcclusionInfo> {
        let this = &::windows::core::Interface::cast::<IAdvancedVideoCaptureDeviceController10>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraOcclusionInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoDeviceController;{99555575-2e2e-40b8-b6c7-f82d10013210})");
}
unsafe impl ::windows::core::Interface for VideoDeviceController {
    type Vtable = IVideoDeviceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99555575_2e2e_40b8_b6c7_f82d10013210);
}
impl ::windows::core::RuntimeName for VideoDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceController";
}
impl ::core::convert::From<VideoDeviceController> for ::windows::core::IUnknown {
    fn from(value: VideoDeviceController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoDeviceController> for ::windows::core::IUnknown {
    fn from(value: &VideoDeviceController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoDeviceController> for ::windows::core::IInspectable {
    fn from(value: VideoDeviceController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoDeviceController> for ::windows::core::IInspectable {
    fn from(value: &VideoDeviceController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VideoDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoDeviceController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoDeviceController> for IMediaDeviceController {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoDeviceController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaDeviceController> for VideoDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaDeviceController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaDeviceController> for &VideoDeviceController {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaDeviceController> {
        ::core::convert::TryInto::<IMediaDeviceController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoDeviceControllerGetDevicePropertyResult(pub ::windows::core::IInspectable);
impl VideoDeviceControllerGetDevicePropertyResult {
    pub fn Status(&self) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyStatus> {
        let this = self;
        unsafe {
            let mut result__: VideoDeviceControllerGetDevicePropertyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoDeviceControllerGetDevicePropertyStatus>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceControllerGetDevicePropertyResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult;{c5d88395-6ed5-4790-8b5d-0ef13935d0f8})");
}
unsafe impl ::windows::core::Interface for VideoDeviceControllerGetDevicePropertyResult {
    type Vtable = IVideoDeviceControllerGetDevicePropertyResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5d88395_6ed5_4790_8b5d_0ef13935d0f8);
}
impl ::windows::core::RuntimeName for VideoDeviceControllerGetDevicePropertyResult {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult";
}
impl ::core::convert::From<VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IUnknown {
    fn from(value: VideoDeviceControllerGetDevicePropertyResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IUnknown {
    fn from(value: &VideoDeviceControllerGetDevicePropertyResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoDeviceControllerGetDevicePropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoDeviceControllerGetDevicePropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IInspectable {
    fn from(value: VideoDeviceControllerGetDevicePropertyResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoDeviceControllerGetDevicePropertyResult> for ::windows::core::IInspectable {
    fn from(value: &VideoDeviceControllerGetDevicePropertyResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoDeviceControllerGetDevicePropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoDeviceControllerGetDevicePropertyResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VideoDeviceControllerGetDevicePropertyResult {}
unsafe impl ::core::marker::Sync for VideoDeviceControllerGetDevicePropertyResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerGetDevicePropertyStatus {
    pub const Success: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(0i32);
    pub const UnknownFailure: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(1i32);
    pub const BufferTooSmall: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(2i32);
    pub const NotSupported: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(3i32);
    pub const DeviceNotAvailable: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(4i32);
    pub const MaxPropertyValueSizeTooSmall: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(5i32);
    pub const MaxPropertyValueSizeRequired: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(6i32);
}
impl ::core::convert::From<i32> for VideoDeviceControllerGetDevicePropertyStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VideoDeviceControllerGetDevicePropertyStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceControllerGetDevicePropertyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyStatus;i4)");
}
impl ::windows::core::DefaultType for VideoDeviceControllerGetDevicePropertyStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoDeviceControllerSetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerSetDevicePropertyStatus {
    pub const Success: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(0i32);
    pub const UnknownFailure: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(1i32);
    pub const NotSupported: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(2i32);
    pub const InvalidValue: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(3i32);
    pub const DeviceNotAvailable: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(4i32);
    pub const NotInControl: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(5i32);
}
impl ::core::convert::From<i32> for VideoDeviceControllerSetDevicePropertyStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VideoDeviceControllerSetDevicePropertyStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceControllerSetDevicePropertyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerSetDevicePropertyStatus;i4)");
}
impl ::windows::core::DefaultType for VideoDeviceControllerSetDevicePropertyStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoTemporalDenoisingControl(pub ::windows::core::IInspectable);
impl VideoTemporalDenoisingControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<VideoTemporalDenoisingMode> {
        let this = self;
        unsafe {
            let mut result__: VideoTemporalDenoisingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoTemporalDenoisingMode>(result__)
        }
    }
    pub fn SetMode(&self, value: VideoTemporalDenoisingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTemporalDenoisingControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoTemporalDenoisingControl;{7ab34735-3e2a-4a32-baff-4358c4fbdd57})");
}
unsafe impl ::windows::core::Interface for VideoTemporalDenoisingControl {
    type Vtable = IVideoTemporalDenoisingControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ab34735_3e2a_4a32_baff_4358c4fbdd57);
}
impl ::windows::core::RuntimeName for VideoTemporalDenoisingControl {
    const NAME: &'static str = "Windows.Media.Devices.VideoTemporalDenoisingControl";
}
impl ::core::convert::From<VideoTemporalDenoisingControl> for ::windows::core::IUnknown {
    fn from(value: VideoTemporalDenoisingControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoTemporalDenoisingControl> for ::windows::core::IUnknown {
    fn from(value: &VideoTemporalDenoisingControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoTemporalDenoisingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoTemporalDenoisingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoTemporalDenoisingControl> for ::windows::core::IInspectable {
    fn from(value: VideoTemporalDenoisingControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoTemporalDenoisingControl> for ::windows::core::IInspectable {
    fn from(value: &VideoTemporalDenoisingControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoTemporalDenoisingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoTemporalDenoisingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VideoTemporalDenoisingControl {}
unsafe impl ::core::marker::Sync for VideoTemporalDenoisingControl {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: VideoTemporalDenoisingMode = VideoTemporalDenoisingMode(0i32);
    pub const On: VideoTemporalDenoisingMode = VideoTemporalDenoisingMode(1i32);
    pub const Auto: VideoTemporalDenoisingMode = VideoTemporalDenoisingMode(2i32);
}
impl ::core::convert::From<i32> for VideoTemporalDenoisingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VideoTemporalDenoisingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VideoTemporalDenoisingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoTemporalDenoisingMode;i4)");
}
impl ::windows::core::DefaultType for VideoTemporalDenoisingMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WhiteBalanceControl(pub ::windows::core::IInspectable);
impl WhiteBalanceControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Preset(&self) -> ::windows::core::Result<ColorTemperaturePreset> {
        let this = self;
        unsafe {
            let mut result__: ColorTemperaturePreset = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ColorTemperaturePreset>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPresetAsync(&self, preset: ColorTemperaturePreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), preset, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, temperature: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), temperature, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WhiteBalanceControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.WhiteBalanceControl;{781f047e-7162-49c8-a8f9-9481c565363e})");
}
unsafe impl ::windows::core::Interface for WhiteBalanceControl {
    type Vtable = IWhiteBalanceControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781f047e_7162_49c8_a8f9_9481c565363e);
}
impl ::windows::core::RuntimeName for WhiteBalanceControl {
    const NAME: &'static str = "Windows.Media.Devices.WhiteBalanceControl";
}
impl ::core::convert::From<WhiteBalanceControl> for ::windows::core::IUnknown {
    fn from(value: WhiteBalanceControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WhiteBalanceControl> for ::windows::core::IUnknown {
    fn from(value: &WhiteBalanceControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WhiteBalanceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WhiteBalanceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WhiteBalanceControl> for ::windows::core::IInspectable {
    fn from(value: WhiteBalanceControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WhiteBalanceControl> for ::windows::core::IInspectable {
    fn from(value: &WhiteBalanceControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WhiteBalanceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WhiteBalanceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ZoomControl(pub ::windows::core::IInspectable);
impl ZoomControl {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>> {
        let this = &::windows::core::Interface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>>(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode> {
        let this = &::windows::core::Interface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__: ZoomTransitionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ZoomTransitionMode>(result__)
        }
    }
    pub fn Configure<'a, Param0: ::windows::core::IntoParam<'a, ZoomSettings>>(&self, settings: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IZoomControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), settings.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ZoomControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ZoomControl;{3a1e0b12-32da-4c17-bfd7-8d0c73c8f5a5})");
}
unsafe impl ::windows::core::Interface for ZoomControl {
    type Vtable = IZoomControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a1e0b12_32da_4c17_bfd7_8d0c73c8f5a5);
}
impl ::windows::core::RuntimeName for ZoomControl {
    const NAME: &'static str = "Windows.Media.Devices.ZoomControl";
}
impl ::core::convert::From<ZoomControl> for ::windows::core::IUnknown {
    fn from(value: ZoomControl) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ZoomControl> for ::windows::core::IUnknown {
    fn from(value: &ZoomControl) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ZoomControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ZoomControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ZoomControl> for ::windows::core::IInspectable {
    fn from(value: ZoomControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ZoomControl> for ::windows::core::IInspectable {
    fn from(value: &ZoomControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ZoomControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ZoomControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ZoomSettings(pub ::windows::core::IInspectable);
impl ZoomSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ZoomSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode> {
        let this = self;
        unsafe {
            let mut result__: ZoomTransitionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ZoomTransitionMode>(result__)
        }
    }
    pub fn SetMode(&self, value: ZoomTransitionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ZoomSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ZoomSettings;{6ad66b24-14b4-4bfd-b18f-88fe24463b52})");
}
unsafe impl ::windows::core::Interface for ZoomSettings {
    type Vtable = IZoomSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ad66b24_14b4_4bfd_b18f_88fe24463b52);
}
impl ::windows::core::RuntimeName for ZoomSettings {
    const NAME: &'static str = "Windows.Media.Devices.ZoomSettings";
}
impl ::core::convert::From<ZoomSettings> for ::windows::core::IUnknown {
    fn from(value: ZoomSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ZoomSettings> for ::windows::core::IUnknown {
    fn from(value: &ZoomSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ZoomSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ZoomSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ZoomSettings> for ::windows::core::IInspectable {
    fn from(value: ZoomSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ZoomSettings> for ::windows::core::IInspectable {
    fn from(value: &ZoomSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ZoomSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ZoomSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ZoomSettings {}
unsafe impl ::core::marker::Sync for ZoomSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ZoomTransitionMode(pub i32);
impl ZoomTransitionMode {
    pub const Auto: ZoomTransitionMode = ZoomTransitionMode(0i32);
    pub const Direct: ZoomTransitionMode = ZoomTransitionMode(1i32);
    pub const Smooth: ZoomTransitionMode = ZoomTransitionMode(2i32);
}
impl ::core::convert::From<i32> for ZoomTransitionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ZoomTransitionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ZoomTransitionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ZoomTransitionMode;i4)");
}
impl ::windows::core::DefaultType for ZoomTransitionMode {
    type DefaultType = Self;
}
