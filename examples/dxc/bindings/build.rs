fn main() {
    windows::build!(
        Windows::Win32::Globalization::CP_UTF8,
        Windows::Win32::Graphics::Hlsl::*,
        Windows::Win32::System::Diagnostics::Debug::ERROR_FILE_NOT_FOUND,
    );
}
