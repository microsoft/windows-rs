fn main() {
    windows::build!(
        Windows::Win32::Globalization::{ISpellChecker, SpellCheckerFactory, ISpellCheckerFactory, CORRECTIVE_ACTION, IEnumSpellingError, ISpellingError},
        Windows::Win32::System::SystemServices::{BOOL, PWSTR, S_FALSE},
        Windows::Win32::System::Com::IEnumString
    )
}
