fn main() {
    windows::build!(
        windows::win32::intl::{ISpellChecker, SpellCheckerFactory, ISpellCheckerFactory, CORRECTIVE_ACTION, IEnumSpellingError, ISpellingError},
        windows::win32::system_services::{BOOL, PWSTR},
        windows::win32::com::IEnumString
    )
}
