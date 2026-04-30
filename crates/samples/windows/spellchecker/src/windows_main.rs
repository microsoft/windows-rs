use windows::{core::*, Win32::Foundation::*, Win32::Globalization::*, Win32::System::Com::*};

fn main() -> Result<()> {
    let input = std::env::args()
        .nth(1)
        .expect("Expected one command line argument for text to be spell-corrected");
    // Initialize the COM runtime for this thread
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
    }

    // Create ISpellCheckerFactory
    let factory: ISpellCheckerFactory =
        unsafe { CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_ALL)? };

    // Make sure that the "en-US" locale is supported
    let locale = w!("en-US");
    let supported = unsafe { factory.IsSupported(locale)? };
    supported.expect("en-US is supported");

    // Create a ISpellChecker
    let checker = unsafe { factory.CreateSpellChecker(locale)? };

    // Get errors enumerator for the supplied string
    println!("Checking the text: '{input}'");
    let errors = unsafe { checker.ComprehensiveCheck(&HSTRING::from(&input))? };

    // Loop through all the errors
    loop {
        let mut error = None;

        if unsafe { errors.Next(&mut error) } != S_OK {
            break;
        }

        let error = error.unwrap();

        // Get the start index and length of the error
        let start_index = unsafe { error.StartIndex()? };
        let length = unsafe { error.Length()? };

        // Get the substring from the utf8 encoded string
        let substring = &input[start_index as usize..(start_index + length) as usize];

        // Get the corrective action
        let action = unsafe { error.CorrectiveAction()? };
        println!("{action:?}");

        match action {
            CORRECTIVE_ACTION_DELETE => {
                println!("Delete '{substring}'");
            }
            CORRECTIVE_ACTION_REPLACE => {
                // Get the replacement as a widestring and convert to a Rust String
                let replacement = unsafe { error.Replacement()? };

                println!("Replace: {substring} with {}", unsafe {
                    replacement.display()
                });

                unsafe { CoTaskMemFree(Some(replacement.as_ptr() as *mut _)) };
            }
            CORRECTIVE_ACTION_GET_SUGGESTIONS => {
                // Get an enumerator for all the suggestions for a substring
                let suggestions = unsafe { checker.Suggest(&HSTRING::from(substring))? };

                // Loop through the suggestions
                loop {
                    // Get the next suggestion breaking if the call to `Next` failed
                    let mut suggestion = [PWSTR::null()];
                    unsafe {
                        _ = suggestions.Next(&mut suggestion, None);
                    }
                    if suggestion[0].is_null() {
                        break;
                    }

                    println!("Maybe replace: {substring} with {}", unsafe {
                        suggestion[0].display()
                    });

                    unsafe { CoTaskMemFree(Some(suggestion[0].as_ptr() as *mut _)) };
                }
            }
            _ => {}
        }
    }
    Ok(())
}
