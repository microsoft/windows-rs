fn main() {
    windows::build!(
        windows::win32::dxgi::DXGI_ADAPTER_DESC1,
        windows::win32::windows_and_messaging::NCCALCSIZE_PARAMS,
        windows::win32::ip_helper::IPV6_ADDRESS_EX,
    );
}
