pub trait IWindowsDevicesAllJoynBusAttachmentFactoryInteropImpl: Sized {
    fn CreateFromWin32Handle();
}
pub trait IWindowsDevicesAllJoynBusAttachmentInteropImpl: Sized {
    fn Win32Handle();
}
pub trait IWindowsDevicesAllJoynBusObjectFactoryInteropImpl: Sized {
    fn CreateFromWin32Handle();
}
pub trait IWindowsDevicesAllJoynBusObjectInteropImpl: Sized {
    fn AddPropertyGetHandler();
    fn AddPropertySetHandler();
    fn Win32Handle();
}
