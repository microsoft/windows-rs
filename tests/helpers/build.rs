fn main() {
    windows::build! {
        Windows::Win32::Globalization::{
            SetThreadPreferredUILanguages, MUI_LANGUAGE_NAME,
        },
    };
}
