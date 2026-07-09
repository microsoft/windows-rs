// Generates the Windows spell-checking API (`ISpellCheckerFactory`, `ISpellChecker`,
// `IEnumSpellingError`, …) from the in-house Win32 metadata with `windows-bindgen`,
// then checks a line of text supplied on the command line and prints corrections.

#![allow(unused_qualifications, nonstandard_style, clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use windows_core::*;

fn main() -> Result<()> {
    let input = std::env::args()
        .nth(1)
        .expect("Expected one command line argument for text to be spell-corrected");

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED.0 as u32).ok()?;

        let factory: ISpellCheckerFactory =
            CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_ALL)?;

        // Make sure that the "en-US" locale is supported.
        let locale = w!("en-US");
        assert!(factory.IsSupported(locale)?.as_bool(), "en-US is supported");

        let checker = factory.CreateSpellChecker(locale)?;

        println!("Checking the text: '{input}'");
        let errors = checker.ComprehensiveCheck(&HSTRING::from(&input))?;

        // `IEnumSpellingError::Next` carries a `[retval]`, so it projects as
        // `-> Result<ISpellingError>`; the terminal `S_FALSE` surfaces as `Err`, so
        // iterating until the first `Err` walks the whole enumeration.
        while let Ok(error) = errors.Next() {
            let start_index = error.StartIndex()? as usize;
            let length = error.Length()? as usize;
            let substring = &input[start_index..start_index + length];

            let action = error.CorrectiveAction()?;
            println!("{action:?}");

            match action {
                CORRECTIVE_ACTION_DELETE => {
                    println!("Delete '{substring}'");
                }
                CORRECTIVE_ACTION_REPLACE => {
                    let replacement = error.Replacement()?;
                    println!("Replace: {substring} with {}", replacement.display());
                    CoTaskMemFree(replacement.as_ptr() as *mut _);
                }
                CORRECTIVE_ACTION_GET_SUGGESTIONS => {
                    let suggestions = checker.Suggest(&HSTRING::from(substring))?;

                    // `IEnumString::Next` has no `[retval]` (it takes an array plus a
                    // count out-param), so it projects as `-> HRESULT`; a null slot
                    // signals the end of the enumeration.
                    loop {
                        let mut suggestion = [PWSTR::null()];
                        _ = suggestions.Next(&mut suggestion, None);

                        if suggestion[0].is_null() {
                            break;
                        }

                        println!(
                            "Maybe replace: {substring} with {}",
                            suggestion[0].display()
                        );

                        CoTaskMemFree(suggestion[0].as_ptr() as *mut _);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
