fn main() {
    windows::build! {
        Windows::Win32::Foundation::{BOOL, PWSTR, S_FALSE},
        Windows::Win32::Globalization::{
            IEnumSpellingError, ISpellChecker, ISpellCheckerFactory, ISpellingError,
            SpellCheckerFactory, CORRECTIVE_ACTION,
        },
        Windows::Win32::System::Com::IEnumString,
    };
}
