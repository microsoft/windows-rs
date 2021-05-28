fn main() {
    windows::build! {
        Windows::Win32::Globalization::{
            ISpellChecker, SpellCheckerFactory, ISpellCheckerFactory, IEnumSpellingError,
            ISpellingError, CORRECTIVE_ACTION_NONE, CORRECTIVE_ACTION_DELETE,
            CORRECTIVE_ACTION_REPLACE, CORRECTIVE_ACTION_GET_SUGGESTIONS,
        },
        Windows::Win32::System::SystemServices::{BOOL, PWSTR, S_FALSE},
        Windows::Win32::System::Com::IEnumString
    };
}
