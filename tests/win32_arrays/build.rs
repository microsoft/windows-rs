fn main() {
    windows::build!(
        Windows::Win32::Dxgi::DXGI_ADAPTER_DESC1,
        Windows::Win32::WindowsAndMessaging::NCCALCSIZE_PARAMS,
        Windows::Win32::IpHelper::IPV6_ADDRESS_EX,
    );
}
