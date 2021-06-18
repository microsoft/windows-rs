use bindings::Windows::Win32;
use Win32::Foundation::{BOOL, PWSTR, S_FALSE};
use Win32::Globalization;
use Win32::System::Com::CoTaskMemFree;

fn main() -> windows::Result<()> {
    let input = std::env::args()
        .nth(1)
        .expect("Expected one command line argument for text to be spell-corrected");
    // Initialize the COM runtime for this thread
    windows::initialize_mta()?;

    // Create ISpellCheckerFactory
    let factory: Globalization::ISpellCheckerFactory =
        windows::create_instance(&Globalization::SpellCheckerFactory)?;

    // Make sure that the "en-US" locale is supported
    let mut supported: BOOL = false.into();
    let locale = "en-US";
    unsafe { factory.IsSupported(locale, &mut supported).ok()? };
    supported.expect("en-US is supported");

    // Create a ISpellChecker
    let mut checker = None;
    let checker = unsafe { factory.CreateSpellChecker(locale, &mut checker) }.and_some(checker)?;

    // Get errors enumerator for the supplied string
    println!("Checking the text: '{}'", input);
    let mut errors = None;
    let errors =
        unsafe { checker.ComprehensiveCheck(input.clone(), &mut errors) }.and_some(errors)?;

    // Loop through all the errors
    loop {
        // Get the next error in the enumerator
        let mut error = None;
        let result = unsafe { errors.Next(&mut error) };
        if result == S_FALSE {
            break;
        }
        let error = result.and_some(error)?;

        // Get the start index and length of the error
        let mut start_index = 0u32;
        let mut length = 0u32;
        unsafe {
            error.get_StartIndex(&mut start_index).ok()?;
            error.get_Length(&mut length).ok()?;
        }

        // Get the substring from the utf8 encoded string
        let substring = &input[start_index as usize..(start_index + length) as usize];

        // Get the corrective action
        let mut action = Globalization::CORRECTIVE_ACTION_NONE;
        unsafe { error.get_CorrectiveAction(&mut action).ok()? };
        println!("{:?}", action);

        match action {
            Globalization::CORRECTIVE_ACTION_DELETE => {
                println!("Delete '{}'", substring);
            }
            Globalization::CORRECTIVE_ACTION_REPLACE => {
                // Get the replacement as a widestring and convert to a Rust String
                let mut replacement = PWSTR::NULL;
                unsafe { error.get_Replacement(&mut replacement).ok()? };

                println!("Replace: {} with {}", substring, unsafe {
                    read_to_string(replacement)
                });

                unsafe { CoTaskMemFree(replacement.0 as *mut _) };
            }
            Globalization::CORRECTIVE_ACTION_GET_SUGGESTIONS => {
                // Get an enumerator for all the suggestions for a substring
                let mut suggestions = None;
                let suggestions = unsafe { checker.Suggest(substring, &mut suggestions) }
                    .and_some(suggestions)?;

                // Loop through the suggestions
                loop {
                    // Get the next suggestion breaking if the call to `Next` failed
                    let mut suggestion = PWSTR::NULL;
                    let result =
                        unsafe { suggestions.Next(1, &mut suggestion, std::ptr::null_mut()) };
                    if result == S_FALSE {
                        break;
                    }
                    result.ok()?;

                    println!("Maybe replace: {} with {}", substring, unsafe {
                        read_to_string(suggestion)
                    });

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
