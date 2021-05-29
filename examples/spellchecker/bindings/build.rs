fn main() {
    windows::build! {
        Windows::Win32::Globalization::{
            IEnumSpellingError, ISpellChecker, ISpellCheckerFactory, ISpellingError,
            SpellCheckerFactory, CORRECTIVE_ACTION_DELETE, CORRECTIVE_ACTION_GET_SUGGESTIONS,
            CORRECTIVE_ACTION_NONE, CORRECTIVE_ACTION_REPLACE,
        },
        Windows::Win32::System::Com::IEnumString,
        Windows::Win32::System::SystemServices::{BOOL, PWSTR, S_FALSE},
    };
}
