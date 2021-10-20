fn main() {
    windows::build! {
        Windows::Win32::Graphics::DirectDraw::CO_E_NOTINITIALIZED,
        Windows::Win32::System::Com::DoDragDrop,
    };
}
