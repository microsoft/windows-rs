fn main() {
    windows::core::build! {
        Windows::Win32::Globalization::{SetThreadPreferredUILanguages, MUI_LANGUAGE_NAME},
    };
}
