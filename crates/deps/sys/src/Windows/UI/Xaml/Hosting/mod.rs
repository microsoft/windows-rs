#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DesignerAppExitedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DesignerAppExitedEventArgs {}
impl ::core::clone::Clone for DesignerAppExitedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DesignerAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DesignerAppManager {}
impl ::core::clone::Clone for DesignerAppManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DesignerAppView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DesignerAppView {}
impl ::core::clone::Clone for DesignerAppView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DesignerAppViewState(pub i32);
impl DesignerAppViewState {
    pub const Visible: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for DesignerAppViewState {}
impl ::core::clone::Clone for DesignerAppViewState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DesktopWindowXamlSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DesktopWindowXamlSource {}
impl ::core::clone::Clone for DesktopWindowXamlSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DesktopWindowXamlSourceGotFocusEventArgs {}
impl ::core::clone::Clone for DesktopWindowXamlSourceGotFocusEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
impl ::core::clone::Clone for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementCompositionPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ElementCompositionPreview {}
impl ::core::clone::Clone for ElementCompositionPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesignerAppExitedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesignerAppExitedEventArgs {}
impl ::core::clone::Clone for IDesignerAppExitedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesignerAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesignerAppManager {}
impl ::core::clone::Clone for IDesignerAppManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesignerAppManagerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesignerAppManagerFactory {}
impl ::core::clone::Clone for IDesignerAppManagerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesignerAppView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesignerAppView {}
impl ::core::clone::Clone for IDesignerAppView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesktopWindowXamlSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowXamlSource {}
impl ::core::clone::Clone for IDesktopWindowXamlSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowXamlSourceFactory {}
impl ::core::clone::Clone for IDesktopWindowXamlSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowXamlSourceGotFocusEventArgs {}
impl ::core::clone::Clone for IDesktopWindowXamlSourceGotFocusEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
impl ::core::clone::Clone for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementCompositionPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementCompositionPreview {}
impl ::core::clone::Clone for IElementCompositionPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementCompositionPreviewStatics {}
impl ::core::clone::Clone for IElementCompositionPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementCompositionPreviewStatics2 {}
impl ::core::clone::Clone for IElementCompositionPreviewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementCompositionPreviewStatics3 {}
impl ::core::clone::Clone for IElementCompositionPreviewStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowsXamlManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowsXamlManager {}
impl ::core::clone::Clone for IWindowsXamlManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowsXamlManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowsXamlManagerStatics {}
impl ::core::clone::Clone for IWindowsXamlManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlSourceFocusNavigationRequest {}
impl ::core::clone::Clone for IXamlSourceFocusNavigationRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationRequestFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlSourceFocusNavigationRequestFactory {}
impl ::core::clone::Clone for IXamlSourceFocusNavigationRequestFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlSourceFocusNavigationResult {}
impl ::core::clone::Clone for IXamlSourceFocusNavigationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlSourceFocusNavigationResultFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlSourceFocusNavigationResultFactory {}
impl ::core::clone::Clone for IXamlSourceFocusNavigationResultFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlUIPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlUIPresenter {}
impl ::core::clone::Clone for IXamlUIPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlUIPresenterHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlUIPresenterHost {}
impl ::core::clone::Clone for IXamlUIPresenterHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlUIPresenterHost2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlUIPresenterHost2 {}
impl ::core::clone::Clone for IXamlUIPresenterHost2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlUIPresenterHost3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlUIPresenterHost3 {}
impl ::core::clone::Clone for IXamlUIPresenterHost3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlUIPresenterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlUIPresenterStatics {}
impl ::core::clone::Clone for IXamlUIPresenterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlUIPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlUIPresenterStatics2 {}
impl ::core::clone::Clone for IXamlUIPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowsXamlManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowsXamlManager {}
impl ::core::clone::Clone for WindowsXamlManager {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for XamlSourceFocusNavigationReason {}
impl ::core::clone::Clone for XamlSourceFocusNavigationReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlSourceFocusNavigationRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlSourceFocusNavigationRequest {}
impl ::core::clone::Clone for XamlSourceFocusNavigationRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlSourceFocusNavigationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlSourceFocusNavigationResult {}
impl ::core::clone::Clone for XamlSourceFocusNavigationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlUIPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlUIPresenter {}
impl ::core::clone::Clone for XamlUIPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
