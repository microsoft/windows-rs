use super::bindings::*;

/// Initializes the Windows App Runtime bootstrap for apps that rely on
/// framework-dependent deployment.
///
/// Call once at the top of `main` and keep the returned handle alive for the
/// duration of the process. Self-contained apps do not need to call this.
pub fn bootstrap() -> windows_core::Result<()> {
    unsafe {
        MddBootstrapInitialize2(
            WINDOWSAPPSDK_RELEASE_MAJORMINOR as u32,
            WINDOWSAPPSDK_RELEASE_VERSION_TAG_W.as_ptr(),
            PACKAGE_VERSION {
                Anonymous: PACKAGE_VERSION_0 {
                    Version: WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
                },
            },
            MddBootstrapInitializeOptions_OnNoMatch_ShowUI
                | MddBootstrapInitializeOptions_OnPackageIdentity_NOOP,
        )
        .ok()
    }
}
