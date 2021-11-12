#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DisplayAdapter(pub *mut ::core::ffi::c_void);
pub struct DisplayBitsPerChannel(i32);
#[repr(transparent)]
pub struct DisplayDevice(pub *mut ::core::ffi::c_void);
pub struct DisplayDeviceCapability(i32);
#[repr(transparent)]
pub struct DisplayFence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerDisabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerEnabledEventArgs(pub *mut ::core::ffi::c_void);
pub struct DisplayManagerOptions(i32);
#[repr(transparent)]
pub struct DisplayManagerPathsFailedOrInvalidatedEventArgs(pub *mut ::core::ffi::c_void);
pub struct DisplayManagerResult(i32);
#[repr(transparent)]
pub struct DisplayManagerResultWithState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayModeInfo(pub *mut ::core::ffi::c_void);
pub struct DisplayModeQueryOptions(i32);
#[repr(transparent)]
pub struct DisplayPath(pub *mut ::core::ffi::c_void);
pub struct DisplayPathScaling(i32);
pub struct DisplayPathStatus(i32);
pub struct DisplayPresentStatus(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct DisplayPresentationRate(i32);
#[repr(transparent)]
pub struct DisplayPrimaryDescription(pub *mut ::core::ffi::c_void);
pub struct DisplayRotation(i32);
#[repr(transparent)]
pub struct DisplayScanout(pub *mut ::core::ffi::c_void);
pub struct DisplayScanoutOptions(i32);
#[repr(transparent)]
pub struct DisplaySource(pub *mut ::core::ffi::c_void);
pub struct DisplaySourceStatus(i32);
#[repr(transparent)]
pub struct DisplayState(pub *mut ::core::ffi::c_void);
pub struct DisplayStateApplyOptions(i32);
pub struct DisplayStateFunctionalizeOptions(i32);
#[repr(transparent)]
pub struct DisplayStateOperationResult(pub *mut ::core::ffi::c_void);
pub struct DisplayStateOperationStatus(i32);
#[repr(transparent)]
pub struct DisplaySurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTarget(pub *mut ::core::ffi::c_void);
pub struct DisplayTargetPersistence(i32);
#[repr(transparent)]
pub struct DisplayTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTaskPool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTaskResult(pub *mut ::core::ffi::c_void);
pub struct DisplayTaskSignalKind(i32);
#[repr(transparent)]
pub struct DisplayView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayWireFormat(pub *mut ::core::ffi::c_void);
pub struct DisplayWireFormatColorSpace(i32);
pub struct DisplayWireFormatEotf(i32);
pub struct DisplayWireFormatHdrMetadata(i32);
pub struct DisplayWireFormatPixelEncoding(i32);
#[repr(transparent)]
pub struct IDisplayAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayAdapterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayFence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerDisabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerEnabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerResultWithState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayModeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayModeInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPath2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPrimaryDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayScanout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplaySource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplaySource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayStateOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplaySurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTask2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTaskPool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTaskPool2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTaskResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayWireFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayWireFormatFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayWireFormatStatics(pub *mut ::core::ffi::c_void);
