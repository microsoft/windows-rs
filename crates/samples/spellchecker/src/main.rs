use windows::{core::*, Win32::Globalization::*, Win32::System::Com::*};

fn main() -> Result<()> {
    let input = std::env::args().nth(1).expect("Expected one command line argument for text to be spell-corrected");
    // Initialize the COM runtime for this thread
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    }

    // Create ISpellCheckerFactory
    let factory: ISpellCheckerFactory = unsafe { CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_ALL)? };

    // Make sure that the "en-US" locale is supported
    let locale = "en-US";
    let supported = unsafe { factory.IsSupported(locale)? };
    supported.expect("en-US is supported");

    // Create a ISpellChecker
    let checker = unsafe { factory.CreateSpellChecker(locale)? };

    // Get errors enumerator for the supplied string
    println!("Checking the text: '{}'", input);
    let errors = unsafe { checker.ComprehensiveCheck(input.clone())? };

    // Loop through all the errors
    while let Ok(error) = unsafe { errors.Next() } {
        // Get the start index and length of the error
        let start_index = unsafe { error.StartIndex()? };
        let length = unsafe { error.Length()? };

        // Get the substring from the utf8 encoded string
        let substring = &input[start_index as usize..(start_index + length) as usize];

        // Get the corrective action
        let action = unsafe { error.CorrectiveAction()? };
        println!("{:?}", action);

        match action {
            CORRECTIVE_ACTION_DELETE => {
                println!("Delete '{}'", substring);
            }
            CORRECTIVE_ACTION_REPLACE => {
                // Get the replacement as a widestring and convert to a Rust String
                let replacement = unsafe { error.Replacement()? };

                println!("Replace: {} with {}", substring, unsafe { read_to_string(replacement) });

                unsafe { CoTaskMemFree(replacement.0 as *mut _) };
            }
            CORRECTIVE_ACTION_GET_SUGGESTIONS => {
                // Get an enumerator for all the suggestions for a substring
                let suggestions = unsafe { checker.Suggest(substring)? };

                // Loop through the suggestions
                loop {
                    // Get the next suggestion breaking if the call to `Next` failed
                    let mut suggestion = PWSTR::default();
                    unsafe {
                        let _ = suggestions.Next(1, &mut suggestion, std::ptr::null_mut());
                    }
                    if suggestion.0.is_null() {
                        break;
                    }

                    println!("Maybe replace: {} with {}", substring, unsafe { read_to_string(suggestion) });

                    unsafe { CoTaskMemFree(suggestion.0 as *mut _) };
                }
            }
            _ => {}
        }
    }
    Ok(())
}

unsafe fn read_to_string(ptr: PWSTR) -> String {
    let mut len = 0usize;
    let mut cursor = ptr;
    loop {
        let val = cursor.0.read();
        if val == 0 {
            break;
        }
        len += 1;
        cursor = PWSTR(cursor.0.add(1));
    }

    let slice = std::slice::from_raw_parts(ptr.0, len);
    String::from_utf16(slice).unwrap()
}
