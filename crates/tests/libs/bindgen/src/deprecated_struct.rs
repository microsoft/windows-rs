#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IPlayToSourceSelectedEventArgs,
    IPlayToSourceSelectedEventArgs_Vtbl,
    0x0c9d8511_5202_4dcb_8c67_abda12bb3c12
);
impl windows_core::RuntimeType for IPlayToSourceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceSelectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Icon: usize,
    pub SupportsImage:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportsAudio:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportsVideo:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToSourceSelectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PlayToSourceSelectedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PlayToSourceSelectedEventArgs {
    #[deprecated(
        note = "PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10."
    )]
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    #[deprecated(
        note = "PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10."
    )]
    pub fn SupportsImage(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsImage)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    #[deprecated(
        note = "PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10."
    )]
    pub fn SupportsAudio(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsAudio)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    #[deprecated(
        note = "PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10."
    )]
    pub fn SupportsVideo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsVideo)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayToSourceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPlayToSourceSelectedEventArgs>();
}
unsafe impl windows_core::Interface for PlayToSourceSelectedEventArgs {
    type Vtable = <IPlayToSourceSelectedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IPlayToSourceSelectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToSourceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
}
unsafe impl Send for PlayToSourceSelectedEventArgs {}
unsafe impl Sync for PlayToSourceSelectedEventArgs {}
