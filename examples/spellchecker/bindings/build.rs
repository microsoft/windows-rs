fn main() {
    windows::build! {
        Windows::Win32::Globalization::{
            IEnumSpellingError, ISpellChecker, ISpellCheckerFactory, ISpellingError,
            SpellCheckerFactory, CORRECTIVE_ACTION,
        },
        Windows::Win32::System::Com::IEnumString,
        Windows::Win32::System::SystemServices::{BOOL, PWSTR, S_FALSE},
    };
}
