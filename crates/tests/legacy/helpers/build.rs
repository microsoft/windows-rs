fn main() {
    windows::core::build_legacy! {
        Windows::Win32::Globalization::{SetThreadPreferredUILanguages, MUI_LANGUAGE_NAME},
    };
}
