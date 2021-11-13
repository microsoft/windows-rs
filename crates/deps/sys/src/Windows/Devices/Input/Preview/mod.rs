#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Configuring: Self = Self(2i32);
    pub const ScreenSetupNeeded: Self = Self(3i32);
    pub const UserCalibrationNeeded: Self = Self(4i32);
}
impl ::core::marker::Copy for GazeDeviceConfigurationStatePreview {}
impl ::core::clone::Clone for GazeDeviceConfigurationStatePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeDevicePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeDevicePreview {}
impl ::core::clone::Clone for GazeDevicePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeDeviceWatcherAddedPreviewEventArgs {}
impl ::core::clone::Clone for GazeDeviceWatcherAddedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeDeviceWatcherPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeDeviceWatcherPreview {}
impl ::core::clone::Clone for GazeDeviceWatcherPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeDeviceWatcherRemovedPreviewEventArgs {}
impl ::core::clone::Clone for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeDeviceWatcherUpdatedPreviewEventArgs {}
impl ::core::clone::Clone for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeEnteredPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeEnteredPreviewEventArgs {}
impl ::core::clone::Clone for GazeEnteredPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeExitedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeExitedPreviewEventArgs {}
impl ::core::clone::Clone for GazeExitedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeInputSourcePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeInputSourcePreview {}
impl ::core::clone::Clone for GazeInputSourcePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeMovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazeMovedPreviewEventArgs {}
impl ::core::clone::Clone for GazeMovedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazePointPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GazePointPreview {}
impl ::core::clone::Clone for GazePointPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeDevicePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeDevicePreview {}
impl ::core::clone::Clone for IGazeDevicePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeDeviceWatcherAddedPreviewEventArgs {}
impl ::core::clone::Clone for IGazeDeviceWatcherAddedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeDeviceWatcherPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeDeviceWatcherPreview {}
impl ::core::clone::Clone for IGazeDeviceWatcherPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeDeviceWatcherRemovedPreviewEventArgs {}
impl ::core::clone::Clone for IGazeDeviceWatcherRemovedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeDeviceWatcherUpdatedPreviewEventArgs {}
impl ::core::clone::Clone for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeEnteredPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeEnteredPreviewEventArgs {}
impl ::core::clone::Clone for IGazeEnteredPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeExitedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeExitedPreviewEventArgs {}
impl ::core::clone::Clone for IGazeExitedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeInputSourcePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeInputSourcePreview {}
impl ::core::clone::Clone for IGazeInputSourcePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeInputSourcePreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeInputSourcePreviewStatics {}
impl ::core::clone::Clone for IGazeInputSourcePreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazeMovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazeMovedPreviewEventArgs {}
impl ::core::clone::Clone for IGazeMovedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGazePointPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGazePointPreview {}
impl ::core::clone::Clone for IGazePointPreview {
    fn clone(&self) -> Self {
        *self
    }
}
