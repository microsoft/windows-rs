fn main() {
    windows::runtime::build! {
        Windows::Win32::Globalization::{SetThreadPreferredUILanguages, MUI_LANGUAGE_NAME},
    };
}
