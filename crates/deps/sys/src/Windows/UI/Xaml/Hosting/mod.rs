#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DesignerAppExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DesignerAppManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DesignerAppView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DesignerAppViewState(pub i32);
impl DesignerAppViewState {
    pub const Visible: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
#[repr(transparent)]
pub struct DesktopWindowXamlSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementCompositionPreview(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HostingContract(i32);
#[repr(transparent)]
pub struct IDesignerAppExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesignerAppManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesignerAppManagerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesignerAppView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopWindowXamlSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementCompositionPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsXamlManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsXamlManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResultFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUIPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUIPresenterHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUIPresenterHost2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUIPresenterHost3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUIPresenterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUIPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowsXamlManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlSourceFocusNavigationReason(pub i32);
impl XamlSourceFocusNavigationReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Restore: Self = Self(1i32);
    pub const First: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
    pub const Left: Self = Self(7i32);
    pub const Up: Self = Self(8i32);
    pub const Right: Self = Self(9i32);
    pub const Down: Self = Self(10i32);
}
#[repr(transparent)]
pub struct XamlSourceFocusNavigationRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlSourceFocusNavigationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlUIPresenter(pub *mut ::core::ffi::c_void);
