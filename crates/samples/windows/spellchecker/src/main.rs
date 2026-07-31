use windows::Win32::*;
use windows::core::*;

fn main() -> Result<()> {
    let input = std::env::args()
        .nth(1)
        .expect("Expected one command line argument for text to be spell-corrected");

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;

        let factory: ISpellCheckerFactory =
            CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_ALL as u32)?;

        let locale = w!("en-US");
        assert!(
            factory.IsSupported(locale)?.as_bool(),
            "en-US must be supported"
        );

        let checker = factory.CreateSpellChecker(locale)?;

        println!("Checking the text: '{input}'");
        let text = HSTRING::from(&input);
        let errors = checker.ComprehensiveCheck(&text)?;

        // Error offsets index the UTF-16 input, not the original UTF-8 string.
        let wide: &[u16] = &text;

        // The terminal S_FALSE projects as Err, ending this Result-based enumeration.
        while let Ok(error) = errors.Next() {
            let start_index = error.StartIndex()? as usize;
            let length = error.Length()? as usize;
            let substring = String::from_utf16_lossy(&wide[start_index..start_index + length]);

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
                    let suggestions = checker.Suggest(&HSTRING::from(&substring))?;

                    loop {
                        let mut suggestion = [PWSTR::null()];
                        // This enumerator returns HRESULT and signals completion with a null slot.
                        suggestions.Next(&mut suggestion, None).ok()?;

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
