#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IApoAcousticEchoCancellation(::windows::core::IUnknown);
impl IApoAcousticEchoCancellation {}
::windows::imp::interface_hierarchy!(IApoAcousticEchoCancellation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IApoAcousticEchoCancellation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAcousticEchoCancellation {}
impl ::core::fmt::Debug for IApoAcousticEchoCancellation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAcousticEchoCancellation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApoAcousticEchoCancellation {
    type Vtable = IApoAcousticEchoCancellation_Vtbl;
}
impl ::core::clone::Clone for IApoAcousticEchoCancellation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApoAcousticEchoCancellation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25385759_3236_4101_a943_25693dfb5d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAcousticEchoCancellation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IApoAuxiliaryInputConfiguration(::windows::core::IUnknown);
impl IApoAuxiliaryInputConfiguration {
    pub unsafe fn AddAuxiliaryInput(&self, dwinputid: u32, pbydata: &[u8], pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAuxiliaryInput)(::windows::core::Interface::as_raw(self), dwinputid, pbydata.len() as _, ::core::mem::transmute(pbydata.as_ptr()), pinputconnection).ok()
    }
    pub unsafe fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAuxiliaryInput)(::windows::core::Interface::as_raw(self), dwinputid).ok()
    }
    pub unsafe fn IsInputFormatSupported<P0>(&self, prequestedinputformat: P0) -> ::windows::core::Result<IAudioMediaType>
    where
        P0: ::windows::core::IntoParam<IAudioMediaType>,
    {
        let mut result__ = ::windows::core::zeroed::<IAudioMediaType>();
        (::windows::core::Interface::vtable(self).IsInputFormatSupported)(::windows::core::Interface::as_raw(self), prequestedinputformat.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IApoAuxiliaryInputConfiguration, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IApoAuxiliaryInputConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAuxiliaryInputConfiguration {}
impl ::core::fmt::Debug for IApoAuxiliaryInputConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAuxiliaryInputConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApoAuxiliaryInputConfiguration {
    type Vtable = IApoAuxiliaryInputConfiguration_Vtbl;
}
impl ::core::clone::Clone for IApoAuxiliaryInputConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApoAuxiliaryInputConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ceb0aab_fa19_48ed_a857_87771ae1b768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputConfiguration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddAuxiliaryInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT,
    pub RemoveAuxiliaryInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows::core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestedinputformat: *mut ::core::ffi::c_void, ppsupportedinputformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IApoAuxiliaryInputRT(::windows::core::IUnknown);
impl IApoAuxiliaryInputRT {
    pub unsafe fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
        (::windows::core::Interface::vtable(self).AcceptInput)(::windows::core::Interface::as_raw(self), dwinputid, pinputconnection)
    }
}
::windows::imp::interface_hierarchy!(IApoAuxiliaryInputRT, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IApoAuxiliaryInputRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAuxiliaryInputRT {}
impl ::core::fmt::Debug for IApoAuxiliaryInputRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAuxiliaryInputRT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApoAuxiliaryInputRT {
    type Vtable = IApoAuxiliaryInputRT_Vtbl;
}
impl ::core::clone::Clone for IApoAuxiliaryInputRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApoAuxiliaryInputRT {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf851809c_c177_49a0_b1b2_b66f017943ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputRT_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AcceptInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY),
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioDeviceModulesClient(::windows::core::IUnknown);
impl IAudioDeviceModulesClient {
    pub unsafe fn SetAudioDeviceModulesManager<P0>(&self, paudiodevicemodulesmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetAudioDeviceModulesManager)(::windows::core::Interface::as_raw(self), paudiodevicemodulesmanager.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioDeviceModulesClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioDeviceModulesClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioDeviceModulesClient {}
impl ::core::fmt::Debug for IAudioDeviceModulesClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioDeviceModulesClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioDeviceModulesClient {
    type Vtable = IAudioDeviceModulesClient_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceModulesClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioDeviceModulesClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98f37dac_d0b6_49f5_896a_aa4d169a4c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAudioDeviceModulesManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioMediaType(::windows::core::IUnknown);
impl IAudioMediaType {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCompressedFormat(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsCompressedFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsEqual<P0>(&self, piaudiotype: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IAudioMediaType>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).IsEqual)(::windows::core::Interface::as_raw(self), piaudiotype.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAudioFormat(&self) -> *mut super::WAVEFORMATEX {
        (::windows::core::Interface::vtable(self).GetAudioFormat)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetUncompressedAudioFormat(&self, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUncompressedAudioFormat)(::windows::core::Interface::as_raw(self), puncompressedaudioformat).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioMediaType, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMediaType {}
impl ::core::fmt::Debug for IAudioMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMediaType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioMediaType {
    type Vtable = IAudioMediaType_Vtbl;
}
impl ::core::clone::Clone for IAudioMediaType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioMediaType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e997f73_b71f_4798_873b_ed7dfcf15b4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMediaType_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCompressedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCompressedFormat: usize,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piaudiotype: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX,
    pub GetUncompressedAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObject(::windows::core::IUnknown);
impl IAudioProcessingObject {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRegistrationProperties(&self) -> ::windows::core::Result<*mut APO_REG_PROPERTIES> {
        let mut result__ = ::windows::core::zeroed::<*mut APO_REG_PROPERTIES>();
        (::windows::core::Interface::vtable(self).GetRegistrationProperties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Initialize(&self, pbydata: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pbydata.len() as _, ::core::mem::transmute(pbydata.as_ptr())).ok()
    }
    pub unsafe fn IsInputFormatSupported<P0, P1>(&self, poppositeformat: P0, prequestedinputformat: P1) -> ::windows::core::Result<IAudioMediaType>
    where
        P0: ::windows::core::IntoParam<IAudioMediaType>,
        P1: ::windows::core::IntoParam<IAudioMediaType>,
    {
        let mut result__ = ::windows::core::zeroed::<IAudioMediaType>();
        (::windows::core::Interface::vtable(self).IsInputFormatSupported)(::windows::core::Interface::as_raw(self), poppositeformat.into_param().abi(), prequestedinputformat.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsOutputFormatSupported<P0, P1>(&self, poppositeformat: P0, prequestedoutputformat: P1) -> ::windows::core::Result<IAudioMediaType>
    where
        P0: ::windows::core::IntoParam<IAudioMediaType>,
        P1: ::windows::core::IntoParam<IAudioMediaType>,
    {
        let mut result__ = ::windows::core::zeroed::<IAudioMediaType>();
        (::windows::core::Interface::vtable(self).IsOutputFormatSupported)(::windows::core::Interface::as_raw(self), poppositeformat.into_param().abi(), prequestedoutputformat.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetInputChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioProcessingObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObject {}
impl ::core::fmt::Debug for IAudioProcessingObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObject {
    type Vtable = IAudioProcessingObject_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd7f2b29_24d0_4b5c_b177_592c39f9ca10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT,
    pub GetRegistrationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poppositeformat: *mut ::core::ffi::c_void, prequestedinputformat: *mut ::core::ffi::c_void, ppsupportedinputformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsOutputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poppositeformat: *mut ::core::ffi::c_void, prequestedoutputformat: *mut ::core::ffi::c_void, ppsupportedoutputformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInputChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectConfiguration(::windows::core::IUnknown);
impl IAudioProcessingObjectConfiguration {
    pub unsafe fn LockForProcess(&self, ppinputconnections: &[*const APO_CONNECTION_DESCRIPTOR], ppoutputconnections: &[*const APO_CONNECTION_DESCRIPTOR]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockForProcess)(::windows::core::Interface::as_raw(self), ppinputconnections.len() as _, ::core::mem::transmute(ppinputconnections.as_ptr()), ppoutputconnections.len() as _, ::core::mem::transmute(ppoutputconnections.as_ptr())).ok()
    }
    pub unsafe fn UnlockForProcess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockForProcess)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObjectConfiguration, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioProcessingObjectConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectConfiguration {}
impl ::core::fmt::Debug for IAudioProcessingObjectConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectConfiguration {
    type Vtable = IAudioProcessingObjectConfiguration_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObjectConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObjectConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e5ed805_aba6_49c3_8f9a_2b8c889c4fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectConfiguration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT,
    pub UnlockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectLoggingService(::windows::core::IUnknown);
impl IAudioProcessingObjectLoggingService {
    pub unsafe fn ApoLog<P0>(&self, level: APO_LOG_LEVEL, format: P0)
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ApoLog)(::windows::core::Interface::as_raw(self), level, format.into_param().abi())
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObjectLoggingService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioProcessingObjectLoggingService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectLoggingService {}
impl ::core::fmt::Debug for IAudioProcessingObjectLoggingService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectLoggingService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectLoggingService {
    type Vtable = IAudioProcessingObjectLoggingService_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObjectLoggingService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObjectLoggingService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x698f0107_1745_4708_95a5_d84478a62a65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectLoggingService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ApoLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: ::windows::core::PCWSTR),
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectNotifications(::windows::core::IUnknown);
impl IAudioProcessingObjectNotifications {
    pub unsafe fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApoNotificationRegistrationInfo)(::windows::core::Interface::as_raw(self), aponotifications, count).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION) {
        (::windows::core::Interface::vtable(self).HandleNotification)(::windows::core::Interface::as_raw(self), aponotification)
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObjectNotifications, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioProcessingObjectNotifications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectNotifications {}
impl ::core::fmt::Debug for IAudioProcessingObjectNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectNotifications").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectNotifications {
    type Vtable = IAudioProcessingObjectNotifications_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObjectNotifications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObjectNotifications {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56b0c76f_02fd_4b21_a52e_9f8219fc86e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectNotifications_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetApoNotificationRegistrationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub HandleNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    HandleNotification: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectNotifications2(::windows::core::IUnknown);
impl IAudioProcessingObjectNotifications2 {
    pub unsafe fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetApoNotificationRegistrationInfo)(::windows::core::Interface::as_raw(self), aponotifications, count).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION) {
        (::windows::core::Interface::vtable(self).base__.HandleNotification)(::windows::core::Interface::as_raw(self), aponotification)
    }
    pub unsafe fn GetApoNotificationRegistrationInfo2(&self, maxaponotificationtypesupported: APO_NOTIFICATION_TYPE, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApoNotificationRegistrationInfo2)(::windows::core::Interface::as_raw(self), maxaponotificationtypesupported, aponotifications, count).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObjectNotifications2, ::windows::core::IUnknown, IAudioProcessingObjectNotifications);
impl ::core::cmp::PartialEq for IAudioProcessingObjectNotifications2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectNotifications2 {}
impl ::core::fmt::Debug for IAudioProcessingObjectNotifications2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectNotifications2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectNotifications2 {
    type Vtable = IAudioProcessingObjectNotifications2_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObjectNotifications2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObjectNotifications2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca2cfbde_a9d6_4eb0_bc95_c4d026b380f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectNotifications2_Vtbl {
    pub base__: IAudioProcessingObjectNotifications_Vtbl,
    pub GetApoNotificationRegistrationInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxaponotificationtypesupported: APO_NOTIFICATION_TYPE, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectRT(::windows::core::IUnknown);
impl IAudioProcessingObjectRT {
    pub unsafe fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
        (::windows::core::Interface::vtable(self).APOProcess)(::windows::core::Interface::as_raw(self), u32numinputconnections, ppinputconnections, u32numoutputconnections, ppoutputconnections)
    }
    pub unsafe fn CalcInputFrames(&self, u32outputframecount: u32) -> u32 {
        (::windows::core::Interface::vtable(self).CalcInputFrames)(::windows::core::Interface::as_raw(self), u32outputframecount)
    }
    pub unsafe fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32 {
        (::windows::core::Interface::vtable(self).CalcOutputFrames)(::windows::core::Interface::as_raw(self), u32inputframecount)
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObjectRT, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioProcessingObjectRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectRT {}
impl ::core::fmt::Debug for IAudioProcessingObjectRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectRT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectRT {
    type Vtable = IAudioProcessingObjectRT_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObjectRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObjectRT {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e1d6a6d_ddbc_4e95_a4c7_ad64ba37846c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRT_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub APOProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY),
    pub CalcInputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32,
    pub CalcOutputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectRTQueueService(::windows::core::IUnknown);
impl IAudioProcessingObjectRTQueueService {
    pub unsafe fn GetRealTimeWorkQueue(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetRealTimeWorkQueue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObjectRTQueueService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioProcessingObjectRTQueueService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectRTQueueService {}
impl ::core::fmt::Debug for IAudioProcessingObjectRTQueueService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectRTQueueService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectRTQueueService {
    type Vtable = IAudioProcessingObjectRTQueueService_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObjectRTQueueService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObjectRTQueueService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacd65e2f_955b_4b57_b9bf_ac297bb752c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRTQueueService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRealTimeWorkQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectVBR(::windows::core::IUnknown);
impl IAudioProcessingObjectVBR {
    pub unsafe fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CalcMaxInputFrames)(::windows::core::Interface::as_raw(self), u32maxoutputframecount, &mut result__).from_abi(result__)
    }
    pub unsafe fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CalcMaxOutputFrames)(::windows::core::Interface::as_raw(self), u32maxinputframecount, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioProcessingObjectVBR, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioProcessingObjectVBR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectVBR {}
impl ::core::fmt::Debug for IAudioProcessingObjectVBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectVBR").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectVBR {
    type Vtable = IAudioProcessingObjectVBR_Vtbl;
}
impl ::core::clone::Clone for IAudioProcessingObjectVBR {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioProcessingObjectVBR {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba1db8f_78ad_49cd_9591_f79d80a17c81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectVBR_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CalcMaxInputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows::core::HRESULT,
    pub CalcMaxOutputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffects(::windows::core::IUnknown);
impl IAudioSystemEffects {}
::windows::imp::interface_hierarchy!(IAudioSystemEffects, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSystemEffects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects {}
impl ::core::fmt::Debug for IAudioSystemEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffects {
    type Vtable = IAudioSystemEffects_Vtbl;
}
impl ::core::clone::Clone for IAudioSystemEffects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSystemEffects {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fa00f27_add6_499a_8a9d_6b98521fa75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffects2(::windows::core::IUnknown);
impl IAudioSystemEffects2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectsList<P0>(&self, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).GetEffectsList)(::windows::core::Interface::as_raw(self), ppeffectsids, pceffects, event.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSystemEffects2, ::windows::core::IUnknown, IAudioSystemEffects);
impl ::core::cmp::PartialEq for IAudioSystemEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects2 {}
impl ::core::fmt::Debug for IAudioSystemEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffects2 {
    type Vtable = IAudioSystemEffects2_Vtbl;
}
impl ::core::clone::Clone for IAudioSystemEffects2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSystemEffects2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafe99d2_7436_44ce_9e0e_4d89afbfff56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects2_Vtbl {
    pub base__: IAudioSystemEffects_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEffectsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEffectsList: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffects3(::windows::core::IUnknown);
impl IAudioSystemEffects3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectsList<P0>(&self, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.GetEffectsList)(::windows::core::Interface::as_raw(self), ppeffectsids, pceffects, event.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetControllableSystemEffectsList<P0>(&self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).GetControllableSystemEffectsList)(::windows::core::Interface::as_raw(self), effects, numeffects, event.into_param().abi()).ok()
    }
    pub unsafe fn SetAudioSystemEffectState(&self, effectid: ::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAudioSystemEffectState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(effectid), state).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSystemEffects3, ::windows::core::IUnknown, IAudioSystemEffects, IAudioSystemEffects2);
impl ::core::cmp::PartialEq for IAudioSystemEffects3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects3 {}
impl ::core::fmt::Debug for IAudioSystemEffects3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffects3 {
    type Vtable = IAudioSystemEffects3_Vtbl;
}
impl ::core::clone::Clone for IAudioSystemEffects3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSystemEffects3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc58b31cd_fc6a_4255_bc1f_ad29bb0a4a17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects3_Vtbl {
    pub base__: IAudioSystemEffects2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetControllableSystemEffectsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetControllableSystemEffectsList: usize,
    pub SetAudioSystemEffectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffectsCustomFormats(::windows::core::IUnknown);
impl IAudioSystemEffectsCustomFormats {
    pub unsafe fn GetFormatCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetFormatCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFormat(&self, nformat: u32) -> ::windows::core::Result<IAudioMediaType> {
        let mut result__ = ::windows::core::zeroed::<IAudioMediaType>();
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), nformat, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFormatRepresentation(&self, nformat: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFormatRepresentation)(::windows::core::Interface::as_raw(self), nformat, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioSystemEffectsCustomFormats, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSystemEffectsCustomFormats {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsCustomFormats {}
impl ::core::fmt::Debug for IAudioSystemEffectsCustomFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsCustomFormats").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsCustomFormats {
    type Vtable = IAudioSystemEffectsCustomFormats_Vtbl;
}
impl ::core::clone::Clone for IAudioSystemEffectsCustomFormats {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSystemEffectsCustomFormats {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1176e34_bb7f_4f05_bebd_1b18a534e097);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsCustomFormats_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFormatRepresentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073919i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_UNLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073914i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_APO_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073910i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_BUFFERS_OVERLAP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073915i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073917i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_APO_CLSID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073916i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFCOUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073909i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFICIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073908i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073911i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CURVE_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073907i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_INPUTID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073906i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073912i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073918i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073913i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 13 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 15 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 18 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 17 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 16 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 14 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 20 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 19 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 13 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 10 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 7 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToBluetooth: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 30 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToCapture: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 33 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToRender: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToUsb: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 31 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 0 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Author: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 26 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EffectPackSchema_Version: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 29 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const PKEY_FX_EffectPack_Schema_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7abf23d9_727e_4d0b_86a3_dd501d260001);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 7 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Enumerator: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 23 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 4 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 10 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 9 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 8 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 6 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ObjectId: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 27 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 12 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 11 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 2 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 1 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_State: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 28 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 5 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_SupportAppLauncher: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 21 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_SupportedFormats: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 22 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 3 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_VersionMajor: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 24 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_VersionMinor: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 25 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 9 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 12 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 6 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 8 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 11 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 5 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const SID_AudioProcessingObjectLoggingService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b8008af_09f9_456e_a173_bdb58499bce7);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const SID_AudioProcessingObjectRTQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458c1a1f_6899_4c12_99ac_e2e6ac253104);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APO_BUFFER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_VALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(2i32);
impl ::core::marker::Copy for APO_BUFFER_FLAGS {}
impl ::core::clone::Clone for APO_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APO_BUFFER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APO_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
impl ::core::marker::Copy for APO_CONNECTION_BUFFER_TYPE {}
impl ::core::clone::Clone for APO_CONNECTION_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_CONNECTION_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APO_CONNECTION_BUFFER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APO_CONNECTION_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_CONNECTION_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APO_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
impl ::core::marker::Copy for APO_FLAG {}
impl ::core::clone::Clone for APO_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APO_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APO_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APO_LOG_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = APO_LOG_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = APO_LOG_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = APO_LOG_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = APO_LOG_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = APO_LOG_LEVEL(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = APO_LOG_LEVEL(5i32);
impl ::core::marker::Copy for APO_LOG_LEVEL {}
impl ::core::clone::Clone for APO_LOG_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_LOG_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APO_LOG_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APO_LOG_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_LOG_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APO_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME2: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_DEVICE_ORIENTATION: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_MICROPHONE_BOOST: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(6i32);
impl ::core::marker::Copy for APO_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for APO_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APO_NOTIFICATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APO_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIO_FLOW_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
impl ::core::marker::Copy for AUDIO_FLOW_TYPE {}
impl ::core::clone::Clone for AUDIO_FLOW_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_FLOW_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIO_FLOW_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIO_FLOW_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_FLOW_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIO_SYSTEMEFFECT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(1i32);
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT_STATE {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_SYSTEMEFFECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIO_SYSTEMEFFECT_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_SYSTEMEFFECT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVICE_ORIENTATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const DEVICE_NOT_ROTATED: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const DEVICE_ROTATED_90_DEGREES_CLOCKWISE: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const DEVICE_ROTATED_180_DEGREES_CLOCKWISE: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const DEVICE_ROTATED_270_DEGREES_CLOCKWISE: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(3i32);
impl ::core::marker::Copy for DEVICE_ORIENTATION_TYPE {}
impl ::core::clone::Clone for DEVICE_ORIENTATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_ORIENTATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEVICE_ORIENTATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEVICE_ORIENTATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_ORIENTATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAudioConstriction(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
impl ::core::marker::Copy for EAudioConstriction {}
impl ::core::clone::Clone for EAudioConstriction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAudioConstriction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EAudioConstriction {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EAudioConstriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAudioConstriction").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows::core::GUID,
}
impl ::core::marker::Copy for APOInitBaseStruct {}
impl ::core::clone::Clone for APOInitBaseStruct {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APOInitBaseStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitBaseStruct").field("cbSize", &self.cbSize).field("clsid", &self.clsid).finish()
    }
}
impl ::windows::core::TypeKind for APOInitBaseStruct {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APOInitBaseStruct {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.clsid == other.clsid
    }
}
impl ::core::cmp::Eq for APOInitBaseStruct {}
impl ::core::default::Default for APOInitBaseStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pAPOSystemEffectsProperties: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDeviceCollection>>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for APOInitSystemEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects").field("APOInit", &self.APOInit).field("pAPOEndpointProperties", &self.pAPOEndpointProperties).field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties).field("pReserved", &self.pReserved).field("pDeviceCollection", &self.pDeviceCollection).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::TypeKind for APOInitSystemEffects {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for APOInitSystemEffects {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties && self.pReserved == other.pReserved && self.pDeviceCollection == other.pDeviceCollection
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for APOInitSystemEffects {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for APOInitSystemEffects {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pAPOSystemEffectsProperties: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDeviceCollection>>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for APOInitSystemEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects2")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties)
            .field("pReserved", &self.pReserved)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .field("nSoftwareIoDeviceInCollection", &self.nSoftwareIoDeviceInCollection)
            .field("nSoftwareIoConnectorIndex", &self.nSoftwareIoConnectorIndex)
            .field("AudioProcessingMode", &self.AudioProcessingMode)
            .field("InitializeForDiscoveryOnly", &self.InitializeForDiscoveryOnly)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::TypeKind for APOInitSystemEffects2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APOInitSystemEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties && self.pReserved == other.pReserved && self.pDeviceCollection == other.pDeviceCollection && self.nSoftwareIoDeviceInCollection == other.nSoftwareIoDeviceInCollection && self.nSoftwareIoConnectorIndex == other.nSoftwareIoConnectorIndex && self.AudioProcessingMode == other.AudioProcessingMode && self.InitializeForDiscoveryOnly == other.InitializeForDiscoveryOnly
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APOInitSystemEffects2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pServiceProvider: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::System::Com::IServiceProvider>>,
    pub pDeviceCollection: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDeviceCollection>>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for APOInitSystemEffects3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects3")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pServiceProvider", &self.pServiceProvider)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .field("nSoftwareIoDeviceInCollection", &self.nSoftwareIoDeviceInCollection)
            .field("nSoftwareIoConnectorIndex", &self.nSoftwareIoConnectorIndex)
            .field("AudioProcessingMode", &self.AudioProcessingMode)
            .field("InitializeForDiscoveryOnly", &self.InitializeForDiscoveryOnly)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::TypeKind for APOInitSystemEffects3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APOInitSystemEffects3 {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pServiceProvider == other.pServiceProvider && self.pDeviceCollection == other.pDeviceCollection && self.nSoftwareIoDeviceInCollection == other.nSoftwareIoDeviceInCollection && self.nSoftwareIoConnectorIndex == other.nSoftwareIoConnectorIndex && self.AudioProcessingMode == other.AudioProcessingMode && self.InitializeForDiscoveryOnly == other.InitializeForDiscoveryOnly
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APOInitSystemEffects3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APOInitSystemEffects3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: ::std::mem::ManuallyDrop<::core::option::Option<IAudioMediaType>>,
    pub u32Signature: u32,
}
impl ::core::clone::Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_DESCRIPTOR").field("Type", &self.Type).field("pBuffer", &self.pBuffer).field("u32MaxFrameCount", &self.u32MaxFrameCount).field("pFormat", &self.pFormat).field("u32Signature", &self.u32Signature).finish()
    }
}
impl ::windows::core::TypeKind for APO_CONNECTION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pBuffer == other.pBuffer && self.u32MaxFrameCount == other.u32MaxFrameCount && self.pFormat == other.pFormat && self.u32Signature == other.u32Signature
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_DESCRIPTOR {}
impl ::core::default::Default for APO_CONNECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_PROPERTY {
    pub pBuffer: usize,
    pub u32ValidFrameCount: u32,
    pub u32BufferFlags: APO_BUFFER_FLAGS,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_PROPERTY").field("pBuffer", &self.pBuffer).field("u32ValidFrameCount", &self.u32ValidFrameCount).field("u32BufferFlags", &self.u32BufferFlags).field("u32Signature", &self.u32Signature).finish()
    }
}
impl ::windows::core::TypeKind for APO_CONNECTION_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.u32ValidFrameCount == other.u32ValidFrameCount && self.u32BufferFlags == other.u32BufferFlags && self.u32Signature == other.u32Signature
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_PROPERTY_V2 {
    pub property: APO_CONNECTION_PROPERTY,
    pub u64QPCTime: u64,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_PROPERTY_V2").field("property", &self.property).field("u64QPCTime", &self.u64QPCTime).finish()
    }
}
impl ::windows::core::TypeKind for APO_CONNECTION_PROPERTY_V2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.property == other.property && self.u64QPCTime == other.u64QPCTime
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::TypeKind for APO_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APO_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: ::std::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION>,
    pub audioEndpointPropertyChange: ::std::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION>,
    pub audioSystemEffectsPropertyChange: ::std::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION>,
    pub audioEndpointVolumeChange2: ::std::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2>,
    pub deviceOrientation: DEVICE_ORIENTATION_TYPE,
    pub audioMicrophoneBoostChange: ::std::mem::ManuallyDrop<AUDIO_MICROPHONE_BOOST_NOTIFICATION>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::TypeKind for APO_NOTIFICATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APO_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: ::std::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioEndpointPropertyChange: ::std::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioSystemEffectsPropertyChange: ::std::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioMicrophoneBoost: ::std::mem::ManuallyDrop<AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR>,
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for APO_NOTIFICATION_DESCRIPTOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows::core::GUID,
    pub Flags: APO_FLAG,
    pub szFriendlyName: [u16; 256],
    pub szCopyrightInfo: [u16; 256],
    pub u32MajorVersion: u32,
    pub u32MinorVersion: u32,
    pub u32MinInputConnections: u32,
    pub u32MaxInputConnections: u32,
    pub u32MinOutputConnections: u32,
    pub u32MaxOutputConnections: u32,
    pub u32MaxInstances: u32,
    pub u32NumAPOInterfaces: u32,
    pub iidAPOInterfaceList: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for APO_REG_PROPERTIES {}
impl ::core::clone::Clone for APO_REG_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APO_REG_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_REG_PROPERTIES")
            .field("clsid", &self.clsid)
            .field("Flags", &self.Flags)
            .field("szFriendlyName", &self.szFriendlyName)
            .field("szCopyrightInfo", &self.szCopyrightInfo)
            .field("u32MajorVersion", &self.u32MajorVersion)
            .field("u32MinorVersion", &self.u32MinorVersion)
            .field("u32MinInputConnections", &self.u32MinInputConnections)
            .field("u32MaxInputConnections", &self.u32MaxInputConnections)
            .field("u32MinOutputConnections", &self.u32MinOutputConnections)
            .field("u32MaxOutputConnections", &self.u32MaxOutputConnections)
            .field("u32MaxInstances", &self.u32MaxInstances)
            .field("u32NumAPOInterfaces", &self.u32NumAPOInterfaces)
            .field("iidAPOInterfaceList", &self.iidAPOInterfaceList)
            .finish()
    }
}
impl ::windows::core::TypeKind for APO_REG_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APO_REG_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.Flags == other.Flags && self.szFriendlyName == other.szFriendlyName && self.szCopyrightInfo == other.szCopyrightInfo && self.u32MajorVersion == other.u32MajorVersion && self.u32MinorVersion == other.u32MinorVersion && self.u32MinInputConnections == other.u32MinInputConnections && self.u32MaxInputConnections == other.u32MaxInputConnections && self.u32MinOutputConnections == other.u32MinOutputConnections && self.u32MaxOutputConnections == other.u32MaxOutputConnections && self.u32MaxInstances == other.u32MaxInstances && self.u32NumAPOInterfaces == other.u32NumAPOInterfaces && self.iidAPOInterfaceList == other.iidAPOInterfaceList
    }
}
impl ::core::cmp::Eq for APO_REG_PROPERTIES {}
impl ::core::default::Default for APO_REG_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
}
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
impl ::windows::core::TypeKind for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
    pub propertyStore: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("propertyStore", &self.propertyStore).field("propertyKey", &self.propertyKey).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::TypeKind for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.propertyStore == other.propertyStore && self.propertyKey == other.propertyKey
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
impl ::windows::core::TypeKind for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("volume", &self.volume).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.volume == other.volume
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    pub endpoint: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
    pub volume: *mut AUDIO_VOLUME_NOTIFICATION_DATA2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2").field("endpoint", &self.endpoint).field("volume", &self.volume).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.volume == other.volume
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
}
impl ::core::clone::Clone for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
impl ::windows::core::TypeKind for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    pub endpoint: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
    pub eventContext: ::windows::core::GUID,
    pub microphoneBoostEnabled: super::super::super::Foundation::BOOL,
    pub levelInDb: f32,
    pub levelMinInDb: f32,
    pub levelMaxInDb: f32,
    pub levelStepInDb: f32,
    pub muteSupported: super::super::super::Foundation::BOOL,
    pub mute: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_MICROPHONE_BOOST_NOTIFICATION").field("endpoint", &self.endpoint).field("eventContext", &self.eventContext).field("microphoneBoostEnabled", &self.microphoneBoostEnabled).field("levelInDb", &self.levelInDb).field("levelMinInDb", &self.levelMinInDb).field("levelMaxInDb", &self.levelMaxInDb).field("levelStepInDb", &self.levelStepInDb).field("muteSupported", &self.muteSupported).field("mute", &self.mute).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.eventContext == other.eventContext && self.microphoneBoostEnabled == other.microphoneBoostEnabled && self.levelInDb == other.levelInDb && self.levelMinInDb == other.levelMinInDb && self.levelMaxInDb == other.levelMaxInDb && self.levelStepInDb == other.levelStepInDb && self.muteSupported == other.muteSupported && self.mute == other.mute
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_MICROPHONE_BOOST_NOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows::core::GUID,
    pub canSetState: super::super::super::Foundation::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUDIO_SYSTEMEFFECT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.canSetState == other.canSetState && self.state == other.state
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_SYSTEMEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
    pub propertyStoreContext: ::windows::core::GUID,
}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).field("propertyStoreContext", &self.propertyStoreContext).finish()
    }
}
impl ::windows::core::TypeKind for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device && self.propertyStoreContext == other.propertyStoreContext
    }
}
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::std::mem::ManuallyDrop<::core::option::Option<super::IMMDevice>>,
    pub propertyStoreContext: ::windows::core::GUID,
    pub propertyStoreType: super::AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE,
    pub propertyStore: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("propertyStoreContext", &self.propertyStoreContext).field("propertyStoreType", &self.propertyStoreType).field("propertyStore", &self.propertyStore).field("propertyKey", &self.propertyKey).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::TypeKind for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.propertyStoreContext == other.propertyStoreContext && self.propertyStoreType == other.propertyStoreType && self.propertyStore == other.propertyStore && self.propertyKey == other.propertyKey
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA2 {
    pub notificationData: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
    pub masterVolumeInDb: f32,
    pub volumeMinInDb: f32,
    pub volumeMaxInDb: f32,
    pub volumeIncrementInDb: f32,
    pub step: u32,
    pub stepCount: u32,
    pub channelVolumesInDb: [f32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_VOLUME_NOTIFICATION_DATA2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_VOLUME_NOTIFICATION_DATA2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_VOLUME_NOTIFICATION_DATA2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_VOLUME_NOTIFICATION_DATA2").field("notificationData", &self.notificationData).field("masterVolumeInDb", &self.masterVolumeInDb).field("volumeMinInDb", &self.volumeMinInDb).field("volumeMaxInDb", &self.volumeMaxInDb).field("volumeIncrementInDb", &self.volumeIncrementInDb).field("step", &self.step).field("stepCount", &self.stepCount).field("channelVolumesInDb", &self.channelVolumesInDb).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUDIO_VOLUME_NOTIFICATION_DATA2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_VOLUME_NOTIFICATION_DATA2 {
    fn eq(&self, other: &Self) -> bool {
        self.notificationData == other.notificationData && self.masterVolumeInDb == other.masterVolumeInDb && self.volumeMinInDb == other.volumeMinInDb && self.volumeMaxInDb == other.volumeMaxInDb && self.volumeIncrementInDb == other.volumeIncrementInDb && self.step == other.step && self.stepCount == other.stepCount && self.channelVolumesInDb == other.channelVolumesInDb
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_VOLUME_NOTIFICATION_DATA2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_VOLUME_NOTIFICATION_DATA2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: ::windows::core::PWSTR,
    pub pFxProperties: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for AudioFXExtensionParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioFXExtensionParams").field("AddPageParam", &self.AddPageParam).field("pwstrEndpointID", &self.pwstrEndpointID).field("pFxProperties", &self.pFxProperties).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::TypeKind for AudioFXExtensionParams {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for AudioFXExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam && self.pwstrEndpointID == other.pwstrEndpointID && self.pFxProperties == other.pFxProperties
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for AudioFXExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows::core::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl ::core::marker::Copy for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::clone::Clone for UNCOMPRESSEDAUDIOFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNCOMPRESSEDAUDIOFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNCOMPRESSEDAUDIOFORMAT").field("guidFormatType", &self.guidFormatType).field("dwSamplesPerFrame", &self.dwSamplesPerFrame).field("dwBytesPerSampleContainer", &self.dwBytesPerSampleContainer).field("dwValidBitsPerSample", &self.dwValidBitsPerSample).field("fFramesPerSecond", &self.fFramesPerSecond).field("dwChannelMask", &self.dwChannelMask).finish()
    }
}
impl ::windows::core::TypeKind for UNCOMPRESSEDAUDIOFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for UNCOMPRESSEDAUDIOFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.guidFormatType == other.guidFormatType && self.dwSamplesPerFrame == other.dwSamplesPerFrame && self.dwBytesPerSampleContainer == other.dwBytesPerSampleContainer && self.dwValidBitsPerSample == other.dwValidBitsPerSample && self.fFramesPerSecond == other.fFramesPerSecond && self.dwChannelMask == other.dwChannelMask
    }
}
impl ::core::cmp::Eq for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::default::Default for UNCOMPRESSEDAUDIOFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type FNAPONOTIFICATIONCALLBACK = ::core::option::Option<unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
