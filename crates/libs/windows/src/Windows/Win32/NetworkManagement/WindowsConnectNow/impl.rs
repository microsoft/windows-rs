pub trait IWCNConnectNotifyImpl: Sized {
    fn ConnectSucceeded();
    fn ConnectFailed();
}
pub trait IWCNDeviceImpl: Sized {
    fn SetPassword();
    fn Connect();
    fn GetAttribute();
    fn GetIntegerAttribute();
    fn GetStringAttribute();
    fn GetNetworkProfile();
    fn SetNetworkProfile();
    fn GetVendorExtension();
    fn SetVendorExtension();
    fn Unadvise();
    fn SetNFCPasswordParams();
}
