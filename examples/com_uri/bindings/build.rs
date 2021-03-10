fn main() {
    windows::build!(
        windows::win32::automation::BSTR,
        windows::win32::com::CreateUri,
    );
}
