use super::*;

/// Initializes the Windows App Runtime for framework-dependent deployment.
///
/// Call once at the start of the process after using
/// `windows_reactor_setup::as_framework_dependent()` or
/// `windows_reactor_setup::as_example()` in `build.rs`. Self-contained
/// applications do not call this function.
pub fn bootstrap() -> windows_core::Result<()> {
    unsafe {
        bindings::MddBootstrapInitialize2(
            bindings::WINDOWSAPPSDK_RELEASE_MAJORMINOR as u32,
            bindings::WINDOWSAPPSDK_RELEASE_VERSION_TAG_W.as_ptr(),
            bindings::PACKAGE_VERSION {
                Anonymous: bindings::PACKAGE_VERSION_0 {
                    Version: bindings::WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
                },
            },
            bindings::MddBootstrapInitializeOptions_OnNoMatch_ShowUI
                | bindings::MddBootstrapInitializeOptions_OnPackageIdentity_NOOP,
        )
        .ok()
    }
}
