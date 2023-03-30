#[cfg(feature = "Win32_Media_Audio_Apo")]
pub mod Apo;
#[cfg(feature = "Win32_Media_Audio_DirectMusic")]
pub mod DirectMusic;
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub mod DirectSound;
#[cfg(feature = "Win32_Media_Audio_Endpoints")]
pub mod Endpoints;
#[cfg(feature = "Win32_Media_Audio_XAudio2")]
pub mod XAudio2;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ActivateAudioInterfaceAsync<P0, P1>(deviceinterfacepath: P0, riid: *const ::windows::core::GUID, activationparams: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>, completionhandler: P1) -> ::windows::core::Result<IActivateAudioInterfaceAsyncOperation>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<IActivateAudioInterfaceCompletionHandler>,
{
    ::windows_targets::link ! ( "mmdevapi.dll""system" fn ActivateAudioInterfaceAsync ( deviceinterfacepath : ::windows::core::PCWSTR , riid : *const ::windows::core::GUID , activationparams : *const super::super::System::Com::StructuredStorage:: PROPVARIANT , completionhandler : * mut::core::ffi::c_void , activationoperation : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IActivateAudioInterfaceAsyncOperation>();
    ActivateAudioInterfaceAsync(deviceinterfacepath.into_param().abi(), riid, ::core::mem::transmute(activationparams.unwrap_or(::std::ptr::null())), completionhandler.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CoRegisterMessageFilter<P0>(lpmessagefilter: P0, lplpmessagefilter: ::core::option::Option<*mut ::core::option::Option<IMessageFilter>>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IMessageFilter>,
{
    ::windows_targets::link ! ( "ole32.dll""system" fn CoRegisterMessageFilter ( lpmessagefilter : * mut::core::ffi::c_void , lplpmessagefilter : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CoRegisterMessageFilter(lpmessagefilter.into_param().abi(), ::core::mem::transmute(lplpmessagefilter.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitor() -> ::windows::core::Result<IAudioStateMonitor> {
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateCaptureAudioStateMonitor ( audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateCaptureAudioStateMonitor(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<IAudioStateMonitor> {
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateCaptureAudioStateMonitorForCategory ( category : AUDIO_STREAM_CATEGORY , audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateCaptureAudioStateMonitorForCategory(category, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId<P0>(category: AUDIO_STREAM_CATEGORY, deviceid: P0) -> ::windows::core::Result<IAudioStateMonitor>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId ( category : AUDIO_STREAM_CATEGORY , deviceid : ::windows::core::PCWSTR , audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category, deviceid.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::core::Result<IAudioStateMonitor> {
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole ( category : AUDIO_STREAM_CATEGORY , role : ERole , audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category, role, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitor() -> ::windows::core::Result<IAudioStateMonitor> {
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateRenderAudioStateMonitor ( audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateRenderAudioStateMonitor(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<IAudioStateMonitor> {
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateRenderAudioStateMonitorForCategory ( category : AUDIO_STREAM_CATEGORY , audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateRenderAudioStateMonitorForCategory(category, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceId<P0>(category: AUDIO_STREAM_CATEGORY, deviceid: P0) -> ::windows::core::Result<IAudioStateMonitor>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceId ( category : AUDIO_STREAM_CATEGORY , deviceid : ::windows::core::PCWSTR , audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateRenderAudioStateMonitorForCategoryAndDeviceId(category, deviceid.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::core::Result<IAudioStateMonitor> {
    ::windows_targets::link ! ( "windows.media.mediacontrol.dll""system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole ( category : AUDIO_STREAM_CATEGORY , role : ERole , audiostatemonitor : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IAudioStateMonitor>();
    CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category, role, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundA<P0, P1>(pszsound: P0, hmod: P1, fdwsound: SND_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn PlaySoundA ( pszsound : ::windows::core::PCSTR , hmod : super::super::Foundation:: HMODULE , fdwsound : SND_FLAGS ) -> super::super::Foundation:: BOOL );
    PlaySoundA(pszsound.into_param().abi(), hmod.into_param().abi(), fdwsound)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundW<P0, P1>(pszsound: P0, hmod: P1, fdwsound: SND_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn PlaySoundW ( pszsound : ::windows::core::PCWSTR , hmod : super::super::Foundation:: HMODULE , fdwsound : SND_FLAGS ) -> super::super::Foundation:: BOOL );
    PlaySoundW(pszsound.into_param().abi(), hmod.into_param().abi(), fdwsound)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddA<P0, P1>(phadid: *mut isize, hinstmodule: P0, lparam: P1, dwpriority: u32, fdwadd: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverAddA ( phadid : *mut isize , hinstmodule : super::super::Foundation:: HMODULE , lparam : super::super::Foundation:: LPARAM , dwpriority : u32 , fdwadd : u32 ) -> u32 );
    acmDriverAddA(phadid, hinstmodule.into_param().abi(), lparam.into_param().abi(), dwpriority, fdwadd)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddW<P0, P1>(phadid: *mut isize, hinstmodule: P0, lparam: P1, dwpriority: u32, fdwadd: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverAddW ( phadid : *mut isize , hinstmodule : super::super::Foundation:: HMODULE , lparam : super::super::Foundation:: LPARAM , dwpriority : u32 , fdwadd : u32 ) -> u32 );
    acmDriverAddW(phadid, hinstmodule.into_param().abi(), lparam.into_param().abi(), dwpriority, fdwadd)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmDriverClose<P0>(had: P0, fdwclose: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverClose ( had : HACMDRIVER , fdwclose : u32 ) -> u32 );
    acmDriverClose(had.into_param().abi(), fdwclose)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsA<P0>(hadid: P0, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVERID>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverDetailsA ( hadid : HACMDRIVERID , padd : *mut ACMDRIVERDETAILSA , fdwdetails : u32 ) -> u32 );
    acmDriverDetailsA(hadid.into_param().abi(), padd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsW<P0>(hadid: P0, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVERID>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverDetailsW ( hadid : HACMDRIVERID , padd : *mut ACMDRIVERDETAILSW , fdwdetails : u32 ) -> u32 );
    acmDriverDetailsW(hadid.into_param().abi(), padd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> u32 {
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverEnum ( fncallback : ACMDRIVERENUMCB , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmDriverEnum(fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmDriverID<P0>(hao: P0, phadid: *mut isize, fdwdriverid: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMOBJ>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverID ( hao : HACMOBJ , phadid : *mut isize , fdwdriverid : u32 ) -> u32 );
    acmDriverID(hao.into_param().abi(), phadid, fdwdriverid)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverMessage<P0, P1, P2>(had: P0, umsg: u32, lparam1: P1, lparam2: P2) -> super::super::Foundation::LRESULT
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverMessage ( had : HACMDRIVER , umsg : u32 , lparam1 : super::super::Foundation:: LPARAM , lparam2 : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: LRESULT );
    acmDriverMessage(had.into_param().abi(), umsg, lparam1.into_param().abi(), lparam2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmDriverOpen<P0>(phad: *mut isize, hadid: P0, fdwopen: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVERID>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverOpen ( phad : *mut isize , hadid : HACMDRIVERID , fdwopen : u32 ) -> u32 );
    acmDriverOpen(phad, hadid.into_param().abi(), fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmDriverPriority<P0>(hadid: P0, dwpriority: u32, fdwpriority: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVERID>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverPriority ( hadid : HACMDRIVERID , dwpriority : u32 , fdwpriority : u32 ) -> u32 );
    acmDriverPriority(hadid.into_param().abi(), dwpriority, fdwpriority)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmDriverRemove<P0>(hadid: P0, fdwremove: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVERID>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmDriverRemove ( hadid : HACMDRIVERID , fdwremove : u32 ) -> u32 );
    acmDriverRemove(hadid.into_param().abi(), fdwremove)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32 {
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterChooseA ( pafltrc : *mut ACMFILTERCHOOSEA ) -> u32 );
    acmFilterChooseA(pafltrc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32 {
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterChooseW ( pafltrc : *mut ACMFILTERCHOOSEW ) -> u32 );
    acmFilterChooseW(pafltrc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFilterDetailsA<P0>(had: P0, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterDetailsA ( had : HACMDRIVER , pafd : *mut ACMFILTERDETAILSA , fdwdetails : u32 ) -> u32 );
    acmFilterDetailsA(had.into_param().abi(), pafd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFilterDetailsW<P0>(had: P0, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterDetailsW ( had : HACMDRIVER , pafd : *mut ACMFILTERDETAILSW , fdwdetails : u32 ) -> u32 );
    acmFilterDetailsW(had.into_param().abi(), pafd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumA<P0>(had: P0, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterEnumA ( had : HACMDRIVER , pafd : *mut ACMFILTERDETAILSA , fncallback : ACMFILTERENUMCBA , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFilterEnumA(had.into_param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumW<P0>(had: P0, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterEnumW ( had : HACMDRIVER , pafd : *mut ACMFILTERDETAILSW , fncallback : ACMFILTERENUMCBW , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFilterEnumW(had.into_param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFilterTagDetailsA<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterTagDetailsA ( had : HACMDRIVER , paftd : *mut ACMFILTERTAGDETAILSA , fdwdetails : u32 ) -> u32 );
    acmFilterTagDetailsA(had.into_param().abi(), paftd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFilterTagDetailsW<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterTagDetailsW ( had : HACMDRIVER , paftd : *mut ACMFILTERTAGDETAILSW , fdwdetails : u32 ) -> u32 );
    acmFilterTagDetailsW(had.into_param().abi(), paftd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumA<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterTagEnumA ( had : HACMDRIVER , paftd : *mut ACMFILTERTAGDETAILSA , fncallback : ACMFILTERTAGENUMCBA , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFilterTagEnumA(had.into_param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumW<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFilterTagEnumW ( had : HACMDRIVER , paftd : *mut ACMFILTERTAGDETAILSW , fncallback : ACMFILTERTAGENUMCBW , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFilterTagEnumW(had.into_param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32 {
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatChooseA ( pafmtc : *mut ACMFORMATCHOOSEA ) -> u32 );
    acmFormatChooseA(pafmtc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32 {
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatChooseW ( pafmtc : *mut ACMFORMATCHOOSEW ) -> u32 );
    acmFormatChooseW(pafmtc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFormatDetailsA<P0>(had: P0, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatDetailsA ( had : HACMDRIVER , pafd : *mut ACMFORMATDETAILSA , fdwdetails : u32 ) -> u32 );
    acmFormatDetailsA(had.into_param().abi(), pafd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFormatDetailsW<P0>(had: P0, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatDetailsW ( had : HACMDRIVER , pafd : *mut tACMFORMATDETAILSW , fdwdetails : u32 ) -> u32 );
    acmFormatDetailsW(had.into_param().abi(), pafd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumA<P0>(had: P0, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatEnumA ( had : HACMDRIVER , pafd : *mut ACMFORMATDETAILSA , fncallback : ACMFORMATENUMCBA , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFormatEnumA(had.into_param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumW<P0>(had: P0, pafd: *mut tACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatEnumW ( had : HACMDRIVER , pafd : *mut tACMFORMATDETAILSW , fncallback : ACMFORMATENUMCBW , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFormatEnumW(had.into_param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFormatSuggest<P0>(had: P0, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatSuggest ( had : HACMDRIVER , pwfxsrc : *mut WAVEFORMATEX , pwfxdst : *mut WAVEFORMATEX , cbwfxdst : u32 , fdwsuggest : u32 ) -> u32 );
    acmFormatSuggest(had.into_param().abi(), pwfxsrc, pwfxdst, cbwfxdst, fdwsuggest)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFormatTagDetailsA<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatTagDetailsA ( had : HACMDRIVER , paftd : *mut ACMFORMATTAGDETAILSA , fdwdetails : u32 ) -> u32 );
    acmFormatTagDetailsA(had.into_param().abi(), paftd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmFormatTagDetailsW<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatTagDetailsW ( had : HACMDRIVER , paftd : *mut ACMFORMATTAGDETAILSW , fdwdetails : u32 ) -> u32 );
    acmFormatTagDetailsW(had.into_param().abi(), paftd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumA<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatTagEnumA ( had : HACMDRIVER , paftd : *mut ACMFORMATTAGDETAILSA , fncallback : ACMFORMATTAGENUMCBA , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFormatTagEnumA(had.into_param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumW<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmFormatTagEnumW ( had : HACMDRIVER , paftd : *mut ACMFORMATTAGDETAILSW , fncallback : ACMFORMATTAGENUMCBW , dwinstance : usize , fdwenum : u32 ) -> u32 );
    acmFormatTagEnumW(had.into_param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmGetVersion() -> u32 {
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmGetVersion ( ) -> u32 );
    acmGetVersion()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmMetrics<P0>(hao: P0, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<HACMOBJ>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmMetrics ( hao : HACMOBJ , umetric : u32 , pmetric : *mut ::core::ffi::c_void ) -> u32 );
    acmMetrics(hao.into_param().abi(), umetric, pmetric)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmStreamClose<P0>(has: P0, fdwclose: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMSTREAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamClose ( has : HACMSTREAM , fdwclose : u32 ) -> u32 );
    acmStreamClose(has.into_param().abi(), fdwclose)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmStreamConvert<P0>(has: P0, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMSTREAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamConvert ( has : HACMSTREAM , pash : *mut ACMSTREAMHEADER , fdwconvert : u32 ) -> u32 );
    acmStreamConvert(has.into_param().abi(), pash, fdwconvert)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmStreamMessage<P0, P1, P2>(has: P0, umsg: u32, lparam1: P1, lparam2: P2) -> u32
where
    P0: ::windows::core::IntoParam<HACMSTREAM>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamMessage ( has : HACMSTREAM , umsg : u32 , lparam1 : super::super::Foundation:: LPARAM , lparam2 : super::super::Foundation:: LPARAM ) -> u32 );
    acmStreamMessage(has.into_param().abi(), umsg, lparam1.into_param().abi(), lparam2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmStreamOpen<P0>(phas: *mut isize, had: P0, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMDRIVER>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamOpen ( phas : *mut isize , had : HACMDRIVER , pwfxsrc : *mut WAVEFORMATEX , pwfxdst : *mut WAVEFORMATEX , pwfltr : *mut WAVEFILTER , dwcallback : usize , dwinstance : usize , fdwopen : u32 ) -> u32 );
    acmStreamOpen(phas, had.into_param().abi(), pwfxsrc, pwfxdst, pwfltr, dwcallback, dwinstance, fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmStreamPrepareHeader<P0>(has: P0, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMSTREAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamPrepareHeader ( has : HACMSTREAM , pash : *mut ACMSTREAMHEADER , fdwprepare : u32 ) -> u32 );
    acmStreamPrepareHeader(has.into_param().abi(), pash, fdwprepare)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmStreamReset<P0>(has: P0, fdwreset: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMSTREAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamReset ( has : HACMSTREAM , fdwreset : u32 ) -> u32 );
    acmStreamReset(has.into_param().abi(), fdwreset)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmStreamSize<P0>(has: P0, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMSTREAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamSize ( has : HACMSTREAM , cbinput : u32 , pdwoutputbytes : *mut u32 , fdwsize : u32 ) -> u32 );
    acmStreamSize(has.into_param().abi(), cbinput, pdwoutputbytes, fdwsize)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn acmStreamUnprepareHeader<P0>(has: P0, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32
where
    P0: ::windows::core::IntoParam<HACMSTREAM>,
{
    ::windows_targets::link ! ( "msacm32.dll""system" fn acmStreamUnprepareHeader ( has : HACMSTREAM , pash : *mut ACMSTREAMHEADER , fdwunprepare : u32 ) -> u32 );
    acmStreamUnprepareHeader(has.into_param().abi(), pash, fdwunprepare)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn auxGetDevCapsA ( udeviceid : usize , pac : *mut AUXCAPSA , cbac : u32 ) -> u32 );
    auxGetDevCapsA(udeviceid, pac, cbac)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn auxGetDevCapsW ( udeviceid : usize , pac : *mut AUXCAPSW , cbac : u32 ) -> u32 );
    auxGetDevCapsW(udeviceid, pac, cbac)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn auxGetNumDevs() -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn auxGetNumDevs ( ) -> u32 );
    auxGetNumDevs()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn auxGetVolume ( udeviceid : u32 , pdwvolume : *mut u32 ) -> u32 );
    auxGetVolume(udeviceid, pdwvolume)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn auxOutMessage ( udeviceid : u32 , umsg : u32 , dw1 : usize , dw2 : usize ) -> u32 );
    auxOutMessage(udeviceid, umsg, dw1, dw2)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn auxSetVolume ( udeviceid : u32 , dwvolume : u32 ) -> u32 );
    auxSetVolume(udeviceid, dwvolume)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiConnect<P0, P1>(hmi: P0, hmo: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<HMIDI>,
    P1: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiConnect ( hmi : HMIDI , hmo : HMIDIOUT , preserved : *const ::core::ffi::c_void ) -> u32 );
    midiConnect(hmi.into_param().abi(), hmo.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiDisconnect<P0, P1>(hmi: P0, hmo: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<HMIDI>,
    P1: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiDisconnect ( hmi : HMIDI , hmo : HMIDIOUT , preserved : *const ::core::ffi::c_void ) -> u32 );
    midiDisconnect(hmi.into_param().abi(), hmo.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInAddBuffer<P0>(hmi: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInAddBuffer ( hmi : HMIDIIN , pmh : *mut MIDIHDR , cbmh : u32 ) -> u32 );
    midiInAddBuffer(hmi.into_param().abi(), pmh, cbmh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInClose<P0>(hmi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInClose ( hmi : HMIDIIN ) -> u32 );
    midiInClose(hmi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInGetDevCapsA ( udeviceid : usize , pmic : *mut MIDIINCAPSA , cbmic : u32 ) -> u32 );
    midiInGetDevCapsA(udeviceid, pmic, cbmic)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInGetDevCapsW ( udeviceid : usize , pmic : *mut MIDIINCAPSW , cbmic : u32 ) -> u32 );
    midiInGetDevCapsW(udeviceid, pmic, cbmic)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInGetErrorTextA ( mmrerror : u32 , psztext : ::windows::core::PSTR , cchtext : u32 ) -> u32 );
    midiInGetErrorTextA(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInGetErrorTextW ( mmrerror : u32 , psztext : ::windows::core::PWSTR , cchtext : u32 ) -> u32 );
    midiInGetErrorTextW(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetID<P0>(hmi: P0, pudeviceid: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInGetID ( hmi : HMIDIIN , pudeviceid : *mut u32 ) -> u32 );
    midiInGetID(hmi.into_param().abi(), pudeviceid)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetNumDevs() -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInGetNumDevs ( ) -> u32 );
    midiInGetNumDevs()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInMessage<P0>(hmi: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInMessage ( hmi : HMIDIIN , umsg : u32 , dw1 : usize , dw2 : usize ) -> u32 );
    midiInMessage(hmi.into_param().abi(), umsg, dw1, dw2)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInOpen ( phmi : *mut HMIDIIN , udeviceid : u32 , dwcallback : usize , dwinstance : usize , fdwopen : MIDI_WAVE_OPEN_TYPE ) -> u32 );
    midiInOpen(phmi, udeviceid, dwcallback, dwinstance, fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInPrepareHeader<P0>(hmi: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInPrepareHeader ( hmi : HMIDIIN , pmh : *mut MIDIHDR , cbmh : u32 ) -> u32 );
    midiInPrepareHeader(hmi.into_param().abi(), pmh, cbmh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInReset<P0>(hmi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInReset ( hmi : HMIDIIN ) -> u32 );
    midiInReset(hmi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInStart<P0>(hmi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInStart ( hmi : HMIDIIN ) -> u32 );
    midiInStart(hmi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInStop<P0>(hmi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInStop ( hmi : HMIDIIN ) -> u32 );
    midiInStop(hmi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInUnprepareHeader<P0>(hmi: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiInUnprepareHeader ( hmi : HMIDIIN , pmh : *mut MIDIHDR , cbmh : u32 ) -> u32 );
    midiInUnprepareHeader(hmi.into_param().abi(), pmh, cbmh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutCacheDrumPatches<P0>(hmo: P0, upatch: u32, pwkya: &[u16; 128], fucache: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutCacheDrumPatches ( hmo : HMIDIOUT , upatch : u32 , pwkya : *const u16 , fucache : u32 ) -> u32 );
    midiOutCacheDrumPatches(hmo.into_param().abi(), upatch, ::core::mem::transmute(pwkya.as_ptr()), fucache)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutCachePatches<P0>(hmo: P0, ubank: u32, pwpa: &[u16; 128], fucache: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutCachePatches ( hmo : HMIDIOUT , ubank : u32 , pwpa : *const u16 , fucache : u32 ) -> u32 );
    midiOutCachePatches(hmo.into_param().abi(), ubank, ::core::mem::transmute(pwpa.as_ptr()), fucache)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutClose<P0>(hmo: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutClose ( hmo : HMIDIOUT ) -> u32 );
    midiOutClose(hmo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutGetDevCapsA ( udeviceid : usize , pmoc : *mut MIDIOUTCAPSA , cbmoc : u32 ) -> u32 );
    midiOutGetDevCapsA(udeviceid, pmoc, cbmoc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutGetDevCapsW ( udeviceid : usize , pmoc : *mut MIDIOUTCAPSW , cbmoc : u32 ) -> u32 );
    midiOutGetDevCapsW(udeviceid, pmoc, cbmoc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutGetErrorTextA ( mmrerror : u32 , psztext : ::windows::core::PSTR , cchtext : u32 ) -> u32 );
    midiOutGetErrorTextA(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutGetErrorTextW ( mmrerror : u32 , psztext : ::windows::core::PWSTR , cchtext : u32 ) -> u32 );
    midiOutGetErrorTextW(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetID<P0>(hmo: P0, pudeviceid: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutGetID ( hmo : HMIDIOUT , pudeviceid : *mut u32 ) -> u32 );
    midiOutGetID(hmo.into_param().abi(), pudeviceid)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetNumDevs() -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutGetNumDevs ( ) -> u32 );
    midiOutGetNumDevs()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetVolume<P0>(hmo: P0, pdwvolume: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutGetVolume ( hmo : HMIDIOUT , pdwvolume : *mut u32 ) -> u32 );
    midiOutGetVolume(hmo.into_param().abi(), pdwvolume)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutLongMsg<P0>(hmo: P0, pmh: *const MIDIHDR, cbmh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutLongMsg ( hmo : HMIDIOUT , pmh : *const MIDIHDR , cbmh : u32 ) -> u32 );
    midiOutLongMsg(hmo.into_param().abi(), pmh, cbmh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutMessage<P0>(hmo: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutMessage ( hmo : HMIDIOUT , umsg : u32 , dw1 : usize , dw2 : usize ) -> u32 );
    midiOutMessage(hmo.into_param().abi(), umsg, dw1, dw2)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutOpen ( phmo : *mut HMIDIOUT , udeviceid : u32 , dwcallback : usize , dwinstance : usize , fdwopen : MIDI_WAVE_OPEN_TYPE ) -> u32 );
    midiOutOpen(phmo, udeviceid, dwcallback, dwinstance, fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutPrepareHeader<P0>(hmo: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutPrepareHeader ( hmo : HMIDIOUT , pmh : *mut MIDIHDR , cbmh : u32 ) -> u32 );
    midiOutPrepareHeader(hmo.into_param().abi(), pmh, cbmh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutReset<P0>(hmo: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutReset ( hmo : HMIDIOUT ) -> u32 );
    midiOutReset(hmo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutSetVolume<P0>(hmo: P0, dwvolume: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutSetVolume ( hmo : HMIDIOUT , dwvolume : u32 ) -> u32 );
    midiOutSetVolume(hmo.into_param().abi(), dwvolume)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutShortMsg<P0>(hmo: P0, dwmsg: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutShortMsg ( hmo : HMIDIOUT , dwmsg : u32 ) -> u32 );
    midiOutShortMsg(hmo.into_param().abi(), dwmsg)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutUnprepareHeader<P0>(hmo: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDIOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiOutUnprepareHeader ( hmo : HMIDIOUT , pmh : *mut MIDIHDR , cbmh : u32 ) -> u32 );
    midiOutUnprepareHeader(hmo.into_param().abi(), pmh, cbmh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamClose<P0>(hms: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDISTRM>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamClose ( hms : HMIDISTRM ) -> u32 );
    midiStreamClose(hms.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: &mut [u32], dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamOpen ( phms : *mut HMIDISTRM , pudeviceid : *mut u32 , cmidi : u32 , dwcallback : usize , dwinstance : usize , fdwopen : u32 ) -> u32 );
    midiStreamOpen(phms, ::core::mem::transmute(pudeviceid.as_ptr()), pudeviceid.len() as _, dwcallback, dwinstance, fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamOut<P0>(hms: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDISTRM>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamOut ( hms : HMIDISTRM , pmh : *mut MIDIHDR , cbmh : u32 ) -> u32 );
    midiStreamOut(hms.into_param().abi(), pmh, cbmh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamPause<P0>(hms: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDISTRM>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamPause ( hms : HMIDISTRM ) -> u32 );
    midiStreamPause(hms.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamPosition<P0>(hms: P0, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDISTRM>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamPosition ( hms : HMIDISTRM , lpmmt : *mut super:: MMTIME , cbmmt : u32 ) -> u32 );
    midiStreamPosition(hms.into_param().abi(), lpmmt, cbmmt)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamProperty<P0>(hms: P0, lppropdata: *mut u8, dwproperty: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIDISTRM>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamProperty ( hms : HMIDISTRM , lppropdata : *mut u8 , dwproperty : u32 ) -> u32 );
    midiStreamProperty(hms.into_param().abi(), lppropdata, dwproperty)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamRestart<P0>(hms: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDISTRM>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamRestart ( hms : HMIDISTRM ) -> u32 );
    midiStreamRestart(hms.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamStop<P0>(hms: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIDISTRM>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn midiStreamStop ( hms : HMIDISTRM ) -> u32 );
    midiStreamStop(hms.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerClose<P0>(hmx: P0) -> u32
where
    P0: ::windows::core::IntoParam<HMIXER>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerClose ( hmx : HMIXER ) -> u32 );
    mixerClose(hmx.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsA<P0>(hmxobj: P0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetControlDetailsA ( hmxobj : HMIXEROBJ , pmxcd : *mut MIXERCONTROLDETAILS , fdwdetails : u32 ) -> u32 );
    mixerGetControlDetailsA(hmxobj.into_param().abi(), pmxcd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsW<P0>(hmxobj: P0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetControlDetailsW ( hmxobj : HMIXEROBJ , pmxcd : *mut MIXERCONTROLDETAILS , fdwdetails : u32 ) -> u32 );
    mixerGetControlDetailsW(hmxobj.into_param().abi(), pmxcd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetDevCapsA ( umxid : usize , pmxcaps : *mut MIXERCAPSA , cbmxcaps : u32 ) -> u32 );
    mixerGetDevCapsA(umxid, pmxcaps, cbmxcaps)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetDevCapsW ( umxid : usize , pmxcaps : *mut MIXERCAPSW , cbmxcaps : u32 ) -> u32 );
    mixerGetDevCapsW(umxid, pmxcaps, cbmxcaps)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetID<P0>(hmxobj: P0, pumxid: *mut u32, fdwid: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetID ( hmxobj : HMIXEROBJ , pumxid : *mut u32 , fdwid : u32 ) -> u32 );
    mixerGetID(hmxobj.into_param().abi(), pumxid, fdwid)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetLineControlsA<P0>(hmxobj: P0, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetLineControlsA ( hmxobj : HMIXEROBJ , pmxlc : *mut MIXERLINECONTROLSA , fdwcontrols : u32 ) -> u32 );
    mixerGetLineControlsA(hmxobj.into_param().abi(), pmxlc, fdwcontrols)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetLineControlsW<P0>(hmxobj: P0, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetLineControlsW ( hmxobj : HMIXEROBJ , pmxlc : *mut MIXERLINECONTROLSW , fdwcontrols : u32 ) -> u32 );
    mixerGetLineControlsW(hmxobj.into_param().abi(), pmxlc, fdwcontrols)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetLineInfoA<P0>(hmxobj: P0, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetLineInfoA ( hmxobj : HMIXEROBJ , pmxl : *mut MIXERLINEA , fdwinfo : u32 ) -> u32 );
    mixerGetLineInfoA(hmxobj.into_param().abi(), pmxl, fdwinfo)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetLineInfoW<P0>(hmxobj: P0, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetLineInfoW ( hmxobj : HMIXEROBJ , pmxl : *mut MIXERLINEW , fdwinfo : u32 ) -> u32 );
    mixerGetLineInfoW(hmxobj.into_param().abi(), pmxl, fdwinfo)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerGetNumDevs() -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerGetNumDevs ( ) -> u32 );
    mixerGetNumDevs()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerMessage<P0>(hmx: P0, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32
where
    P0: ::windows::core::IntoParam<HMIXER>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerMessage ( hmx : HMIXER , umsg : u32 , dwparam1 : usize , dwparam2 : usize ) -> u32 );
    mixerMessage(hmx.into_param().abi(), umsg, dwparam1, dwparam2)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn mixerOpen(phmx: ::core::option::Option<*mut isize>, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerOpen ( phmx : *mut isize , umxid : u32 , dwcallback : usize , dwinstance : usize , fdwopen : u32 ) -> u32 );
    mixerOpen(::core::mem::transmute(phmx.unwrap_or(::std::ptr::null_mut())), umxid, dwcallback, dwinstance, fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerSetControlDetails<P0>(hmxobj: P0, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32
where
    P0: ::windows::core::IntoParam<HMIXEROBJ>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn mixerSetControlDetails ( hmxobj : HMIXEROBJ , pmxcd : *const MIXERCONTROLDETAILS , fdwdetails : u32 ) -> u32 );
    mixerSetControlDetails(hmxobj.into_param().abi(), pmxcd, fdwdetails)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundA<P0>(pszsound: P0, fusound: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn sndPlaySoundA ( pszsound : ::windows::core::PCSTR , fusound : u32 ) -> super::super::Foundation:: BOOL );
    sndPlaySoundA(pszsound.into_param().abi(), fusound)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundW<P0>(pszsound: P0, fusound: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn sndPlaySoundW ( pszsound : ::windows::core::PCWSTR , fusound : u32 ) -> super::super::Foundation:: BOOL );
    sndPlaySoundW(pszsound.into_param().abi(), fusound)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInAddBuffer<P0>(hwi: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInAddBuffer ( hwi : HWAVEIN , pwh : *mut WAVEHDR , cbwh : u32 ) -> u32 );
    waveInAddBuffer(hwi.into_param().abi(), pwh, cbwh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInClose<P0>(hwi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInClose ( hwi : HWAVEIN ) -> u32 );
    waveInClose(hwi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInGetDevCapsA ( udeviceid : usize , pwic : *mut WAVEINCAPSA , cbwic : u32 ) -> u32 );
    waveInGetDevCapsA(udeviceid, pwic, cbwic)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInGetDevCapsW ( udeviceid : usize , pwic : *mut WAVEINCAPSW , cbwic : u32 ) -> u32 );
    waveInGetDevCapsW(udeviceid, pwic, cbwic)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInGetErrorTextA ( mmrerror : u32 , psztext : ::windows::core::PSTR , cchtext : u32 ) -> u32 );
    waveInGetErrorTextA(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInGetErrorTextW ( mmrerror : u32 , psztext : ::windows::core::PWSTR , cchtext : u32 ) -> u32 );
    waveInGetErrorTextW(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetID<P0>(hwi: P0, pudeviceid: *const u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInGetID ( hwi : HWAVEIN , pudeviceid : *const u32 ) -> u32 );
    waveInGetID(hwi.into_param().abi(), pudeviceid)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetNumDevs() -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInGetNumDevs ( ) -> u32 );
    waveInGetNumDevs()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetPosition<P0>(hwi: P0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInGetPosition ( hwi : HWAVEIN , pmmt : *mut super:: MMTIME , cbmmt : u32 ) -> u32 );
    waveInGetPosition(hwi.into_param().abi(), pmmt, cbmmt)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInMessage<P0>(hwi: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInMessage ( hwi : HWAVEIN , umsg : u32 , dw1 : usize , dw2 : usize ) -> u32 );
    waveInMessage(hwi.into_param().abi(), umsg, dw1, dw2)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInOpen(phwi: ::core::option::Option<*mut HWAVEIN>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInOpen ( phwi : *mut HWAVEIN , udeviceid : u32 , pwfx : *const WAVEFORMATEX , dwcallback : usize , dwinstance : usize , fdwopen : MIDI_WAVE_OPEN_TYPE ) -> u32 );
    waveInOpen(::core::mem::transmute(phwi.unwrap_or(::std::ptr::null_mut())), udeviceid, pwfx, dwcallback, dwinstance, fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInPrepareHeader<P0>(hwi: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInPrepareHeader ( hwi : HWAVEIN , pwh : *mut WAVEHDR , cbwh : u32 ) -> u32 );
    waveInPrepareHeader(hwi.into_param().abi(), pwh, cbwh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInReset<P0>(hwi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInReset ( hwi : HWAVEIN ) -> u32 );
    waveInReset(hwi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInStart<P0>(hwi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInStart ( hwi : HWAVEIN ) -> u32 );
    waveInStart(hwi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInStop<P0>(hwi: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInStop ( hwi : HWAVEIN ) -> u32 );
    waveInStop(hwi.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInUnprepareHeader<P0>(hwi: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEIN>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveInUnprepareHeader ( hwi : HWAVEIN , pwh : *mut WAVEHDR , cbwh : u32 ) -> u32 );
    waveInUnprepareHeader(hwi.into_param().abi(), pwh, cbwh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutBreakLoop<P0>(hwo: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutBreakLoop ( hwo : HWAVEOUT ) -> u32 );
    waveOutBreakLoop(hwo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutClose<P0>(hwo: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutClose ( hwo : HWAVEOUT ) -> u32 );
    waveOutClose(hwo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetDevCapsA ( udeviceid : usize , pwoc : *mut WAVEOUTCAPSA , cbwoc : u32 ) -> u32 );
    waveOutGetDevCapsA(udeviceid, pwoc, cbwoc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetDevCapsW ( udeviceid : usize , pwoc : *mut WAVEOUTCAPSW , cbwoc : u32 ) -> u32 );
    waveOutGetDevCapsW(udeviceid, pwoc, cbwoc)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetErrorTextA ( mmrerror : u32 , psztext : ::windows::core::PSTR , cchtext : u32 ) -> u32 );
    waveOutGetErrorTextA(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetErrorTextW ( mmrerror : u32 , psztext : ::windows::core::PWSTR , cchtext : u32 ) -> u32 );
    waveOutGetErrorTextW(mmrerror, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetID<P0>(hwo: P0, pudeviceid: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetID ( hwo : HWAVEOUT , pudeviceid : *mut u32 ) -> u32 );
    waveOutGetID(hwo.into_param().abi(), pudeviceid)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetNumDevs() -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetNumDevs ( ) -> u32 );
    waveOutGetNumDevs()
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetPitch<P0>(hwo: P0, pdwpitch: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetPitch ( hwo : HWAVEOUT , pdwpitch : *mut u32 ) -> u32 );
    waveOutGetPitch(hwo.into_param().abi(), pdwpitch)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetPlaybackRate<P0>(hwo: P0, pdwrate: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetPlaybackRate ( hwo : HWAVEOUT , pdwrate : *mut u32 ) -> u32 );
    waveOutGetPlaybackRate(hwo.into_param().abi(), pdwrate)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetPosition<P0>(hwo: P0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetPosition ( hwo : HWAVEOUT , pmmt : *mut super:: MMTIME , cbmmt : u32 ) -> u32 );
    waveOutGetPosition(hwo.into_param().abi(), pmmt, cbmmt)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetVolume<P0>(hwo: P0, pdwvolume: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutGetVolume ( hwo : HWAVEOUT , pdwvolume : *mut u32 ) -> u32 );
    waveOutGetVolume(hwo.into_param().abi(), pdwvolume)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutMessage<P0>(hwo: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutMessage ( hwo : HWAVEOUT , umsg : u32 , dw1 : usize , dw2 : usize ) -> u32 );
    waveOutMessage(hwo.into_param().abi(), umsg, dw1, dw2)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutOpen(phwo: ::core::option::Option<*mut HWAVEOUT>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutOpen ( phwo : *mut HWAVEOUT , udeviceid : u32 , pwfx : *const WAVEFORMATEX , dwcallback : usize , dwinstance : usize , fdwopen : MIDI_WAVE_OPEN_TYPE ) -> u32 );
    waveOutOpen(::core::mem::transmute(phwo.unwrap_or(::std::ptr::null_mut())), udeviceid, pwfx, dwcallback, dwinstance, fdwopen)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutPause<P0>(hwo: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutPause ( hwo : HWAVEOUT ) -> u32 );
    waveOutPause(hwo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutPrepareHeader<P0>(hwo: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutPrepareHeader ( hwo : HWAVEOUT , pwh : *mut WAVEHDR , cbwh : u32 ) -> u32 );
    waveOutPrepareHeader(hwo.into_param().abi(), pwh, cbwh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutReset<P0>(hwo: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutReset ( hwo : HWAVEOUT ) -> u32 );
    waveOutReset(hwo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutRestart<P0>(hwo: P0) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutRestart ( hwo : HWAVEOUT ) -> u32 );
    waveOutRestart(hwo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutSetPitch<P0>(hwo: P0, dwpitch: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutSetPitch ( hwo : HWAVEOUT , dwpitch : u32 ) -> u32 );
    waveOutSetPitch(hwo.into_param().abi(), dwpitch)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutSetPlaybackRate<P0>(hwo: P0, dwrate: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutSetPlaybackRate ( hwo : HWAVEOUT , dwrate : u32 ) -> u32 );
    waveOutSetPlaybackRate(hwo.into_param().abi(), dwrate)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutSetVolume<P0>(hwo: P0, dwvolume: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutSetVolume ( hwo : HWAVEOUT , dwvolume : u32 ) -> u32 );
    waveOutSetVolume(hwo.into_param().abi(), dwvolume)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutUnprepareHeader<P0>(hwo: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutUnprepareHeader ( hwo : HWAVEOUT , pwh : *mut WAVEHDR , cbwh : u32 ) -> u32 );
    waveOutUnprepareHeader(hwo.into_param().abi(), pwh, cbwh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutWrite<P0>(hwo: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: ::windows::core::IntoParam<HWAVEOUT>,
{
    ::windows_targets::link ! ( "winmm.dll""system" fn waveOutWrite ( hwo : HWAVEOUT , pwh : *mut WAVEHDR , cbwh : u32 ) -> u32 );
    waveOutWrite(hwo.into_param().abi(), pwh, cbwh)
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAcousticEchoCancellationControl(::windows::core::IUnknown);
impl IAcousticEchoCancellationControl {
    pub unsafe fn SetEchoCancellationRenderEndpoint<P0>(&self, endpointid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetEchoCancellationRenderEndpoint)(::windows::core::Interface::as_raw(self), endpointid.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAcousticEchoCancellationControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAcousticEchoCancellationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAcousticEchoCancellationControl {}
impl ::core::fmt::Debug for IAcousticEchoCancellationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAcousticEchoCancellationControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAcousticEchoCancellationControl {
    type Vtable = IAcousticEchoCancellationControl_Vtbl;
}
impl ::core::clone::Clone for IAcousticEchoCancellationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAcousticEchoCancellationControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4ae25b5_aaa3_437d_b6b3_dbbe2d0e9549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcousticEchoCancellationControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetEchoCancellationRenderEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IActivateAudioInterfaceAsyncOperation(::windows::core::IUnknown);
impl IActivateAudioInterfaceAsyncOperation {
    pub unsafe fn GetActivateResult(&self, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetActivateResult)(::windows::core::Interface::as_raw(self), activateresult, ::core::mem::transmute(activatedinterface)).ok()
    }
}
::windows::imp::interface_hierarchy!(IActivateAudioInterfaceAsyncOperation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActivateAudioInterfaceAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceAsyncOperation {}
impl ::core::fmt::Debug for IActivateAudioInterfaceAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceAsyncOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActivateAudioInterfaceAsyncOperation {
    type Vtable = IActivateAudioInterfaceAsyncOperation_Vtbl;
}
impl ::core::clone::Clone for IActivateAudioInterfaceAsyncOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivateAudioInterfaceAsyncOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a22d78_cde4_431d_b8cc_843a71199b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetActivateResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IActivateAudioInterfaceCompletionHandler(::windows::core::IUnknown);
impl IActivateAudioInterfaceCompletionHandler {
    pub unsafe fn ActivateCompleted<P0>(&self, activateoperation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActivateAudioInterfaceAsyncOperation>,
    {
        (::windows::core::Interface::vtable(self).ActivateCompleted)(::windows::core::Interface::as_raw(self), activateoperation.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IActivateAudioInterfaceCompletionHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActivateAudioInterfaceCompletionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceCompletionHandler {}
impl ::core::fmt::Debug for IActivateAudioInterfaceCompletionHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceCompletionHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActivateAudioInterfaceCompletionHandler {
    type Vtable = IActivateAudioInterfaceCompletionHandler_Vtbl;
}
impl ::core::clone::Clone for IActivateAudioInterfaceCompletionHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivateAudioInterfaceCompletionHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d949ab_9862_444a_80f6_c261334da5eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ActivateCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activateoperation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioAmbisonicsControl(::windows::core::IUnknown);
impl IAudioAmbisonicsControl {
    pub unsafe fn SetData(&self, pambisonicsparams: &[AMBISONICS_PARAMS]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pambisonicsparams.as_ptr()), pambisonicsparams.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHeadTracking<P0>(&self, benableheadtracking: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetHeadTracking)(::windows::core::Interface::as_raw(self), benableheadtracking.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHeadTracking(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetHeadTracking)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRotation)(::windows::core::Interface::as_raw(self), x, y, z, w).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioAmbisonicsControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioAmbisonicsControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAmbisonicsControl {}
impl ::core::fmt::Debug for IAudioAmbisonicsControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAmbisonicsControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioAmbisonicsControl {
    type Vtable = IAudioAmbisonicsControl_Vtbl;
}
impl ::core::clone::Clone for IAudioAmbisonicsControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioAmbisonicsControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28724c91_df35_4856_9f76_d6a26413f3df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAmbisonicsControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHeadTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHeadTracking: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHeadTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHeadTracking: usize,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioAutoGainControl(::windows::core::IUnknown);
impl IAudioAutoGainControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, benable: P0, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioAutoGainControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioAutoGainControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAutoGainControl {}
impl ::core::fmt::Debug for IAudioAutoGainControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAutoGainControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioAutoGainControl {
    type Vtable = IAudioAutoGainControl_Vtbl;
}
impl ::core::clone::Clone for IAudioAutoGainControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioAutoGainControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAutoGainControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioBass(::windows::core::IUnknown);
impl IAudioBass {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLevelRange)(::windows::core::Interface::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).base__.GetLevel)(::windows::core::Interface::as_raw(self), nchannel, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevel)(::windows::core::Interface::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelUniform)(::windows::core::Interface::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelAllChannels)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioBass, ::windows::core::IUnknown, IPerChannelDbLevel);
impl ::core::cmp::PartialEq for IAudioBass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioBass {}
impl ::core::fmt::Debug for IAudioBass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioBass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioBass {
    type Vtable = IAudioBass_Vtbl;
}
impl ::core::clone::Clone for IAudioBass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioBass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2b1a1d9_4db3_425d_a2b2_bd335cb3e2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBass_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioCaptureClient(::windows::core::IUnknown);
impl IAudioCaptureClient {
    pub unsafe fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: ::core::option::Option<*mut u64>, pu64qpcposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), ppdata, pnumframestoread, pdwflags, ::core::mem::transmute(pu64deviceposition.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pu64qpcposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseBuffer)(::windows::core::Interface::as_raw(self), numframesread).ok()
    }
    pub unsafe fn GetNextPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNextPacketSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioCaptureClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioCaptureClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioCaptureClient {}
impl ::core::fmt::Debug for IAudioCaptureClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioCaptureClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioCaptureClient {
    type Vtable = IAudioCaptureClient_Vtbl;
}
impl ::core::clone::Clone for IAudioCaptureClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioCaptureClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8adbd64_e71e_48a0_a4de_185c395cd317);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows::core::HRESULT,
    pub GetNextPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioChannelConfig(::windows::core::IUnknown);
impl IAudioChannelConfig {
    pub unsafe fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelConfig)(::windows::core::Interface::as_raw(self), dwconfig, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetChannelConfig(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetChannelConfig)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioChannelConfig, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioChannelConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioChannelConfig {}
impl ::core::fmt::Debug for IAudioChannelConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioChannelConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioChannelConfig {
    type Vtable = IAudioChannelConfig_Vtbl;
}
impl ::core::clone::Clone for IAudioChannelConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioChannelConfig {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb11c46f_ec28_493c_b88a_5db88062ce98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioChannelConfig_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetChannelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetChannelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconfig: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClient(::windows::core::IUnknown);
impl IAudioClient {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetBufferSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetStreamLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCurrentPadding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: ::core::option::Option<*mut *mut WAVEFORMATEX>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).IsFormatSupported)(::windows::core::Interface::as_raw(self), sharemode, pformat, ::core::mem::transmute(ppclosestmatch.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).GetMixFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: ::core::option::Option<*mut i64>, phnsminimumdeviceperiod: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDevicePeriod)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phnsdefaultdeviceperiod.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phnsminimumdeviceperiod.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).SetEventHandle)(::windows::core::Interface::as_raw(self), eventhandle.into_param().abi()).ok()
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetService)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient {}
impl ::core::fmt::Debug for IAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient {
    type Vtable = IAudioClient_Vtbl;
}
impl ::core::clone::Clone for IAudioClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT,
    pub GetStreamLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phnslatency: *mut i64) -> ::windows::core::HRESULT,
    pub GetCurrentPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetMixFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetDevicePeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventHandle: usize,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClient2(::windows::core::IUnknown);
impl IAudioClient2 {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetBufferSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).base__.GetStreamLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCurrentPadding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: ::core::option::Option<*mut *mut WAVEFORMATEX>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.IsFormatSupported)(::windows::core::Interface::as_raw(self), sharemode, pformat, ::core::mem::transmute(ppclosestmatch.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).base__.GetMixFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: ::core::option::Option<*mut i64>, phnsminimumdeviceperiod: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDevicePeriod)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phnsdefaultdeviceperiod.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phnsminimumdeviceperiod.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEventHandle)(::windows::core::Interface::as_raw(self), eventhandle.into_param().abi()).ok()
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetService)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsOffloadCapable)(::windows::core::Interface::as_raw(self), category, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientProperties)(::windows::core::Interface::as_raw(self), pproperties).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBufferSizeLimits<P0>(&self, pformat: *const WAVEFORMATEX, beventdriven: P0, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetBufferSizeLimits)(::windows::core::Interface::as_raw(self), pformat, beventdriven.into_param().abi(), phnsminbufferduration, phnsmaxbufferduration).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioClient2, ::windows::core::IUnknown, IAudioClient);
impl ::core::cmp::PartialEq for IAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient2 {}
impl ::core::fmt::Debug for IAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient2 {
    type Vtable = IAudioClient2_Vtbl;
}
impl ::core::clone::Clone for IAudioClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioClient2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x726778cd_f60a_4eda_82de_e47610cd78aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient2_Vtbl {
    pub base__: IAudioClient_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOffloadCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOffloadCapable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: *const AudioClientProperties) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBufferSizeLimits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBufferSizeLimits: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClient3(::windows::core::IUnknown);
impl IAudioClient3 {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Initialize)(::windows::core::Interface::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetBufferSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStreamLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCurrentPadding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: ::core::option::Option<*mut *mut WAVEFORMATEX>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.IsFormatSupported)(::windows::core::Interface::as_raw(self), sharemode, pformat, ::core::mem::transmute(ppclosestmatch.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMixFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: ::core::option::Option<*mut i64>, phnsminimumdeviceperiod: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevicePeriod)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phnsdefaultdeviceperiod.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phnsminimumdeviceperiod.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetEventHandle)(::windows::core::Interface::as_raw(self), eventhandle.into_param().abi()).ok()
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetService)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsOffloadCapable)(::windows::core::Interface::as_raw(self), category, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetClientProperties)(::windows::core::Interface::as_raw(self), pproperties).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBufferSizeLimits<P0>(&self, pformat: *const WAVEFORMATEX, beventdriven: P0, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.GetBufferSizeLimits)(::windows::core::Interface::as_raw(self), pformat, beventdriven.into_param().abi(), phnsminbufferduration, phnsmaxbufferduration).ok()
    }
    pub unsafe fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSharedModeEnginePeriod)(::windows::core::Interface::as_raw(self), pformat, pdefaultperiodinframes, pfundamentalperiodinframes, pminperiodinframes, pmaxperiodinframes).ok()
    }
    pub unsafe fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentSharedModeEnginePeriod)(::windows::core::Interface::as_raw(self), ppformat, pcurrentperiodinframes).ok()
    }
    pub unsafe fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeSharedAudioStream)(::windows::core::Interface::as_raw(self), streamflags, periodinframes, pformat, ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioClient3, ::windows::core::IUnknown, IAudioClient, IAudioClient2);
impl ::core::cmp::PartialEq for IAudioClient3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient3 {}
impl ::core::fmt::Debug for IAudioClient3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient3 {
    type Vtable = IAudioClient3_Vtbl;
}
impl ::core::clone::Clone for IAudioClient3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioClient3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed4ee07_8e67_4cd4_8c1a_2b7a5987ad42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient3_Vtbl {
    pub base__: IAudioClient2_Vtbl,
    pub GetSharedModeEnginePeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::HRESULT,
    pub GetCurrentSharedModeEnginePeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::HRESULT,
    pub InitializeSharedAudioStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClientDuckingControl(::windows::core::IUnknown);
impl IAudioClientDuckingControl {
    pub unsafe fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDuckingOptionsForCurrentStream)(::windows::core::Interface::as_raw(self), options).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioClientDuckingControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioClientDuckingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClientDuckingControl {}
impl ::core::fmt::Debug for IAudioClientDuckingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClientDuckingControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClientDuckingControl {
    type Vtable = IAudioClientDuckingControl_Vtbl;
}
impl ::core::clone::Clone for IAudioClientDuckingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioClientDuckingControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc789d381_a28c_4168_b28f_d3a837924dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClientDuckingControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDuckingOptionsForCurrentStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClock(::windows::core::IUnknown);
impl IAudioClock {
    pub unsafe fn GetFrequency(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetFrequency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPosition)(::windows::core::Interface::as_raw(self), pu64position, ::core::mem::transmute(pu64qpcposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCharacteristics)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioClock, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock {}
impl ::core::fmt::Debug for IAudioClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClock {
    type Vtable = IAudioClock_Vtbl;
}
impl ::core::clone::Clone for IAudioClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioClock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd63314f_3fba_4a1b_812c_ef96358728e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu64frequency: *mut u64) -> ::windows::core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT,
    pub GetCharacteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClock2(::windows::core::IUnknown);
impl IAudioClock2 {
    pub unsafe fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDevicePosition)(::windows::core::Interface::as_raw(self), deviceposition, ::core::mem::transmute(qpcposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioClock2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioClock2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock2 {}
impl ::core::fmt::Debug for IAudioClock2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClock2 {
    type Vtable = IAudioClock2_Vtbl;
}
impl ::core::clone::Clone for IAudioClock2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioClock2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f49ff73_6727_49ac_a008_d98cf5e70048);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDevicePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClockAdjustment(::windows::core::IUnknown);
impl IAudioClockAdjustment {
    pub unsafe fn SetSampleRate(&self, flsamplerate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSampleRate)(::windows::core::Interface::as_raw(self), flsamplerate).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioClockAdjustment, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioClockAdjustment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClockAdjustment {}
impl ::core::fmt::Debug for IAudioClockAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClockAdjustment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClockAdjustment {
    type Vtable = IAudioClockAdjustment_Vtbl;
}
impl ::core::clone::Clone for IAudioClockAdjustment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioClockAdjustment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6e4c0a0_46d9_4fb8_be21_57a3ef2b626c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClockAdjustment_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flsamplerate: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioEffectsChangedNotificationClient(::windows::core::IUnknown);
impl IAudioEffectsChangedNotificationClient {
    pub unsafe fn OnAudioEffectsChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAudioEffectsChanged)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioEffectsChangedNotificationClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioEffectsChangedNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsChangedNotificationClient {}
impl ::core::fmt::Debug for IAudioEffectsChangedNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsChangedNotificationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEffectsChangedNotificationClient {
    type Vtable = IAudioEffectsChangedNotificationClient_Vtbl;
}
impl ::core::clone::Clone for IAudioEffectsChangedNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioEffectsChangedNotificationClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ded44f_3c5d_4b2b_bd1e_5dc1ee20bbf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsChangedNotificationClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnAudioEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioEffectsManager(::windows::core::IUnknown);
impl IAudioEffectsManager {
    pub unsafe fn RegisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioEffectsChangedNotificationClient>,
    {
        (::windows::core::Interface::vtable(self).RegisterAudioEffectsChangedNotificationCallback)(::windows::core::Interface::as_raw(self), client.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioEffectsChangedNotificationClient>,
    {
        (::windows::core::Interface::vtable(self).UnregisterAudioEffectsChangedNotificationCallback)(::windows::core::Interface::as_raw(self), client.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAudioEffects)(::windows::core::Interface::as_raw(self), effects, numeffects).ok()
    }
    pub unsafe fn SetAudioEffectState(&self, effectid: ::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAudioEffectState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(effectid), state).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioEffectsManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsManager {}
impl ::core::fmt::Debug for IAudioEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEffectsManager {
    type Vtable = IAudioEffectsManager_Vtbl;
}
impl ::core::clone::Clone for IAudioEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioEffectsManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4460b3ae_4b44_4527_8676_7548a8acd260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, client: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, client: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAudioEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAudioEffects: usize,
    pub SetAudioEffectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioFormatEnumerator(::windows::core::IUnknown);
impl IAudioFormatEnumerator {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFormat(&self, index: u32) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioFormatEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioFormatEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFormatEnumerator {}
impl ::core::fmt::Debug for IAudioFormatEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFormatEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioFormatEnumerator {
    type Vtable = IAudioFormatEnumerator_Vtbl;
}
impl ::core::clone::Clone for IAudioFormatEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFormatEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdaa858_895a_4a22_a5eb_67bda506096d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFormatEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioInputSelector(::windows::core::IUnknown);
impl IAudioInputSelector {
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::windows::core::Interface::as_raw(self), nidselect, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioInputSelector, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioInputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputSelector {}
impl ::core::fmt::Debug for IAudioInputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioInputSelector {
    type Vtable = IAudioInputSelector_Vtbl;
}
impl ::core::clone::Clone for IAudioInputSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioInputSelector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f03dc02_5e6e_4653_8f72_a030c123d598);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputSelector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioLoudness(::windows::core::IUnknown);
impl IAudioLoudness {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetEnabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, benable: P0, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioLoudness, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioLoudness {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioLoudness {}
impl ::core::fmt::Debug for IAudioLoudness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioLoudness").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioLoudness {
    type Vtable = IAudioLoudness_Vtbl;
}
impl ::core::clone::Clone for IAudioLoudness {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioLoudness {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLoudness_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioMidrange(::windows::core::IUnknown);
impl IAudioMidrange {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLevelRange)(::windows::core::Interface::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).base__.GetLevel)(::windows::core::Interface::as_raw(self), nchannel, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevel)(::windows::core::Interface::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelUniform)(::windows::core::Interface::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelAllChannels)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioMidrange, ::windows::core::IUnknown, IPerChannelDbLevel);
impl ::core::cmp::PartialEq for IAudioMidrange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMidrange {}
impl ::core::fmt::Debug for IAudioMidrange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMidrange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioMidrange {
    type Vtable = IAudioMidrange_Vtbl;
}
impl ::core::clone::Clone for IAudioMidrange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioMidrange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e54b6d7_b44b_40d9_9a9e_e691d9ce6edf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMidrange_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioMute(::windows::core::IUnknown);
impl IAudioMute {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<P0>(&self, bmuted: P0, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMute)(::windows::core::Interface::as_raw(self), bmuted.into_param().abi(), ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetMute)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioMute, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioMute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMute {}
impl ::core::fmt::Debug for IAudioMute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioMute {
    type Vtable = IAudioMute_Vtbl;
}
impl ::core::clone::Clone for IAudioMute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioMute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMute_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMute: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioOutputSelector(::windows::core::IUnknown);
impl IAudioOutputSelector {
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::windows::core::Interface::as_raw(self), nidselect, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioOutputSelector, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioOutputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioOutputSelector {}
impl ::core::fmt::Debug for IAudioOutputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioOutputSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioOutputSelector {
    type Vtable = IAudioOutputSelector_Vtbl;
}
impl ::core::clone::Clone for IAudioOutputSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioOutputSelector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputSelector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioPeakMeter(::windows::core::IUnknown);
impl IAudioPeakMeter {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetLevel)(::windows::core::Interface::as_raw(self), nchannel, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioPeakMeter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioPeakMeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioPeakMeter {}
impl ::core::fmt::Debug for IAudioPeakMeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioPeakMeter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioPeakMeter {
    type Vtable = IAudioPeakMeter_Vtbl;
}
impl ::core::clone::Clone for IAudioPeakMeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioPeakMeter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPeakMeter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioRenderClient(::windows::core::IUnknown);
impl IAudioRenderClient {
    pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::windows::core::zeroed::<*mut u8>();
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), numframesrequested, &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseBuffer)(::windows::core::Interface::as_raw(self), numframeswritten, dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioRenderClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioRenderClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioRenderClient {}
impl ::core::fmt::Debug for IAudioRenderClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioRenderClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioRenderClient {
    type Vtable = IAudioRenderClient_Vtbl;
}
impl ::core::clone::Clone for IAudioRenderClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioRenderClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf294acfc_3146_4483_a7bf_addca7c260e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionControl(::windows::core::IUnknown);
impl IAudioSessionControl {
    pub unsafe fn GetState(&self) -> ::windows::core::Result<AudioSessionState> {
        let mut result__ = ::windows::core::zeroed::<AudioSessionState>();
        (::windows::core::Interface::vtable(self).GetState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, value: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::windows::core::Interface::as_raw(self), value.into_param().abi(), eventcontext).ok()
    }
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetIconPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIconPath<P0>(&self, value: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetIconPath)(::windows::core::Interface::as_raw(self), value.into_param().abi(), eventcontext).ok()
    }
    pub unsafe fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetGroupingParam)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGroupingParam)(::windows::core::Interface::as_raw(self), r#override, eventcontext).ok()
    }
    pub unsafe fn RegisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSessionEvents>,
    {
        (::windows::core::Interface::vtable(self).RegisterAudioSessionNotification)(::windows::core::Interface::as_raw(self), newnotifications.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSessionEvents>,
    {
        (::windows::core::Interface::vtable(self).UnregisterAudioSessionNotification)(::windows::core::Interface::as_raw(self), newnotifications.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSessionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl {}
impl ::core::fmt::Debug for IAudioSessionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionControl {
    type Vtable = IAudioSessionControl_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4b1a599_7266_4319_a8ca_e70acb11e8cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut AudioSessionState) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetIconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetGroupingParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetGroupingParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterAudioSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newnotifications: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterAudioSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newnotifications: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionControl2(::windows::core::IUnknown);
impl IAudioSessionControl2 {
    pub unsafe fn GetState(&self) -> ::windows::core::Result<AudioSessionState> {
        let mut result__ = ::windows::core::zeroed::<AudioSessionState>();
        (::windows::core::Interface::vtable(self).base__.GetState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, value: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDisplayName)(::windows::core::Interface::as_raw(self), value.into_param().abi(), eventcontext).ok()
    }
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetIconPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIconPath<P0>(&self, value: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetIconPath)(::windows::core::Interface::as_raw(self), value.into_param().abi(), eventcontext).ok()
    }
    pub unsafe fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetGroupingParam)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetGroupingParam)(::windows::core::Interface::as_raw(self), r#override, eventcontext).ok()
    }
    pub unsafe fn RegisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSessionEvents>,
    {
        (::windows::core::Interface::vtable(self).base__.RegisterAudioSessionNotification)(::windows::core::Interface::as_raw(self), newnotifications.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSessionEvents>,
    {
        (::windows::core::Interface::vtable(self).base__.UnregisterAudioSessionNotification)(::windows::core::Interface::as_raw(self), newnotifications.into_param().abi()).ok()
    }
    pub unsafe fn GetSessionIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSessionIdentifier)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSessionInstanceIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSessionInstanceIdentifier)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetProcessId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsSystemSoundsSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsSystemSoundsSession)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDuckingPreference<P0>(&self, optout: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetDuckingPreference)(::windows::core::Interface::as_raw(self), optout.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionControl2, ::windows::core::IUnknown, IAudioSessionControl);
impl ::core::cmp::PartialEq for IAudioSessionControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl2 {}
impl ::core::fmt::Debug for IAudioSessionControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionControl2 {
    type Vtable = IAudioSessionControl2_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfb7ff88_7239_4fc9_8fa2_07c950be9c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl2_Vtbl {
    pub base__: IAudioSessionControl_Vtbl,
    pub GetSessionIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSessionInstanceIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT,
    pub IsSystemSoundsSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDuckingPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optout: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDuckingPreference: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionEnumerator(::windows::core::IUnknown);
impl IAudioSessionEnumerator {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSession(&self, sessioncount: i32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__ = ::windows::core::zeroed::<IAudioSessionControl>();
        (::windows::core::Interface::vtable(self).GetSession)(::windows::core::Interface::as_raw(self), sessioncount, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSessionEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEnumerator {}
impl ::core::fmt::Debug for IAudioSessionEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionEnumerator {
    type Vtable = IAudioSessionEnumerator_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2f5bb11_0570_40ca_acdd_3aa01277dee8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessioncount: *mut i32) -> ::windows::core::HRESULT,
    pub GetSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessioncount: i32, session: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionEvents(::windows::core::IUnknown);
impl IAudioSessionEvents {
    pub unsafe fn OnDisplayNameChanged<P0>(&self, newdisplayname: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnDisplayNameChanged)(::windows::core::Interface::as_raw(self), newdisplayname.into_param().abi(), eventcontext).ok()
    }
    pub unsafe fn OnIconPathChanged<P0>(&self, newiconpath: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnIconPathChanged)(::windows::core::Interface::as_raw(self), newiconpath.into_param().abi(), eventcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSimpleVolumeChanged<P0>(&self, newvolume: f32, newmute: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnSimpleVolumeChanged)(::windows::core::Interface::as_raw(self), newvolume, newmute.into_param().abi(), eventcontext).ok()
    }
    pub unsafe fn OnChannelVolumeChanged(&self, newchannelvolumearray: &[f32], changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChannelVolumeChanged)(::windows::core::Interface::as_raw(self), newchannelvolumearray.len() as _, ::core::mem::transmute(newchannelvolumearray.as_ptr()), changedchannel, eventcontext).ok()
    }
    pub unsafe fn OnGroupingParamChanged(&self, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnGroupingParamChanged)(::windows::core::Interface::as_raw(self), newgroupingparam, eventcontext).ok()
    }
    pub unsafe fn OnStateChanged(&self, newstate: AudioSessionState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStateChanged)(::windows::core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSessionDisconnected)(::windows::core::Interface::as_raw(self), disconnectreason).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSessionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEvents {}
impl ::core::fmt::Debug for IAudioSessionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionEvents {
    type Vtable = IAudioSessionEvents_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24918acc_64b3_37c1_8ca9_74a66e9957a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnDisplayNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newdisplayname: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnIconPathChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newiconpath: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnSimpleVolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnSimpleVolumeChanged: usize,
    pub OnChannelVolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnGroupingParamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: AudioSessionState) -> ::windows::core::HRESULT,
    pub OnSessionDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionManager(::windows::core::IUnknown);
impl IAudioSessionManager {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__ = ::windows::core::zeroed::<IAudioSessionControl>();
        (::windows::core::Interface::vtable(self).GetAudioSessionControl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null())), streamflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume> {
        let mut result__ = ::windows::core::zeroed::<ISimpleAudioVolume>();
        (::windows::core::Interface::vtable(self).GetSimpleAudioVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null())), streamflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager {}
impl ::core::fmt::Debug for IAudioSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionManager {
    type Vtable = IAudioSessionManager_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa971f1_4d5e_40bb_935e_967039bfbee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAudioSessionControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, sessioncontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSimpleAudioVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, audiovolume: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionManager2(::windows::core::IUnknown);
impl IAudioSessionManager2 {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__ = ::windows::core::zeroed::<IAudioSessionControl>();
        (::windows::core::Interface::vtable(self).base__.GetAudioSessionControl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null())), streamflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume> {
        let mut result__ = ::windows::core::zeroed::<ISimpleAudioVolume>();
        (::windows::core::Interface::vtable(self).base__.GetSimpleAudioVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null())), streamflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSessionEnumerator(&self) -> ::windows::core::Result<IAudioSessionEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IAudioSessionEnumerator>();
        (::windows::core::Interface::vtable(self).GetSessionEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterSessionNotification<P0>(&self, sessionnotification: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSessionNotification>,
    {
        (::windows::core::Interface::vtable(self).RegisterSessionNotification)(::windows::core::Interface::as_raw(self), sessionnotification.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterSessionNotification<P0>(&self, sessionnotification: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSessionNotification>,
    {
        (::windows::core::Interface::vtable(self).UnregisterSessionNotification)(::windows::core::Interface::as_raw(self), sessionnotification.into_param().abi()).ok()
    }
    pub unsafe fn RegisterDuckNotification<P0, P1>(&self, sessionid: P0, ducknotification: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IAudioVolumeDuckNotification>,
    {
        (::windows::core::Interface::vtable(self).RegisterDuckNotification)(::windows::core::Interface::as_raw(self), sessionid.into_param().abi(), ducknotification.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterDuckNotification<P0>(&self, ducknotification: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioVolumeDuckNotification>,
    {
        (::windows::core::Interface::vtable(self).UnregisterDuckNotification)(::windows::core::Interface::as_raw(self), ducknotification.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionManager2, ::windows::core::IUnknown, IAudioSessionManager);
impl ::core::cmp::PartialEq for IAudioSessionManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager2 {}
impl ::core::fmt::Debug for IAudioSessionManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionManager2 {
    type Vtable = IAudioSessionManager2_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77aa99a0_1bd6_484f_8bc7_2c654c9a9b6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager2_Vtbl {
    pub base__: IAudioSessionManager_Vtbl,
    pub GetSessionEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionnotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionnotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterDuckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR, ducknotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterDuckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ducknotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionNotification(::windows::core::IUnknown);
impl IAudioSessionNotification {
    pub unsafe fn OnSessionCreated<P0>(&self, newsession: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSessionControl>,
    {
        (::windows::core::Interface::vtable(self).OnSessionCreated)(::windows::core::Interface::as_raw(self), newsession.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSessionNotification, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSessionNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionNotification {}
impl ::core::fmt::Debug for IAudioSessionNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionNotification {
    type Vtable = IAudioSessionNotification_Vtbl;
}
impl ::core::clone::Clone for IAudioSessionNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSessionNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x641dd20b_4d41_49cc_aba3_174b9477bb08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionNotification_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSessionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newsession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioStateMonitor(::windows::core::IUnknown);
impl IAudioStateMonitor {
    pub unsafe fn RegisterCallback(&self, callback: PAudioStateMonitorCallback, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).RegisterCallback)(::windows::core::Interface::as_raw(self), callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnregisterCallback(&self, registration: i64) {
        (::windows::core::Interface::vtable(self).UnregisterCallback)(::windows::core::Interface::as_raw(self), registration)
    }
    pub unsafe fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel {
        (::windows::core::Interface::vtable(self).GetSoundLevel)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IAudioStateMonitor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStateMonitor {}
impl ::core::fmt::Debug for IAudioStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStateMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
}
impl ::core::clone::Clone for IAudioStateMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioStateMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bd8738_e30d_4c77_bf5c_834e87c657e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: PAudioStateMonitorCallback, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows::core::HRESULT,
    pub UnregisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registration: i64),
    pub GetSoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> AudioStateMonitorSoundLevel,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioStreamVolume(::windows::core::IUnknown);
impl IAudioStreamVolume {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelVolume)(::windows::core::Interface::as_raw(self), dwindex, flevel).ok()
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetChannelVolume)(::windows::core::Interface::as_raw(self), dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllVolumes)(::windows::core::Interface::as_raw(self), pfvolumes.len() as _, ::core::mem::transmute(pfvolumes.as_ptr())).ok()
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAllVolumes)(::windows::core::Interface::as_raw(self), pfvolumes.len() as _, ::core::mem::transmute(pfvolumes.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioStreamVolume, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioStreamVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStreamVolume {}
impl ::core::fmt::Debug for IAudioStreamVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStreamVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioStreamVolume {
    type Vtable = IAudioStreamVolume_Vtbl;
}
impl ::core::clone::Clone for IAudioStreamVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioStreamVolume {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93014887_242d_4068_8a15_cf5e93b90fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamVolume_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32) -> ::windows::core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient(::windows::core::IUnknown);
impl IAudioSystemEffectsPropertyChangeNotificationClient {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPropertyChanged)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(key)).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSystemEffectsPropertyChangeNotificationClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyChangeNotificationClient {}
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyChangeNotificationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsPropertyChangeNotificationClient {
    type Vtable = IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl;
}
impl ::core::clone::Clone for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSystemEffectsPropertyChangeNotificationClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20049d40_56d5_400e_a2ef_385599feed49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OnPropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OnPropertyChanged: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffectsPropertyStore(::windows::core::IUnknown);
impl IAudioSystemEffectsPropertyStore {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::windows::core::zeroed::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>();
        (::windows::core::Interface::vtable(self).OpenDefaultPropertyStore)(::windows::core::Interface::as_raw(self), stgmaccess, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenUserPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::windows::core::zeroed::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>();
        (::windows::core::Interface::vtable(self).OpenUserPropertyStore)(::windows::core::Interface::as_raw(self), stgmaccess, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::windows::core::zeroed::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>();
        (::windows::core::Interface::vtable(self).OpenVolatilePropertyStore)(::windows::core::Interface::as_raw(self), stgmaccess, &mut result__).from_abi(result__)
    }
    pub unsafe fn ResetUserPropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetUserPropertyStore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResetVolatilePropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetVolatilePropertyStore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RegisterPropertyChangeNotification<P0>(&self, callback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        (::windows::core::Interface::vtable(self).RegisterPropertyChangeNotification)(::windows::core::Interface::as_raw(self), callback.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterPropertyChangeNotification<P0>(&self, callback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        (::windows::core::Interface::vtable(self).UnregisterPropertyChangeNotification)(::windows::core::Interface::as_raw(self), callback.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioSystemEffectsPropertyStore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyStore {}
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsPropertyStore {
    type Vtable = IAudioSystemEffectsPropertyStore_Vtbl;
}
impl ::core::clone::Clone for IAudioSystemEffectsPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioSystemEffectsPropertyStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x302ae7f9_d7e0_43e4_971b_1f8293613d2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenDefaultPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenDefaultPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenUserPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenUserPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenVolatilePropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenVolatilePropertyStore: usize,
    pub ResetUserPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResetVolatilePropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterPropertyChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterPropertyChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioTreble(::windows::core::IUnknown);
impl IAudioTreble {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLevelRange)(::windows::core::Interface::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).base__.GetLevel)(::windows::core::Interface::as_raw(self), nchannel, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevel)(::windows::core::Interface::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelUniform)(::windows::core::Interface::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelAllChannels)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioTreble, ::windows::core::IUnknown, IPerChannelDbLevel);
impl ::core::cmp::PartialEq for IAudioTreble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioTreble {}
impl ::core::fmt::Debug for IAudioTreble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioTreble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioTreble {
    type Vtable = IAudioTreble_Vtbl;
}
impl ::core::clone::Clone for IAudioTreble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioTreble {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a717812_694e_4907_b74b_bafa5cfdca7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTreble_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioViewManagerService(::windows::core::IUnknown);
impl IAudioViewManagerService {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAudioStreamWindow<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetAudioStreamWindow)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioViewManagerService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioViewManagerService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioViewManagerService {}
impl ::core::fmt::Debug for IAudioViewManagerService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioViewManagerService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioViewManagerService {
    type Vtable = IAudioViewManagerService_Vtbl;
}
impl ::core::clone::Clone for IAudioViewManagerService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioViewManagerService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a7ef10_1f49_45e0_ad35_612057cc8f74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioViewManagerService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAudioStreamWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAudioStreamWindow: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioVolumeDuckNotification(::windows::core::IUnknown);
impl IAudioVolumeDuckNotification {
    pub unsafe fn OnVolumeDuckNotification<P0>(&self, sessionid: P0, countcommunicationsessions: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnVolumeDuckNotification)(::windows::core::Interface::as_raw(self), sessionid.into_param().abi(), countcommunicationsessions).ok()
    }
    pub unsafe fn OnVolumeUnduckNotification<P0>(&self, sessionid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnVolumeUnduckNotification)(::windows::core::Interface::as_raw(self), sessionid.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioVolumeDuckNotification, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAudioVolumeDuckNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeDuckNotification {}
impl ::core::fmt::Debug for IAudioVolumeDuckNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeDuckNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioVolumeDuckNotification {
    type Vtable = IAudioVolumeDuckNotification_Vtbl;
}
impl ::core::clone::Clone for IAudioVolumeDuckNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioVolumeDuckNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3b284d4_6d39_4359_b3cf_b56ddb3bb39c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeDuckNotification_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnVolumeDuckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR, countcommunicationsessions: u32) -> ::windows::core::HRESULT,
    pub OnVolumeUnduckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioVolumeLevel(::windows::core::IUnknown);
impl IAudioVolumeLevel {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLevelRange)(::windows::core::Interface::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).base__.GetLevel)(::windows::core::Interface::as_raw(self), nchannel, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevel)(::windows::core::Interface::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelUniform)(::windows::core::Interface::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLevelAllChannels)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IAudioVolumeLevel, ::windows::core::IUnknown, IPerChannelDbLevel);
impl ::core::cmp::PartialEq for IAudioVolumeLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeLevel {}
impl ::core::fmt::Debug for IAudioVolumeLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioVolumeLevel {
    type Vtable = IAudioVolumeLevel_Vtbl;
}
impl ::core::clone::Clone for IAudioVolumeLevel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioVolumeLevel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fb7b48f_531d_44a2_bcb3_5ad5a134b3dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeLevel_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IChannelAudioVolume(::windows::core::IUnknown);
impl IChannelAudioVolume {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelVolume)(::windows::core::Interface::as_raw(self), dwindex, flevel, eventcontext).ok()
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetChannelVolume)(::windows::core::Interface::as_raw(self), dwindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32], eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllVolumes)(::windows::core::Interface::as_raw(self), pfvolumes.len() as _, ::core::mem::transmute(pfvolumes.as_ptr()), eventcontext).ok()
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAllVolumes)(::windows::core::Interface::as_raw(self), pfvolumes.len() as _, ::core::mem::transmute(pfvolumes.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IChannelAudioVolume, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IChannelAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelAudioVolume {}
impl ::core::fmt::Debug for IChannelAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelAudioVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChannelAudioVolume {
    type Vtable = IChannelAudioVolume_Vtbl;
}
impl ::core::clone::Clone for IChannelAudioVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IChannelAudioVolume {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c158861_b533_4b30_b1cf_e853e51c59b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelAudioVolume_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IConnector(::windows::core::IUnknown);
impl IConnector {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<ConnectorType> {
        let mut result__ = ::windows::core::zeroed::<ConnectorType>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataFlow(&self) -> ::windows::core::Result<DataFlow> {
        let mut result__ = ::windows::core::zeroed::<DataFlow>();
        (::windows::core::Interface::vtable(self).GetDataFlow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConnectTo<P0>(&self, pconnectto: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IConnector>,
    {
        (::windows::core::Interface::vtable(self).ConnectTo)(::windows::core::Interface::as_raw(self), pconnectto.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsConnected)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectedTo(&self) -> ::windows::core::Result<IConnector> {
        let mut result__ = ::windows::core::zeroed::<IConnector>();
        (::windows::core::Interface::vtable(self).GetConnectedTo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectorIdConnectedTo(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetConnectorIdConnectedTo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIdConnectedTo(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDeviceIdConnectedTo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IConnector, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnector {}
impl ::core::fmt::Debug for IConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConnector {
    type Vtable = IConnector_Vtbl;
}
impl ::core::clone::Clone for IConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c2c4058_23f5_41de_877a_df3af236a09e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut ConnectorType) -> ::windows::core::HRESULT,
    pub GetDataFlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflow: *mut DataFlow) -> ::windows::core::HRESULT,
    pub ConnectTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    pub GetConnectedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConnectorIdConnectedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrconnectorid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDeviceIdConnectedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IControlChangeNotify(::windows::core::IUnknown);
impl IControlChangeNotify {
    pub unsafe fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnNotify)(::windows::core::Interface::as_raw(self), dwsenderprocessid, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IControlChangeNotify, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IControlChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChangeNotify {}
impl ::core::fmt::Debug for IControlChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChangeNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IControlChangeNotify {
    type Vtable = IControlChangeNotify_Vtbl;
}
impl ::core::clone::Clone for IControlChangeNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlChangeNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa09513ed_c709_4d21_bd7b_5f34c47f3947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChangeNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IControlInterface(::windows::core::IUnknown);
impl IControlInterface {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetIID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IControlInterface, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IControlInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlInterface {}
impl ::core::fmt::Debug for IControlInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IControlInterface {
    type Vtable = IControlInterface_Vtbl;
}
impl ::core::clone::Clone for IControlInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlInterface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d37c3f_5140_444a_ae24_400789f3cbf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlInterface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IDeviceSpecificProperty(::windows::core::IUnknown);
impl IDeviceSpecificProperty {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetValue(&self, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), pvvalue, pcbvalue).ok()
    }
    pub unsafe fn SetValue(&self, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), pvvalue, cbvalue, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Get4BRange)(::windows::core::Interface::as_raw(self), plmin, plmax, plstepping).ok()
    }
}
::windows::imp::interface_hierarchy!(IDeviceSpecificProperty, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDeviceSpecificProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceSpecificProperty {}
impl ::core::fmt::Debug for IDeviceSpecificProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceSpecificProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDeviceSpecificProperty {
    type Vtable = IDeviceSpecificProperty_Vtbl;
}
impl ::core::clone::Clone for IDeviceSpecificProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceSpecificProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b22bcbf_2586_4af0_8583_205d391b807c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSpecificProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtype: *mut u16) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Get4BRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IDeviceTopology(::windows::core::IUnknown);
impl IDeviceTopology {
    pub unsafe fn GetConnectorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetConnectorCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnector(&self, nindex: u32) -> ::windows::core::Result<IConnector> {
        let mut result__ = ::windows::core::zeroed::<IConnector>();
        (::windows::core::Interface::vtable(self).GetConnector)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSubunitCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSubunitCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSubunit(&self, nindex: u32) -> ::windows::core::Result<ISubunit> {
        let mut result__ = ::windows::core::zeroed::<ISubunit>();
        (::windows::core::Interface::vtable(self).GetSubunit)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPartById(&self, nid: u32) -> ::windows::core::Result<IPart> {
        let mut result__ = ::windows::core::zeroed::<IPart>();
        (::windows::core::Interface::vtable(self).GetPartById)(::windows::core::Interface::as_raw(self), nid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDeviceId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignalPath<P0, P1, P2>(&self, pipartfrom: P0, pipartto: P1, brejectmixedpaths: P2) -> ::windows::core::Result<IPartsList>
    where
        P0: ::windows::core::IntoParam<IPart>,
        P1: ::windows::core::IntoParam<IPart>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IPartsList>();
        (::windows::core::Interface::vtable(self).GetSignalPath)(::windows::core::Interface::as_raw(self), pipartfrom.into_param().abi(), pipartto.into_param().abi(), brejectmixedpaths.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDeviceTopology, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDeviceTopology {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceTopology {}
impl ::core::fmt::Debug for IDeviceTopology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceTopology").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDeviceTopology {
    type Vtable = IDeviceTopology_Vtbl;
}
impl ::core::clone::Clone for IDeviceTopology {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceTopology {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a07407e_6497_4a18_9787_32f79bd0d98f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceTopology_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetConnectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSubunitCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSubunit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppsubunit: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPartById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nid: u32, pppart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSignalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipartfrom: *mut ::core::ffi::c_void, pipartto: *mut ::core::ffi::c_void, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSignalPath: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IKsJackDescription3(::windows::core::IUnknown);
impl IKsJackDescription3 {
    pub unsafe fn GetJackCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetJackCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub unsafe fn GetJackDescription3(&self, njack: u32) -> ::windows::core::Result<super::KernelStreaming::KSJACK_DESCRIPTION3> {
        let mut result__ = ::windows::core::zeroed::<super::KernelStreaming::KSJACK_DESCRIPTION3>();
        (::windows::core::Interface::vtable(self).GetJackDescription3)(::windows::core::Interface::as_raw(self), njack, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IKsJackDescription3, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IKsJackDescription3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackDescription3 {}
impl ::core::fmt::Debug for IKsJackDescription3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackDescription3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKsJackDescription3 {
    type Vtable = IKsJackDescription3_Vtbl;
}
impl ::core::clone::Clone for IKsJackDescription3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IKsJackDescription3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3f6778b_6660_4cc8_a291_ecc4192d9967);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetJackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub GetJackDescription3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, njack: u32, pdescription3: *mut super::KernelStreaming::KSJACK_DESCRIPTION3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_KernelStreaming"))]
    GetJackDescription3: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDevice(::windows::core::IUnknown);
impl IMMDevice {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Activate<T>(&self, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, dwclsctx, ::core::mem::transmute(pactivationparams.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OpenPropertyStore(&self, stgmaccess: super::super::System::Com::STGM) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::windows::core::zeroed::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>();
        (::windows::core::Interface::vtable(self).OpenPropertyStore)(::windows::core::Interface::as_raw(self), stgmaccess, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMMDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMMDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDevice {}
impl ::core::fmt::Debug for IMMDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDevice {
    type Vtable = IMMDevice_Vtbl;
}
impl ::core::clone::Clone for IMMDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMMDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd666063f_1587_4e43_81f1_b948e807363f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Activate: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub OpenPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: super::super::System::Com::STGM, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    OpenPropertyStore: usize,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstrid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDeviceActivator(::windows::core::IUnknown);
impl IMMDeviceActivator {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Activate<P0>(&self, iid: *const ::windows::core::GUID, pdevice: P0, pactivationparams: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMMDevice>,
    {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), iid, pdevice.into_param().abi(), ::core::mem::transmute(pactivationparams.unwrap_or(::std::ptr::null())), ppinterface).ok()
    }
}
::windows::imp::interface_hierarchy!(IMMDeviceActivator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMMDeviceActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceActivator {}
impl ::core::fmt::Debug for IMMDeviceActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceActivator {
    type Vtable = IMMDeviceActivator_Vtbl;
}
impl ::core::clone::Clone for IMMDeviceActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMMDeviceActivator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0d0ea4_d0a9_4b0e_935b_09516746fac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceActivator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, pdevice: *mut ::core::ffi::c_void, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Activate: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDeviceCollection(::windows::core::IUnknown);
impl IMMDeviceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Item(&self, ndevice: u32) -> ::windows::core::Result<IMMDevice> {
        let mut result__ = ::windows::core::zeroed::<IMMDevice>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), ndevice, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMMDeviceCollection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMMDeviceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceCollection {}
impl ::core::fmt::Debug for IMMDeviceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceCollection {
    type Vtable = IMMDeviceCollection_Vtbl;
}
impl ::core::clone::Clone for IMMDeviceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMMDeviceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdevices: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ndevice: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDeviceEnumerator(::windows::core::IUnknown);
impl IMMDeviceEnumerator {
    pub unsafe fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: u32) -> ::windows::core::Result<IMMDeviceCollection> {
        let mut result__ = ::windows::core::zeroed::<IMMDeviceCollection>();
        (::windows::core::Interface::vtable(self).EnumAudioEndpoints)(::windows::core::Interface::as_raw(self), dataflow, dwstatemask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> ::windows::core::Result<IMMDevice> {
        let mut result__ = ::windows::core::zeroed::<IMMDevice>();
        (::windows::core::Interface::vtable(self).GetDefaultAudioEndpoint)(::windows::core::Interface::as_raw(self), dataflow, role, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<P0>(&self, pwstrid: P0) -> ::windows::core::Result<IMMDevice>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMMDevice>();
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), pwstrid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMMNotificationClient>,
    {
        (::windows::core::Interface::vtable(self).RegisterEndpointNotificationCallback)(::windows::core::Interface::as_raw(self), pclient.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMMNotificationClient>,
    {
        (::windows::core::Interface::vtable(self).UnregisterEndpointNotificationCallback)(::windows::core::Interface::as_raw(self), pclient.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMMDeviceEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMMDeviceEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceEnumerator {}
impl ::core::fmt::Debug for IMMDeviceEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceEnumerator {
    type Vtable = IMMDeviceEnumerator_Vtbl;
}
impl ::core::clone::Clone for IMMDeviceEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMMDeviceEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa95664d2_9614_4f35_a746_de8db63617e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumAudioEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterEndpointNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclient: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterEndpointNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclient: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMEndpoint(::windows::core::IUnknown);
impl IMMEndpoint {
    pub unsafe fn GetDataFlow(&self) -> ::windows::core::Result<EDataFlow> {
        let mut result__ = ::windows::core::zeroed::<EDataFlow>();
        (::windows::core::Interface::vtable(self).GetDataFlow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMMEndpoint, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMMEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMEndpoint {}
impl ::core::fmt::Debug for IMMEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMEndpoint {
    type Vtable = IMMEndpoint_Vtbl;
}
impl ::core::clone::Clone for IMMEndpoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMMEndpoint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be09788_6894_4089_8586_9a2a6c265ac5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMEndpoint_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDataFlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataflow: *mut EDataFlow) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMNotificationClient(::windows::core::IUnknown);
impl IMMNotificationClient {
    pub unsafe fn OnDeviceStateChanged<P0>(&self, pwstrdeviceid: P0, dwnewstate: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnDeviceStateChanged)(::windows::core::Interface::as_raw(self), pwstrdeviceid.into_param().abi(), dwnewstate).ok()
    }
    pub unsafe fn OnDeviceAdded<P0>(&self, pwstrdeviceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnDeviceAdded)(::windows::core::Interface::as_raw(self), pwstrdeviceid.into_param().abi()).ok()
    }
    pub unsafe fn OnDeviceRemoved<P0>(&self, pwstrdeviceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnDeviceRemoved)(::windows::core::Interface::as_raw(self), pwstrdeviceid.into_param().abi()).ok()
    }
    pub unsafe fn OnDefaultDeviceChanged<P0>(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnDefaultDeviceChanged)(::windows::core::Interface::as_raw(self), flow, role, pwstrdefaultdeviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OnPropertyValueChanged<P0>(&self, pwstrdeviceid: P0, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnPropertyValueChanged)(::windows::core::Interface::as_raw(self), pwstrdeviceid.into_param().abi(), ::core::mem::transmute(key)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMMNotificationClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMMNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMNotificationClient {}
impl ::core::fmt::Debug for IMMNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMNotificationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMNotificationClient {
    type Vtable = IMMNotificationClient_Vtbl;
}
impl ::core::clone::Clone for IMMNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMMNotificationClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7991eec9_7e89_4d85_8390_6c703cec60c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMNotificationClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnDeviceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR, dwnewstate: u32) -> ::windows::core::HRESULT,
    pub OnDeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnDeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnDefaultDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OnPropertyValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OnPropertyValueChanged: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMessageFilter(::windows::core::IUnknown);
impl IMessageFilter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleInComingCall<P0>(&self, dwcalltype: u32, htaskcaller: P0, dwtickcount: u32, lpinterfaceinfo: ::core::option::Option<*const super::super::System::Com::INTERFACEINFO>) -> u32
    where
        P0: ::windows::core::IntoParam<super::HTASK>,
    {
        (::windows::core::Interface::vtable(self).HandleInComingCall)(::windows::core::Interface::as_raw(self), dwcalltype, htaskcaller.into_param().abi(), dwtickcount, ::core::mem::transmute(lpinterfaceinfo.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn RetryRejectedCall<P0>(&self, htaskcallee: P0, dwtickcount: u32, dwrejecttype: u32) -> u32
    where
        P0: ::windows::core::IntoParam<super::HTASK>,
    {
        (::windows::core::Interface::vtable(self).RetryRejectedCall)(::windows::core::Interface::as_raw(self), htaskcallee.into_param().abi(), dwtickcount, dwrejecttype)
    }
    pub unsafe fn MessagePending<P0>(&self, htaskcallee: P0, dwtickcount: u32, dwpendingtype: u32) -> u32
    where
        P0: ::windows::core::IntoParam<super::HTASK>,
    {
        (::windows::core::Interface::vtable(self).MessagePending)(::windows::core::Interface::as_raw(self), htaskcallee.into_param().abi(), dwtickcount, dwpendingtype)
    }
}
::windows::imp::interface_hierarchy!(IMessageFilter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMessageFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageFilter {}
impl ::core::fmt::Debug for IMessageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMessageFilter {
    type Vtable = IMessageFilter_Vtbl;
}
impl ::core::clone::Clone for IMessageFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000016_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleInComingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleInComingCall: usize,
    pub RetryRejectedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32,
    pub MessagePending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IPart(::windows::core::IUnknown);
impl IPart {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLocalId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLocalId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlobalId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetGlobalId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPartType(&self) -> ::windows::core::Result<PartType> {
        let mut result__ = ::windows::core::zeroed::<PartType>();
        (::windows::core::Interface::vtable(self).GetPartType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSubType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetSubType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetControlInterfaceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetControlInterfaceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetControlInterface(&self, nindex: u32) -> ::windows::core::Result<IControlInterface> {
        let mut result__ = ::windows::core::zeroed::<IControlInterface>();
        (::windows::core::Interface::vtable(self).GetControlInterface)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumPartsIncoming(&self) -> ::windows::core::Result<IPartsList> {
        let mut result__ = ::windows::core::zeroed::<IPartsList>();
        (::windows::core::Interface::vtable(self).EnumPartsIncoming)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumPartsOutgoing(&self) -> ::windows::core::Result<IPartsList> {
        let mut result__ = ::windows::core::zeroed::<IPartsList>();
        (::windows::core::Interface::vtable(self).EnumPartsOutgoing)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTopologyObject(&self) -> ::windows::core::Result<IDeviceTopology> {
        let mut result__ = ::windows::core::zeroed::<IDeviceTopology>();
        (::windows::core::Interface::vtable(self).GetTopologyObject)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Activate(&self, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), dwclscontext, refiid, ::core::mem::transmute(ppvobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn RegisterControlChangeCallback<P0>(&self, riid: *const ::windows::core::GUID, pnotify: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IControlChangeNotify>,
    {
        (::windows::core::Interface::vtable(self).RegisterControlChangeCallback)(::windows::core::Interface::as_raw(self), riid, pnotify.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeCallback<P0>(&self, pnotify: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IControlChangeNotify>,
    {
        (::windows::core::Interface::vtable(self).UnregisterControlChangeCallback)(::windows::core::Interface::as_raw(self), pnotify.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPart, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPart {}
impl ::core::fmt::Debug for IPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPart").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPart {
    type Vtable = IPart_Vtbl;
}
impl ::core::clone::Clone for IPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPart {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPart_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnid: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlobalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrglobalid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPartType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparttype: *mut PartType) -> ::windows::core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetControlInterfaceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetControlInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumPartsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumPartsOutgoing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTopologyObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptopology: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterControlChangeCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterControlChangeCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IPartsList(::windows::core::IUnknown);
impl IPartsList {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPart(&self, nindex: u32) -> ::windows::core::Result<IPart> {
        let mut result__ = ::windows::core::zeroed::<IPart>();
        (::windows::core::Interface::vtable(self).GetPart)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPartsList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPartsList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartsList {}
impl ::core::fmt::Debug for IPartsList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartsList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPartsList {
    type Vtable = IPartsList_Vtbl;
}
impl ::core::clone::Clone for IPartsList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPartsList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartsList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pppart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IPerChannelDbLevel(::windows::core::IUnknown);
impl IPerChannelDbLevel {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLevelRange)(::windows::core::Interface::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetLevel)(::windows::core::Interface::as_raw(self), nchannel, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLevel)(::windows::core::Interface::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLevelUniform)(::windows::core::Interface::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLevelAllChannels)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IPerChannelDbLevel, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPerChannelDbLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerChannelDbLevel {}
impl ::core::fmt::Debug for IPerChannelDbLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerChannelDbLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPerChannelDbLevel {
    type Vtable = IPerChannelDbLevel_Vtbl;
}
impl ::core::clone::Clone for IPerChannelDbLevel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPerChannelDbLevel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerChannelDbLevel_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub GetLevelRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetLevelUniform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetLevelAllChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISimpleAudioVolume(::windows::core::IUnknown);
impl ISimpleAudioVolume {
    pub unsafe fn SetMasterVolume(&self, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMasterVolume)(::windows::core::Interface::as_raw(self), flevel, eventcontext).ok()
    }
    pub unsafe fn GetMasterVolume(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).GetMasterVolume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<P0>(&self, bmute: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMute)(::windows::core::Interface::as_raw(self), bmute.into_param().abi(), eventcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetMute)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISimpleAudioVolume, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimpleAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleAudioVolume {}
impl ::core::fmt::Debug for ISimpleAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleAudioVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimpleAudioVolume {
    type Vtable = ISimpleAudioVolume_Vtbl;
}
impl ::core::clone::Clone for ISimpleAudioVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleAudioVolume {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87ce5498_68d6_44e5_9215_6da47ef883d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleAudioVolume_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetMasterVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMasterVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMute: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioClient(::windows::core::IUnknown);
impl ISpatialAudioClient {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStaticObjectPosition)(::windows::core::Interface::as_raw(self), r#type, x, y, z).ok()
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::windows::core::zeroed::<AudioObjectType>();
        (::windows::core::Interface::vtable(self).GetNativeStaticObjectTypeMask)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxDynamicObjectCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IAudioFormatEnumerator>();
        (::windows::core::Interface::vtable(self).GetSupportedAudioObjectFormatEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxFrameCount)(::windows::core::Interface::as_raw(self), objectformat, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsAudioObjectFormatSupported)(::windows::core::Interface::as_raw(self), objectformat).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsSpatialAudioStreamAvailable)(::windows::core::Interface::as_raw(self), streamuuid, ::core::mem::transmute(auxiliaryinfo.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ActivateSpatialAudioStream<T>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioStream)(::windows::core::Interface::as_raw(self), activationparams, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioClient {}
impl ::core::fmt::Debug for ISpatialAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioClient {
    type Vtable = ISpatialAudioClient_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf8e066_aaaa_49be_9a4d_fd2a858ea27f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStaticObjectPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::HRESULT,
    pub GetNativeStaticObjectTypeMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub GetMaxDynamicObjectCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
    pub GetSupportedAudioObjectFormatEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMaxFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub IsAudioObjectFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub IsSpatialAudioStreamAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    IsSpatialAudioStreamAvailable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub ActivateSpatialAudioStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    ActivateSpatialAudioStream: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioClient2(::windows::core::IUnknown);
impl ISpatialAudioClient2 {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStaticObjectPosition)(::windows::core::Interface::as_raw(self), r#type, x, y, z).ok()
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::windows::core::zeroed::<AudioObjectType>();
        (::windows::core::Interface::vtable(self).base__.GetNativeStaticObjectTypeMask)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxDynamicObjectCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IAudioFormatEnumerator>();
        (::windows::core::Interface::vtable(self).base__.GetSupportedAudioObjectFormatEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMaxFrameCount)(::windows::core::Interface::as_raw(self), objectformat, &mut result__).from_abi(result__)
    }
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsAudioObjectFormatSupported)(::windows::core::Interface::as_raw(self), objectformat).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsSpatialAudioStreamAvailable)(::windows::core::Interface::as_raw(self), streamuuid, ::core::mem::transmute(auxiliaryinfo.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ActivateSpatialAudioStream<T>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.ActivateSpatialAudioStream)(::windows::core::Interface::as_raw(self), activationparams, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsOffloadCapable)(::windows::core::Interface::as_raw(self), category, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMaxFrameCountForCategory<P0>(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: P0, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxFrameCountForCategory)(::windows::core::Interface::as_raw(self), category, offloadenabled.into_param().abi(), objectformat, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioClient2, ::windows::core::IUnknown, ISpatialAudioClient);
impl ::core::cmp::PartialEq for ISpatialAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioClient2 {}
impl ::core::fmt::Debug for ISpatialAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioClient2 {
    type Vtable = ISpatialAudioClient2_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioClient2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaabe452_a66a_4bee_a93e_e320463f6a53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient2_Vtbl {
    pub base__: ISpatialAudioClient_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOffloadCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOffloadCapable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMaxFrameCountForCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMaxFrameCountForCategory: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataClient(::windows::core::IUnknown);
impl ISpatialAudioMetadataClient {
    pub unsafe fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: ::core::option::Option<*mut ::core::option::Option<ISpatialAudioMetadataItemsBuffer>>, metadataitems: *mut ::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataItems)(::windows::core::Interface::as_raw(self), maxitemcount, framecount, ::core::mem::transmute(metadataitemsbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(metadataitems)).ok()
    }
    pub unsafe fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSpatialAudioMetadataItemsBufferLength)(::windows::core::Interface::as_raw(self), maxitemcount, &mut result__).from_abi(result__)
    }
    pub unsafe fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> ::windows::core::Result<ISpatialAudioMetadataWriter> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioMetadataWriter>();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataWriter)(::windows::core::Interface::as_raw(self), overflowmode, &mut result__).from_abi(result__)
    }
    pub unsafe fn ActivateSpatialAudioMetadataCopier(&self) -> ::windows::core::Result<ISpatialAudioMetadataCopier> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioMetadataCopier>();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataCopier)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ActivateSpatialAudioMetadataReader(&self) -> ::windows::core::Result<ISpatialAudioMetadataReader> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioMetadataReader>();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataReader)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioMetadataClient, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioMetadataClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataClient {}
impl ::core::fmt::Debug for ISpatialAudioMetadataClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataClient {
    type Vtable = ISpatialAudioMetadataClient_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioMetadataClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioMetadataClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x777d4a3b_f6ff_4a26_85dc_68d7cdeda1d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ActivateSpatialAudioMetadataItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut *mut ::core::ffi::c_void, metadataitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSpatialAudioMetadataItemsBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioMetadataWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioMetadataCopier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadatacopier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioMetadataReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadatareader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataCopier(::windows::core::IUnknown);
impl ISpatialAudioMetadataCopier {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpatialAudioMetadataItems>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), metadataitems.into_param().abi()).ok()
    }
    pub unsafe fn CopyMetadataForFrames<P0>(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: P0) -> ::windows::core::Result<u16>
    where
        P0: ::windows::core::IntoParam<ISpatialAudioMetadataItems>,
    {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).CopyMetadataForFrames)(::windows::core::Interface::as_raw(self), copyframecount, copymode, dstmetadataitems.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioMetadataCopier, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioMetadataCopier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataCopier {}
impl ::core::fmt::Debug for ISpatialAudioMetadataCopier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataCopier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataCopier {
    type Vtable = ISpatialAudioMetadataCopier_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioMetadataCopier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioMetadataCopier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd224b233_e251_4fd0_9ca2_d5ecf9a68404);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataCopier_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyMetadataForFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: *mut ::core::ffi::c_void, itemscopied: *mut u16) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataItems(::windows::core::IUnknown);
impl ISpatialAudioMetadataItems {
    pub unsafe fn GetFrameCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetFrameCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItemCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetItemCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxItemCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetMaxItemCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxValueBufferLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxValueBufferLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<SpatialAudioMetadataItemsInfo> {
        let mut result__ = ::windows::core::zeroed::<SpatialAudioMetadataItemsInfo>();
        (::windows::core::Interface::vtable(self).GetInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioMetadataItems, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataItems {
    type Vtable = ISpatialAudioMetadataItems_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioMetadataItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioMetadataItems {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcd7c78f_3098_4f22_b547_a2f25a381269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItems_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framecount: *mut u16) -> ::windows::core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxitemcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxValueBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxvaluebufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataItemsBuffer(::windows::core::IUnknown);
impl ISpatialAudioMetadataItemsBuffer {
    pub unsafe fn AttachToBuffer(&self, buffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachToBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _).ok()
    }
    pub unsafe fn AttachToPopulatedBuffer(&self, buffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachToPopulatedBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _).ok()
    }
    pub unsafe fn DetachBuffer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DetachBuffer)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioMetadataItemsBuffer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItemsBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItemsBuffer {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItemsBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItemsBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataItemsBuffer {
    type Vtable = ISpatialAudioMetadataItemsBuffer_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioMetadataItemsBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioMetadataItemsBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42640a16_e1bd_42d9_9ff6_031ab71a2dba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AttachToBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT,
    pub AttachToPopulatedBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataReader(::windows::core::IUnknown);
impl ISpatialAudioMetadataReader {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpatialAudioMetadataItems>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), metadataitems.into_param().abi()).ok()
    }
    pub unsafe fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadNextItem)(::windows::core::Interface::as_raw(self), commandcount, frameoffset).ok()
    }
    pub unsafe fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadNextItemCommand)(::windows::core::Interface::as_raw(self), commandid, valuebuffer, maxvaluebufferlength, valuebufferlength).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioMetadataReader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioMetadataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataReader {}
impl ::core::fmt::Debug for ISpatialAudioMetadataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataReader {
    type Vtable = ISpatialAudioMetadataReader_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioMetadataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioMetadataReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb78e86a2_31d9_4c32_94d2_7df40fc7ebec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReadNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::HRESULT,
    pub ReadNextItemCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataWriter(::windows::core::IUnknown);
impl ISpatialAudioMetadataWriter {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpatialAudioMetadataItems>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), metadataitems.into_param().abi()).ok()
    }
    pub unsafe fn WriteNextItem(&self, frameoffset: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNextItem)(::windows::core::Interface::as_raw(self), frameoffset).ok()
    }
    pub unsafe fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: ::core::option::Option<*const ::core::ffi::c_void>, valuebufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNextItemCommand)(::windows::core::Interface::as_raw(self), commandid, ::core::mem::transmute(valuebuffer.unwrap_or(::std::ptr::null())), valuebufferlength).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioMetadataWriter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataWriter {}
impl ::core::fmt::Debug for ISpatialAudioMetadataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataWriter {
    type Vtable = ISpatialAudioMetadataWriter_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioMetadataWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b17ca01_2955_444d_a430_537dc589a844);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameoffset: u16) -> ::windows::core::HRESULT,
    pub WriteNextItemCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObject(::windows::core::IUnknown);
impl ISpatialAudioObject {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEndOfStream)(::windows::core::Interface::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsActive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::windows::core::zeroed::<AudioObjectType>();
        (::windows::core::Interface::vtable(self).base__.GetAudioObjectType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPosition)(::windows::core::Interface::as_raw(self), x, y, z).ok()
    }
    pub unsafe fn SetVolume(&self, volume: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolume)(::windows::core::Interface::as_raw(self), volume).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObject, ::windows::core::IUnknown, ISpatialAudioObjectBase);
impl ::core::cmp::PartialEq for ISpatialAudioObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObject {}
impl ::core::fmt::Debug for ISpatialAudioObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObject {
    type Vtable = ISpatialAudioObject_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdde28967_521b_46e5_8f00_bd6f2bc8ab1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObject_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectBase(::windows::core::IUnknown);
impl ISpatialAudioObjectBase {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEndOfStream)(::windows::core::Interface::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsActive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::windows::core::zeroed::<AudioObjectType>();
        (::windows::core::Interface::vtable(self).GetAudioObjectType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectBase, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioObjectBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectBase {
    type Vtable = ISpatialAudioObjectBase_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectBase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcce0b8f2_8d4d_4efb_a8cf_3d6ecf1c30e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectBase_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub SetEndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsActive: usize,
    pub GetAudioObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectForHrtf(::windows::core::IUnknown);
impl ISpatialAudioObjectForHrtf {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEndOfStream)(::windows::core::Interface::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsActive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::windows::core::zeroed::<AudioObjectType>();
        (::windows::core::Interface::vtable(self).base__.GetAudioObjectType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPosition)(::windows::core::Interface::as_raw(self), x, y, z).ok()
    }
    pub unsafe fn SetGain(&self, gain: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGain)(::windows::core::Interface::as_raw(self), gain).ok()
    }
    pub unsafe fn SetOrientation(&self, orientation: *const *const f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOrientation)(::windows::core::Interface::as_raw(self), orientation).ok()
    }
    pub unsafe fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnvironment)(::windows::core::Interface::as_raw(self), environment).ok()
    }
    pub unsafe fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDistanceDecay)(::windows::core::Interface::as_raw(self), distancedecay).ok()
    }
    pub unsafe fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDirectivity)(::windows::core::Interface::as_raw(self), directivity).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectForHrtf, ::windows::core::IUnknown, ISpatialAudioObjectBase);
impl ::core::cmp::PartialEq for ISpatialAudioObjectForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForHrtf").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForHrtf {
    type Vtable = ISpatialAudioObjectForHrtf_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectForHrtf {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectForHrtf {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7436ade_1978_4e14_aba0_555bd8eb83b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForHrtf_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, orientation: *const *const f32) -> ::windows::core::HRESULT,
    pub SetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::HRESULT,
    pub SetDistanceDecay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::HRESULT,
    pub SetDirectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectForMetadataCommands(::windows::core::IUnknown);
impl ISpatialAudioObjectForMetadataCommands {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEndOfStream)(::windows::core::Interface::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsActive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::windows::core::zeroed::<AudioObjectType>();
        (::windows::core::Interface::vtable(self).base__.GetAudioObjectType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: ::core::option::Option<*const ::core::ffi::c_void>, valuebufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNextMetadataCommand)(::windows::core::Interface::as_raw(self), commandid, ::core::mem::transmute(valuebuffer.unwrap_or(::std::ptr::null())), valuebufferlength).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectForMetadataCommands, ::windows::core::IUnknown, ISpatialAudioObjectBase);
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataCommands {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataCommands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataCommands").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForMetadataCommands {
    type Vtable = ISpatialAudioObjectForMetadataCommands_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectForMetadataCommands {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectForMetadataCommands {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0df2c94b_f5f9_472d_af6b_c46e0ac9cd05);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub WriteNextMetadataCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectForMetadataItems(::windows::core::IUnknown);
impl ISpatialAudioObjectForMetadataItems {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBuffer)(::windows::core::Interface::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEndOfStream)(::windows::core::Interface::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsActive)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::windows::core::zeroed::<AudioObjectType>();
        (::windows::core::Interface::vtable(self).base__.GetAudioObjectType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSpatialAudioMetadataItems(&self) -> ::windows::core::Result<ISpatialAudioMetadataItems> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioMetadataItems>();
        (::windows::core::Interface::vtable(self).GetSpatialAudioMetadataItems)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectForMetadataItems, ::windows::core::IUnknown, ISpatialAudioObjectBase);
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForMetadataItems {
    type Vtable = ISpatialAudioObjectForMetadataItems_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectForMetadataItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectForMetadataItems {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddea49ff_3bc0_4377_8aad_9fbcfd808566);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataItems_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub GetSpatialAudioMetadataItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStream(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStream {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetAvailableDynamicObjectCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetService)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginUpdatingAudioObjects)(::windows::core::Interface::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndUpdatingAudioObjects)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObject> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioObject>();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObject)(::windows::core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectRenderStream, ::windows::core::IUnknown, ISpatialAudioObjectRenderStreamBase);
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStream {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStream {
    type Vtable = ISpatialAudioObjectRenderStream_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectRenderStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab5f473_b423_477b_85f5_b5a332a04153);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStream_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamBase(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamBase {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetAvailableDynamicObjectCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetService)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUpdatingAudioObjects)(::windows::core::Interface::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndUpdatingAudioObjects)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamBase, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamBase {
    type Vtable = ISpatialAudioObjectRenderStreamBase_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectRenderStreamBase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeaaf403_c1d8_450d_aa05_e0ccee7502a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAvailableDynamicObjectCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginUpdatingAudioObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub EndUpdatingAudioObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamForHrtf(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamForHrtf {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetAvailableDynamicObjectCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetService)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginUpdatingAudioObjects)(::windows::core::Interface::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndUpdatingAudioObjects)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForHrtf> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioObjectForHrtf>();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObjectForHrtf)(::windows::core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamForHrtf, ::windows::core::IUnknown, ISpatialAudioObjectRenderStreamBase);
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForHrtf").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamForHrtf {
    type Vtable = ISpatialAudioObjectRenderStreamForHrtf_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamForHrtf {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectRenderStreamForHrtf {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe08deef9_5363_406e_9fdc_080ee247bbe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForHrtf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamForMetadata(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamForMetadata {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetAvailableDynamicObjectCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetService)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginUpdatingAudioObjects)(::windows::core::Interface::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndUpdatingAudioObjects)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataCommands> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioObjectForMetadataCommands>();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataCommands)(::windows::core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataItems> {
        let mut result__ = ::windows::core::zeroed::<ISpatialAudioObjectForMetadataItems>();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataItems)(::windows::core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamForMetadata, ::windows::core::IUnknown, ISpatialAudioObjectRenderStreamBase);
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForMetadata {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamForMetadata {
    type Vtable = ISpatialAudioObjectRenderStreamForMetadata_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamForMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectRenderStreamForMetadata {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbc9c907_48d5_4a2e_a0c7_f7f0d67c1fb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForMetadataCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioObjectForMetadataItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamNotify(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamNotify {
    pub unsafe fn OnAvailableDynamicObjectCountChange<P0>(&self, sender: P0, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpatialAudioObjectRenderStreamBase>,
    {
        (::windows::core::Interface::vtable(self).OnAvailableDynamicObjectCountChange)(::windows::core::Interface::as_raw(self), sender.into_param().abi(), hnscompliancedeadlinetime, availabledynamicobjectcountchange).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamNotify, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamNotify {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamNotify {
    type Vtable = ISpatialAudioObjectRenderStreamNotify_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioObjectRenderStreamNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdddf83e6_68d7_4c70_883f_a1836afb4a50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnAvailableDynamicObjectCountChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISubunit(::windows::core::IUnknown);
impl ISubunit {}
::windows::imp::interface_hierarchy!(ISubunit, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISubunit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISubunit {}
impl ::core::fmt::Debug for ISubunit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISubunit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISubunit {
    type Vtable = ISubunit_Vtbl;
}
impl ::core::clone::Clone for ISubunit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISubunit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82149a85_dba6_4487_86bb_ea8f7fefcc71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubunit_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_USER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_BASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_BUSY: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_CANCELED: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELP: ::windows::core::PCWSTR = ::windows::core::w!("acmchoose_contexthelp");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELPA: ::windows::core::PCSTR = ::windows::core::s!("acmchoose_contexthelp");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELPW: ::windows::core::PCWSTR = ::windows::core::w!("acmchoose_contexthelp");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENU: ::windows::core::PCWSTR = ::windows::core::w!("acmchoose_contextmenu");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENUA: ::windows::core::PCSTR = ::windows::core::s!("acmchoose_contextmenu");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENUW: ::windows::core::PCWSTR = ::windows::core::w!("acmchoose_contextmenu");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRING: ::windows::core::PCWSTR = ::windows::core::w!("acmchoose_help");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRINGA: ::windows::core::PCSTR = ::windows::core::s!("acmchoose_help");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("acmchoose_help");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287486i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287469i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287464i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287477i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287466i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287463i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287482i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287465i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_DEVICE_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287484i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_DEVICE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287478i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287423i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287422i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287473i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287454i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287447i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287448i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287471i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287468i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287474i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287470i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_HEADTRACKING_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287440i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287424i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287467i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287456i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287479i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_STREAM_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287455i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287451i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287487i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NOT_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287483i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287452i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287453i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287481i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287449i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_RESOURCES_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287450i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287472i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287476i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287480i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287485i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_BUFFER_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(143196161i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_POSITION_STALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(143196163i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(143196162i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_AUXIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVINTERFACE_AUDIO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVINTERFACE_AUDIO_RENDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVINTERFACE_MIDI_INPUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVINTERFACE_MIDI_OUTPUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRVM_MAPPER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DeviceTopology: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const EVENTCONTEXT_VOLUMESLIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_LONG: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_SHORT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_INQUEUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_ISSTRM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_STREAM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_LASTERROR: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NODEVICE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NOMAP: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NOTREADY: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPATCHSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_GET: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_SET: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_TEMPO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDISTRM_ERROR: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_QUERY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_UNCACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALLINE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALVALUE: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_LASTERROR: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MMDeviceEnumerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_FMSYNTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_MAPPER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_MIDIPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SQSYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SWSYNTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SYNTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_WAVETABLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_RING: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_DESTROYED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287232i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287227i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_INTERNAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287219i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_INVALID_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287224i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287226i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287229i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287220i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287231i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287228i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287230i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287221i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287225i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287222i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286956i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286969i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286968i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286940i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286942i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286976i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286955i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286941i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286967i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286952i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286974i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286957i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286939i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286959i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286951i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286971i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286973i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286954i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286944i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286960i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286958i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286943i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286970i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286953i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286975i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286972i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: ::windows::core::PCWSTR = ::windows::core::w!("VAD\\Process_Loopback");
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_PITCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_SYNC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_BADFORMAT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_LASTERROR: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_STILLPLAYING: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_SYNC: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_UNPREPARED: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1M08: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1M16: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1S08: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1S16: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2M08: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2M16: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2S08: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2S16: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44M08: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44S08: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4M08: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4S08: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_PCM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_BEGINLOOP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_ENDLOOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_INQUEUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = AMBISONICS_CHANNEL_ORDERING(0i32);
impl ::core::marker::Copy for AMBISONICS_CHANNEL_ORDERING {}
impl ::core::clone::Clone for AMBISONICS_CHANNEL_ORDERING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_CHANNEL_ORDERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AMBISONICS_CHANNEL_ORDERING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AMBISONICS_CHANNEL_ORDERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_CHANNEL_ORDERING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AMBISONICS_NORMALIZATION(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(1i32);
impl ::core::marker::Copy for AMBISONICS_NORMALIZATION {}
impl ::core::clone::Clone for AMBISONICS_NORMALIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_NORMALIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AMBISONICS_NORMALIZATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AMBISONICS_NORMALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_NORMALIZATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AMBISONICS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = AMBISONICS_TYPE(0i32);
impl ::core::marker::Copy for AMBISONICS_TYPE {}
impl ::core::clone::Clone for AMBISONICS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AMBISONICS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AMBISONICS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDCLNT_SHAREMODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(1i32);
impl ::core::marker::Copy for AUDCLNT_SHAREMODE {}
impl ::core::clone::Clone for AUDCLNT_SHAREMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDCLNT_SHAREMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDCLNT_SHAREMODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDCLNT_SHAREMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_SHAREMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDCLNT_STREAMOPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(4i32);
impl ::core::marker::Copy for AUDCLNT_STREAMOPTIONS {}
impl ::core::clone::Clone for AUDCLNT_STREAMOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDCLNT_STREAMOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDCLNT_STREAMOPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDCLNT_STREAMOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_STREAMOPTIONS").field(&self.0).finish()
    }
}
impl AUDCLNT_STREAMOPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDCLNT_STREAMOPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDCLNT_STREAMOPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIOCLIENT_ACTIVATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(1i32);
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_TYPE {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIOCLIENT_ACTIVATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIOCLIENT_ACTIVATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIOCLIENT_ACTIVATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIO_DUCKING_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(1i32);
impl ::core::marker::Copy for AUDIO_DUCKING_OPTIONS {}
impl ::core::clone::Clone for AUDIO_DUCKING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_DUCKING_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIO_DUCKING_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIO_DUCKING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_DUCKING_OPTIONS").field(&self.0).finish()
    }
}
impl AUDIO_DUCKING_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDIO_DUCKING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDIO_DUCKING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIO_EFFECT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(1i32);
impl ::core::marker::Copy for AUDIO_EFFECT_STATE {}
impl ::core::clone::Clone for AUDIO_EFFECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_EFFECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIO_EFFECT_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIO_EFFECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_EFFECT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIO_STREAM_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(5i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(6i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(7i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(8i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(9i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(10i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(11i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(12i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(13i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(14i32);
impl ::core::marker::Copy for AUDIO_STREAM_CATEGORY {}
impl ::core::clone::Clone for AUDIO_STREAM_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_STREAM_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIO_STREAM_CATEGORY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIO_STREAM_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_STREAM_CATEGORY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(3i32);
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioObjectType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_None: AudioObjectType = AudioObjectType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_Dynamic: AudioObjectType = AudioObjectType(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontLeft: AudioObjectType = AudioObjectType(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontRight: AudioObjectType = AudioObjectType(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontCenter: AudioObjectType = AudioObjectType(8i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_LowFrequency: AudioObjectType = AudioObjectType(16i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_SideLeft: AudioObjectType = AudioObjectType(32i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_SideRight: AudioObjectType = AudioObjectType(64i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackLeft: AudioObjectType = AudioObjectType(128i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackRight: AudioObjectType = AudioObjectType(256i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopFrontLeft: AudioObjectType = AudioObjectType(512i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopFrontRight: AudioObjectType = AudioObjectType(1024i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopBackLeft: AudioObjectType = AudioObjectType(2048i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopBackRight: AudioObjectType = AudioObjectType(4096i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = AudioObjectType(8192i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomFrontRight: AudioObjectType = AudioObjectType(16384i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomBackLeft: AudioObjectType = AudioObjectType(32768i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomBackRight: AudioObjectType = AudioObjectType(65536i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackCenter: AudioObjectType = AudioObjectType(131072i32);
impl ::core::marker::Copy for AudioObjectType {}
impl ::core::clone::Clone for AudioObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioObjectType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioObjectType").field(&self.0).finish()
    }
}
impl AudioObjectType {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AudioObjectType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AudioObjectType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AudioObjectType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AudioObjectType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AudioObjectType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioSessionDisconnectReason(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = AudioSessionDisconnectReason(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = AudioSessionDisconnectReason(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = AudioSessionDisconnectReason(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = AudioSessionDisconnectReason(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = AudioSessionDisconnectReason(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = AudioSessionDisconnectReason(5i32);
impl ::core::marker::Copy for AudioSessionDisconnectReason {}
impl ::core::clone::Clone for AudioSessionDisconnectReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioSessionDisconnectReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioSessionDisconnectReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioSessionDisconnectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionDisconnectReason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioSessionState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateInactive: AudioSessionState = AudioSessionState(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateActive: AudioSessionState = AudioSessionState(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateExpired: AudioSessionState = AudioSessionState(2i32);
impl ::core::marker::Copy for AudioSessionState {}
impl ::core::clone::Clone for AudioSessionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioSessionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioSessionState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioStateMonitorSoundLevel(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Muted: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Low: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Full: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(2i32);
impl ::core::marker::Copy for AudioStateMonitorSoundLevel {}
impl ::core::clone::Clone for AudioStateMonitorSoundLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioStateMonitorSoundLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioStateMonitorSoundLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioStateMonitorSoundLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStateMonitorSoundLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: Self = Self(0i32);
    pub const Physical_Internal: Self = Self(1i32);
    pub const Physical_External: Self = Self(2i32);
    pub const Software_IO: Self = Self(3i32);
    pub const Software_Fixed: Self = Self(4i32);
    pub const Network: Self = Self(5i32);
}
impl ::core::marker::Copy for ConnectorType {}
impl ::core::clone::Clone for ConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ConnectorType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DataFlow(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const In: DataFlow = DataFlow(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Out: DataFlow = DataFlow(1i32);
impl ::core::marker::Copy for DataFlow {}
impl ::core::clone::Clone for DataFlow {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataFlow {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DataFlow {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataFlow").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDataFlow(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eRender: EDataFlow = EDataFlow(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eCapture: EDataFlow = EDataFlow(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eAll: EDataFlow = EDataFlow(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const EDataFlow_enum_count: EDataFlow = EDataFlow(3i32);
impl ::core::marker::Copy for EDataFlow {}
impl ::core::clone::Clone for EDataFlow {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDataFlow {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EDataFlow {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EDataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDataFlow").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ERole(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eConsole: ERole = ERole(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eMultimedia: ERole = ERole(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eCommunications: ERole = ERole(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ERole_enum_count: ERole = ERole(3i32);
impl ::core::marker::Copy for ERole {}
impl ::core::clone::Clone for ERole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERole {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ERole {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ERole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERole").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EndpointFormFactor(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const RemoteNetworkDevice: EndpointFormFactor = EndpointFormFactor(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Speakers: EndpointFormFactor = EndpointFormFactor(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const LineLevel: EndpointFormFactor = EndpointFormFactor(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Headphones: EndpointFormFactor = EndpointFormFactor(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Microphone: EndpointFormFactor = EndpointFormFactor(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Headset: EndpointFormFactor = EndpointFormFactor(5i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Handset: EndpointFormFactor = EndpointFormFactor(6i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const UnknownDigitalPassthrough: EndpointFormFactor = EndpointFormFactor(7i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPDIF: EndpointFormFactor = EndpointFormFactor(8i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DigitalAudioDisplayDevice: EndpointFormFactor = EndpointFormFactor(9i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const UnknownFormFactor: EndpointFormFactor = EndpointFormFactor(10i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const EndpointFormFactor_enum_count: EndpointFormFactor = EndpointFormFactor(11i32);
impl ::core::marker::Copy for EndpointFormFactor {}
impl ::core::clone::Clone for EndpointFormFactor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EndpointFormFactor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EndpointFormFactor {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EndpointFormFactor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointFormFactor").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MIDI_WAVE_OPEN_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(458752u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(65536u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(196608u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(327680u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(8u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(9u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(16u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
impl ::core::marker::Copy for MIDI_WAVE_OPEN_TYPE {}
impl ::core::clone::Clone for MIDI_WAVE_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIDI_WAVE_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MIDI_WAVE_OPEN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MIDI_WAVE_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDI_WAVE_OPEN_TYPE").field(&self.0).finish()
    }
}
impl MIDI_WAVE_OPEN_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(1u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(5u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(2u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(3u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(6u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(8u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(7u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4106u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4105u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4101u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4097u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4098u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4099u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4103u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4100u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4102u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4096u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4104u32);
impl ::core::marker::Copy for MIXERLINE_COMPONENTTYPE {}
impl ::core::clone::Clone for MIXERLINE_COMPONENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIXERLINE_COMPONENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MIXERLINE_COMPONENTTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MIXERLINE_COMPONENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIXERLINE_COMPONENTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_LOOPBACK_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(1i32);
impl ::core::marker::Copy for PROCESS_LOOPBACK_MODE {}
impl ::core::clone::Clone for PROCESS_LOOPBACK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_LOOPBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_LOOPBACK_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_LOOPBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_LOOPBACK_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PartType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Connector: PartType = PartType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Subunit: PartType = PartType(1i32);
impl ::core::marker::Copy for PartType {}
impl ::core::clone::Clone for PartType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PartType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PartType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PartType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PartType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SND_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_APPLICATION: SND_FLAGS = SND_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS: SND_FLAGS = SND_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS_ID: SND_FLAGS = SND_FLAGS(1114112u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_FILENAME: SND_FLAGS = SND_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_RESOURCE: SND_FLAGS = SND_FLAGS(262148u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ASYNC: SND_FLAGS = SND_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NODEFAULT: SND_FLAGS = SND_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_LOOP: SND_FLAGS = SND_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_MEMORY: SND_FLAGS = SND_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NOSTOP: SND_FLAGS = SND_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NOWAIT: SND_FLAGS = SND_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_PURGE: SND_FLAGS = SND_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SENTRY: SND_FLAGS = SND_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SYNC: SND_FLAGS = SND_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SYSTEM: SND_FLAGS = SND_FLAGS(2097152u32);
impl ::core::marker::Copy for SND_FLAGS {}
impl ::core::clone::Clone for SND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SND_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SND_FLAGS").field(&self.0).finish()
    }
}
impl SND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPATIAL_AUDIO_STREAM_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(1i32);
impl ::core::marker::Copy for SPATIAL_AUDIO_STREAM_OPTIONS {}
impl ::core::clone::Clone for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPATIAL_AUDIO_STREAM_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPATIAL_AUDIO_STREAM_OPTIONS").field(&self.0).finish()
    }
}
impl SPATIAL_AUDIO_STREAM_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpatialAudioHrtfDirectivityType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(2i32);
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityType {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfDirectivityType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDirectivityType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpatialAudioHrtfDistanceDecayType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(1i32);
impl ::core::marker::Copy for SpatialAudioHrtfDistanceDecayType {}
impl ::core::clone::Clone for SpatialAudioHrtfDistanceDecayType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecayType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfDistanceDecayType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpatialAudioHrtfDistanceDecayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDistanceDecayType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpatialAudioHrtfEnvironmentType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(4i32);
impl ::core::marker::Copy for SpatialAudioHrtfEnvironmentType {}
impl ::core::clone::Clone for SpatialAudioHrtfEnvironmentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfEnvironmentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfEnvironmentType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpatialAudioHrtfEnvironmentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfEnvironmentType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpatialAudioMetadataCopyMode(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(3i32);
impl ::core::marker::Copy for SpatialAudioMetadataCopyMode {}
impl ::core::clone::Clone for SpatialAudioMetadataCopyMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioMetadataCopyMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpatialAudioMetadataCopyMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpatialAudioMetadataCopyMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataCopyMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpatialAudioMetadataWriterOverflowMode(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(2i32);
impl ::core::marker::Copy for SpatialAudioMetadataWriterOverflowMode {}
impl ::core::clone::Clone for SpatialAudioMetadataWriterOverflowMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioMetadataWriterOverflowMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpatialAudioMetadataWriterOverflowMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpatialAudioMetadataWriterOverflowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataWriterOverflowMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(4i32);
impl ::core::marker::Copy for _AUDCLNT_BUFFERFLAGS {}
impl ::core::clone::Clone for _AUDCLNT_BUFFERFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _AUDCLNT_BUFFERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _AUDCLNT_BUFFERFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _AUDCLNT_BUFFERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AUDCLNT_BUFFERFLAGS").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [u8; 32],
    pub szLongName: [u8; 128],
    pub szCopyright: [u8; 80],
    pub szLicensing: [u8; 128],
    pub szFeatures: [u8; 512],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for ACMDRIVERDETAILSA {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for ACMDRIVERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows::core::TypeKind for ACMDRIVERDETAILSA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [u16; 32],
    pub szLongName: [u16; 128],
    pub szCopyright: [u16; 80],
    pub szLicensing: [u16; 128],
    pub szFeatures: [u16; 512],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for ACMDRIVERDETAILSW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for ACMDRIVERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows::core::TypeKind for ACMDRIVERDETAILSW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVFORMATSUGGEST {
    pub cbStruct: u32,
    pub fdwSuggest: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub cbwfxSrc: u32,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub cbwfxDst: u32,
}
impl ::core::marker::Copy for ACMDRVFORMATSUGGEST {}
impl ::core::clone::Clone for ACMDRVFORMATSUGGEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMDRVFORMATSUGGEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMDRVFORMATSUGGEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVOPENDESCA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: ::windows::core::PCSTR,
    pub pszAliasName: ::windows::core::PCSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for ACMDRVOPENDESCA {}
impl ::core::clone::Clone for ACMDRVOPENDESCA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMDRVOPENDESCA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMDRVOPENDESCA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVOPENDESCW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: ::windows::core::PCWSTR,
    pub pszAliasName: ::windows::core::PCWSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for ACMDRVOPENDESCW {}
impl ::core::clone::Clone for ACMDRVOPENDESCW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMDRVOPENDESCW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMDRVOPENDESCW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub fdwConvert: u32,
    pub padshNext: *mut ACMDRVSTREAMHEADER,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub fdwPrepared: u32,
    pub dwPrepared: usize,
    pub pbPreparedSrc: *mut u8,
    pub cbPreparedSrcLength: u32,
    pub pbPreparedDst: *mut u8,
    pub cbPreparedDstLength: u32,
}
impl ::core::marker::Copy for ACMDRVSTREAMHEADER {}
impl ::core::clone::Clone for ACMDRVSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMDRVSTREAMHEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMDRVSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVSTREAMINSTANCE {
    pub cbStruct: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub pwfltr: *mut WAVEFILTER,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub fdwOpen: u32,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub has: HACMSTREAM,
}
impl ::core::marker::Copy for ACMDRVSTREAMINSTANCE {}
impl ::core::clone::Clone for ACMDRVSTREAMINSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMDRVSTREAMINSTANCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMDRVSTREAMINSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
impl ::core::marker::Copy for ACMDRVSTREAMSIZE {}
impl ::core::clone::Clone for ACMDRVSTREAMSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMDRVSTREAMSIZE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMDRVSTREAMSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: ::windows::core::PCSTR,
    pub szFilterTag: [u8; 48],
    pub szFilter: [u8; 128],
    pub pszName: ::windows::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HMODULE,
    pub pszTemplateName: ::windows::core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACMFILTERCHOOSEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: ::windows::core::PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: ::windows::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HMODULE,
    pub pszTemplateName: ::windows::core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACMFILTERCHOOSEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERDETAILSA {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u8; 128],
}
impl ::core::marker::Copy for ACMFILTERDETAILSA {}
impl ::core::clone::Clone for ACMFILTERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMFILTERDETAILSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
impl ::core::marker::Copy for ACMFILTERDETAILSW {}
impl ::core::clone::Clone for ACMFILTERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMFILTERDETAILSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u8; 48],
}
impl ::core::marker::Copy for ACMFILTERTAGDETAILSA {}
impl ::core::clone::Clone for ACMFILTERTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMFILTERTAGDETAILSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFILTERTAGDETAILSW {}
impl ::core::clone::Clone for ACMFILTERTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMFILTERTAGDETAILSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: ::windows::core::PCSTR,
    pub szFormatTag: [u8; 48],
    pub szFormat: [u8; 128],
    pub pszName: ::windows::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HMODULE,
    pub pszTemplateName: ::windows::core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACMFORMATCHOOSEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: ::windows::core::PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: ::windows::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HMODULE,
    pub pszTemplateName: ::windows::core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACMFORMATCHOOSEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFORMATDETAILSA {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u8; 128],
}
impl ::core::marker::Copy for ACMFORMATDETAILSA {}
impl ::core::clone::Clone for ACMFORMATDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMFORMATDETAILSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u8; 48],
}
impl ::core::marker::Copy for ACMFORMATTAGDETAILSA {}
impl ::core::clone::Clone for ACMFORMATTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMFORMATTAGDETAILSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFORMATTAGDETAILSW {}
impl ::core::clone::Clone for ACMFORMATTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ACMFORMATTAGDETAILSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 15],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ACMSTREAMHEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for ACMSTREAMHEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[cfg(target_arch = "x86")]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 10],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for ACMSTREAMHEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for ACMSTREAMHEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AMBISONICS_PARAMS {
    pub u32Size: u32,
    pub u32Version: u32,
    pub u32Type: AMBISONICS_TYPE,
    pub u32ChannelOrdering: AMBISONICS_CHANNEL_ORDERING,
    pub u32Normalization: AMBISONICS_NORMALIZATION,
    pub u32Order: u32,
    pub u32NumChannels: u32,
    pub pu32ChannelMap: *mut u32,
}
impl ::core::marker::Copy for AMBISONICS_PARAMS {}
impl ::core::clone::Clone for AMBISONICS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AMBISONICS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AMBISONICS_PARAMS").field("u32Size", &self.u32Size).field("u32Version", &self.u32Version).field("u32Type", &self.u32Type).field("u32ChannelOrdering", &self.u32ChannelOrdering).field("u32Normalization", &self.u32Normalization).field("u32Order", &self.u32Order).field("u32NumChannels", &self.u32NumChannels).field("pu32ChannelMap", &self.pu32ChannelMap).finish()
    }
}
impl ::windows::core::TypeKind for AMBISONICS_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AMBISONICS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.u32Size == other.u32Size && self.u32Version == other.u32Version && self.u32Type == other.u32Type && self.u32ChannelOrdering == other.u32ChannelOrdering && self.u32Normalization == other.u32Normalization && self.u32Order == other.u32Order && self.u32NumChannels == other.u32NumChannels && self.pu32ChannelMap == other.pu32ChannelMap
    }
}
impl ::core::cmp::Eq for AMBISONICS_PARAMS {}
impl ::core::default::Default for AMBISONICS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUDIOCLIENT_ACTIVATION_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    pub ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
impl ::core::marker::Copy for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS").field("TargetProcessId", &self.TargetProcessId).field("ProcessLoopbackMode", &self.ProcessLoopbackMode).finish()
    }
}
impl ::windows::core::TypeKind for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.TargetProcessId == other.TargetProcessId && self.ProcessLoopbackMode == other.ProcessLoopbackMode
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::default::Default for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_EFFECT {
    pub id: ::windows::core::GUID,
    pub canSetState: super::super::Foundation::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_EFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUDIO_EFFECT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_EFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.canSetState == other.canSetState && self.state == other.state
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_EFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: ::windows::core::GUID,
    pub bMuted: super::super::Foundation::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_VOLUME_NOTIFICATION_DATA").field("guidEventContext", &self.guidEventContext).field("bMuted", &self.bMuted).field("fMasterVolume", &self.fMasterVolume).field("nChannels", &self.nChannels).field("afChannelVolumes", &self.afChannelVolumes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUDIO_VOLUME_NOTIFICATION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.guidEventContext == other.guidEventContext && self.bMuted == other.bMuted && self.fMasterVolume == other.fMasterVolume && self.nChannels == other.nChannels && self.afChannelVolumes == other.afChannelVolumes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUXCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for AUXCAPS2A {}
impl ::core::clone::Clone for AUXCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUXCAPS2A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUXCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for AUXCAPS2W {}
impl ::core::clone::Clone for AUXCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUXCAPS2W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUXCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for AUXCAPSA {}
impl ::core::clone::Clone for AUXCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUXCAPSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUXCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for AUXCAPSW {}
impl ::core::clone::Clone for AUXCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUXCAPSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: ::windows::core::GUID,
}
impl ::core::marker::Copy for AudioClient3ActivationParams {}
impl ::core::clone::Clone for AudioClient3ActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AudioClient3ActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClient3ActivationParams").field("tracingContextId", &self.tracingContextId).finish()
    }
}
impl ::windows::core::TypeKind for AudioClient3ActivationParams {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AudioClient3ActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId
    }
}
impl ::core::cmp::Eq for AudioClient3ActivationParams {}
impl ::core::default::Default for AudioClient3ActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: super::super::Foundation::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AudioClientProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioClientProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClientProperties").field("cbSize", &self.cbSize).field("bIsOffload", &self.bIsOffload).field("eCategory", &self.eCategory).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AudioClientProperties {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AudioClientProperties {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.bIsOffload == other.bIsOffload && self.eCategory == other.eCategory && self.Options == other.Options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AudioClientProperties {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: ::std::mem::ManuallyDrop<::core::option::Option<IMMDevice>>,
    pub pPnpInterface: ::std::mem::ManuallyDrop<::core::option::Option<IMMDevice>>,
    pub pPnpDevnode: ::std::mem::ManuallyDrop<::core::option::Option<IMMDevice>>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AudioExtensionParams {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioExtensionParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioExtensionParams").field("AddPageParam", &self.AddPageParam).field("pEndpoint", &self.pEndpoint).field("pPnpInterface", &self.pPnpInterface).field("pPnpDevnode", &self.pPnpDevnode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AudioExtensionParams {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AudioExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam && self.pEndpoint == other.pEndpoint && self.pPnpInterface == other.pPnpInterface && self.pPnpDevnode == other.pPnpDevnode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AudioExtensionParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AudioExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: ::windows::core::GUID,
    pub dwAudioStreamFlags: u32,
}
impl ::core::marker::Copy for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIRECTX_AUDIO_ACTIVATION_PARAMS").field("cbDirectXAudioActivationParams", &self.cbDirectXAudioActivationParams).field("guidAudioSession", &self.guidAudioSession).field("dwAudioStreamFlags", &self.dwAudioStreamFlags).finish()
    }
}
impl ::windows::core::TypeKind for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbDirectXAudioActivationParams == other.cbDirectXAudioActivationParams && self.guidAudioSession == other.guidAudioSession && self.dwAudioStreamFlags == other.dwAudioStreamFlags
    }
}
impl ::core::cmp::Eq for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::default::Default for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
impl ::core::marker::Copy for ECHOWAVEFILTER {}
impl ::core::clone::Clone for ECHOWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ECHOWAVEFILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HACMDRIVER(pub isize);
impl HACMDRIVER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMDRIVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMDRIVER {}
impl ::core::fmt::Debug for HACMDRIVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMDRIVER").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HACMDRIVER {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HACMDRIVERID(pub isize);
impl HACMDRIVERID {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HACMDRIVERID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMDRIVERID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMDRIVERID {}
impl ::core::fmt::Debug for HACMDRIVERID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMDRIVERID").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HACMDRIVERID {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HACMOBJ(pub isize);
impl HACMOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HACMOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMOBJ {}
impl ::core::fmt::Debug for HACMOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMOBJ").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HACMOBJ {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HACMSTREAM(pub isize);
impl HACMSTREAM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMSTREAM {}
impl ::core::fmt::Debug for HACMSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMSTREAM").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HACMSTREAM {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMIDI(pub isize);
impl HMIDI {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMIDI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDI {}
impl ::core::fmt::Debug for HMIDI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDI").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HMIDI {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMIDIIN(pub isize);
impl HMIDIIN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMIDIIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDIIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDIIN {}
impl ::core::fmt::Debug for HMIDIIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDIIN").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HMIDIIN {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMIDIOUT(pub isize);
impl HMIDIOUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDIOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDIOUT {}
impl ::core::fmt::Debug for HMIDIOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDIOUT").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HMIDIOUT {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMIDISTRM(pub isize);
impl HMIDISTRM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDISTRM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDISTRM {}
impl ::core::fmt::Debug for HMIDISTRM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDISTRM").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HMIDISTRM {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMIXER(pub isize);
impl HMIXER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMIXER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIXER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIXER {}
impl ::core::fmt::Debug for HMIXER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIXER").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HMIXER {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMIXEROBJ(pub isize);
impl HMIXEROBJ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMIXEROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIXEROBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIXEROBJ {}
impl ::core::fmt::Debug for HMIXEROBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIXEROBJ").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HMIXEROBJ {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HWAVE(pub isize);
impl HWAVE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HWAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVE {}
impl ::core::fmt::Debug for HWAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HWAVE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HWAVEIN(pub isize);
impl HWAVEIN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HWAVEIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVEIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVEIN {}
impl ::core::fmt::Debug for HWAVEIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVEIN").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HWAVEIN {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HWAVEOUT(pub isize);
impl HWAVEOUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVEOUT {}
impl ::core::fmt::Debug for HWAVEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVEOUT").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HWAVEOUT {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl ::core::marker::Copy for MIDIEVENT {}
impl ::core::clone::Clone for MIDIEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIEVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIHDR {
    pub lpData: ::windows::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub lpNext: *mut MIDIHDR,
    pub reserved: usize,
    pub dwOffset: u32,
    pub dwReserved: [usize; 8],
}
impl ::core::marker::Copy for MIDIHDR {}
impl ::core::clone::Clone for MIDIHDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIHDR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for MIDIINCAPS2A {}
impl ::core::clone::Clone for MIDIINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIINCAPS2A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for MIDIINCAPS2W {}
impl ::core::clone::Clone for MIDIINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIINCAPS2W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIINCAPSA {}
impl ::core::clone::Clone for MIDIINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIINCAPSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIINCAPSW {}
impl ::core::clone::Clone for MIDIINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIINCAPSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for MIDIOUTCAPS2A {}
impl ::core::clone::Clone for MIDIOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIOUTCAPS2A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for MIDIOUTCAPS2W {}
impl ::core::clone::Clone for MIDIOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIOUTCAPS2W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIOUTCAPSA {}
impl ::core::clone::Clone for MIDIOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIOUTCAPSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIOUTCAPSW {}
impl ::core::clone::Clone for MIDIOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIOUTCAPSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
impl ::core::marker::Copy for MIDIPROPTEMPO {}
impl ::core::clone::Clone for MIDIPROPTEMPO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIPROPTEMPO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIPROPTEMPO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
impl ::core::marker::Copy for MIDIPROPTIMEDIV {}
impl ::core::clone::Clone for MIDIPROPTIMEDIV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDIPROPTIMEDIV {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDIPROPTIMEDIV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
impl ::core::marker::Copy for MIDISTRMBUFFVER {}
impl ::core::clone::Clone for MIDISTRMBUFFVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIDISTRMBUFFVER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIDISTRMBUFFVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for MIXERCAPS2A {}
impl ::core::clone::Clone for MIXERCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCAPS2A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for MIXERCAPS2W {}
impl ::core::clone::Clone for MIXERCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCAPS2W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl ::core::marker::Copy for MIXERCAPSA {}
impl ::core::clone::Clone for MIXERCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCAPSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl ::core::marker::Copy for MIXERCAPSW {}
impl ::core::clone::Clone for MIXERCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCAPSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLA {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [u8; 16],
    pub szName: [u8; 64],
    pub Bounds: MIXERCONTROLA_0,
    pub Metrics: MIXERCONTROLA_1,
}
impl ::core::marker::Copy for MIXERCONTROLA {}
impl ::core::clone::Clone for MIXERCONTROLA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLA_0 {
    pub Anonymous1: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLA_0 {}
impl ::core::clone::Clone for MIXERCONTROLA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl ::core::marker::Copy for MIXERCONTROLA_0_0 {}
impl ::core::clone::Clone for MIXERCONTROLA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLA_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl ::core::marker::Copy for MIXERCONTROLA_0_1 {}
impl ::core::clone::Clone for MIXERCONTROLA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLA_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLA_1 {}
impl ::core::clone::Clone for MIXERCONTROLA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLA_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub cChannels: u32,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: u32,
    pub paDetails: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MIXERCONTROLDETAILS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::super::Foundation::HWND,
    pub cMultipleItems: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MIXERCONTROLDETAILS_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_BOOLEAN {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_BOOLEAN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLDETAILS_BOOLEAN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u8; 64],
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTA {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLDETAILS_LISTTEXTA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTW {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLDETAILS_LISTTEXTW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_SIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_SIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLDETAILS_SIGNED {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_UNSIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_UNSIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLDETAILS_UNSIGNED {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Bounds: MIXERCONTROLW_0,
    pub Metrics: MIXERCONTROLW_1,
}
impl ::core::marker::Copy for MIXERCONTROLW {}
impl ::core::clone::Clone for MIXERCONTROLW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLW_0 {
    pub Anonymous1: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLW_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLW_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLW_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLW_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLW_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLW_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLW_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERCONTROLW_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINEA {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [u8; 16],
    pub szName: [u8; 64],
    pub Target: MIXERLINEA_0,
}
impl ::core::marker::Copy for MIXERLINEA {}
impl ::core::clone::Clone for MIXERLINEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINEA_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
}
impl ::core::marker::Copy for MIXERLINEA_0 {}
impl ::core::clone::Clone for MIXERLINEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINEA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINEA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINECONTROLSA {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSA_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLA,
}
impl ::core::marker::Copy for MIXERLINECONTROLSA {}
impl ::core::clone::Clone for MIXERLINECONTROLSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINECONTROLSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl ::core::marker::Copy for MIXERLINECONTROLSA_0 {}
impl ::core::clone::Clone for MIXERLINECONTROLSA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINECONTROLSA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINECONTROLSW {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLW,
}
impl ::core::marker::Copy for MIXERLINECONTROLSW {}
impl ::core::clone::Clone for MIXERLINECONTROLSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINECONTROLSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl ::core::marker::Copy for MIXERLINECONTROLSW_0 {}
impl ::core::clone::Clone for MIXERLINECONTROLSW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINECONTROLSW_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINEW {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Target: MIXERLINEW_0,
}
impl ::core::marker::Copy for MIXERLINEW {}
impl ::core::clone::Clone for MIXERLINEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINEW_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
}
impl ::core::marker::Copy for MIXERLINEW_0 {}
impl ::core::clone::Clone for MIXERLINEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MIXERLINEW_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
impl ::core::marker::Copy for PCMWAVEFORMAT {}
impl ::core::clone::Clone for PCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PCMWAVEFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: ::windows::core::GUID,
    pub appId: ::windows::core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
impl ::core::marker::Copy for SpatialAudioClientActivationParams {}
impl ::core::clone::Clone for SpatialAudioClientActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioClientActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioClientActivationParams").field("tracingContextId", &self.tracingContextId).field("appId", &self.appId).field("majorVersion", &self.majorVersion).field("minorVersion1", &self.minorVersion1).field("minorVersion2", &self.minorVersion2).field("minorVersion3", &self.minorVersion3).finish()
    }
}
impl ::windows::core::TypeKind for SpatialAudioClientActivationParams {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SpatialAudioClientActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId && self.appId == other.appId && self.majorVersion == other.majorVersion && self.minorVersion1 == other.minorVersion1 && self.minorVersion2 == other.minorVersion2 && self.minorVersion3 == other.minorVersion3
    }
}
impl ::core::cmp::Eq for SpatialAudioClientActivationParams {}
impl ::core::default::Default for SpatialAudioClientActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::std::mem::ManuallyDrop<::core::option::Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SpatialAudioHrtfActivationParams {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::std::mem::ManuallyDrop<::core::option::Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SpatialAudioHrtfActivationParams2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivity {
    pub Type: SpatialAudioHrtfDirectivityType,
    pub Scaling: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivity {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfDirectivity {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SpatialAudioHrtfDirectivity {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivityCardioid {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub Order: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCardioid {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCardioid {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfDirectivityCardioid {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivityCone {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub InnerAngle: f32,
    pub OuterAngle: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCone {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCone {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfDirectivityCone {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCone {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union SpatialAudioHrtfDirectivityUnion {
    pub Cone: SpatialAudioHrtfDirectivityCone,
    pub Cardiod: SpatialAudioHrtfDirectivityCardioid,
    pub Omni: SpatialAudioHrtfDirectivity,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityUnion {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityUnion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfDirectivityUnion {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityUnion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDistanceDecay {}
impl ::core::clone::Clone for SpatialAudioHrtfDistanceDecay {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SpatialAudioHrtfDistanceDecay {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
impl ::core::marker::Copy for SpatialAudioMetadataItemsInfo {}
impl ::core::clone::Clone for SpatialAudioMetadataItemsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SpatialAudioMetadataItemsInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SpatialAudioMetadataItemsInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::std::mem::ManuallyDrop<::core::option::Option<ISpatialAudioObjectRenderStreamNotify>>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SpatialAudioObjectRenderStreamActivationParams {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::std::mem::ManuallyDrop<::core::option::Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SpatialAudioObjectRenderStreamActivationParams2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ::std::mem::ManuallyDrop<::core::option::Option<ISpatialAudioObjectRenderStreamNotify>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ::std::mem::ManuallyDrop<::core::option::Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for VOLUMEWAVEFILTER {}
impl ::core::clone::Clone for VOLUMEWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VOLUMEWAVEFILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl ::core::marker::Copy for WAVEFILTER {}
impl ::core::clone::Clone for WAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEFILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
impl ::core::marker::Copy for WAVEFORMAT {}
impl ::core::clone::Clone for WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for WAVEFORMATEX {}
impl ::core::clone::Clone for WAVEFORMATEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEFORMATEX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEFORMATEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: ::windows::core::GUID,
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEFORMATEXTENSIBLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE_0 {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEFORMATEXTENSIBLE_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEHDR {
    pub lpData: ::windows::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwLoops: u32,
    pub lpNext: *mut WAVEHDR,
    pub reserved: usize,
}
impl ::core::marker::Copy for WAVEHDR {}
impl ::core::clone::Clone for WAVEHDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEHDR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for WAVEINCAPS2A {}
impl ::core::clone::Clone for WAVEINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEINCAPS2A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for WAVEINCAPS2W {}
impl ::core::clone::Clone for WAVEINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEINCAPS2W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for WAVEINCAPSA {}
impl ::core::clone::Clone for WAVEINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEINCAPSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for WAVEINCAPSW {}
impl ::core::clone::Clone for WAVEINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEINCAPSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for WAVEOUTCAPS2A {}
impl ::core::clone::Clone for WAVEOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEOUTCAPS2A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for WAVEOUTCAPS2W {}
impl ::core::clone::Clone for WAVEOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEOUTCAPS2W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for WAVEOUTCAPSA {}
impl ::core::clone::Clone for WAVEOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEOUTCAPSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for WAVEOUTCAPSW {}
impl ::core::clone::Clone for WAVEOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WAVEOUTCAPSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct tACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
impl ::core::marker::Copy for tACMFORMATDETAILSW {}
impl ::core::clone::Clone for tACMFORMATDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for tACMFORMATDETAILSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for tACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMDRIVERENUMCB = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPACMDRIVERPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = ::core::option::Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = ::core::option::Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type PAudioStateMonitorCallback = ::core::option::Option<unsafe extern "system" fn(audiostatemonitor: ::core::option::Option<IAudioStateMonitor>, context: *const ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
