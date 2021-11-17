use windows::{Win32::System::WinRT::Composition::*, Win32::System::WinRT::*};

#[test]
fn test() {
    // Simple check to ensure that code gen picked up that ISystemMediaTransportControlsInterop derives from IInspectable.
    // 3 for IUnknown, 3 for IInspectable, and 1 for ISystemMediaTransportControlsInterop.
    assert_eq!(core::mem::size_of::<ISystemMediaTransportControlsInterop_abi>(), 7 * core::mem::size_of::<usize>());

    // And ICompositorInterop derives directly from IUnknown.
    // 3 for IUnknown and 3 for ICompositorInterop.
    assert_eq!(core::mem::size_of::<ICompositorInterop_abi>(), 6 * core::mem::size_of::<usize>());
}
