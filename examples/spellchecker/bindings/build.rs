fn main() {
    windows::build!(
        Windows::Win32::Intl::{ISpellChecker, SpellCheckerFactory, ISpellCheckerFactory, CORRECTIVE_ACTION, IEnumSpellingError, ISpellingError},
        Windows::Win32::SystemServices::{BOOL, PWSTR},
        Windows::Win32::Com::IEnumString
    )
}
