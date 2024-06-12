#![cfg(windows_debugger_visualizer)]

use debugger_test::*;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;

#[inline(never)]
fn __break() {}

#[implement(IErrorInfo)]
struct Test;

impl IErrorInfo_Impl for Test_Impl {
    fn GetGUID(&self) -> Result<GUID> {
        Err(Error::new(E_OUTOFMEMORY, "Out of memory message"))
    }
    fn GetSource(&self) -> Result<BSTR> {
        Err(Error::new(E_INVALIDARG, "Invalid argument message"))
    }
    fn GetDescription(&self) -> Result<BSTR> {
        Ok(BSTR::new())
    }
    fn GetHelpFile(&self) -> Result<BSTR> {
        Ok(BSTR::new())
    }
    fn GetHelpContext(&self) -> Result<u32> {
        Ok(1)
    }
}

#[debugger_test(
    debugger = "cdb",
    commands = "
.nvlist
dx array

dx -r2 pstr
dx -r2 pcstr
dx -r2 pwstr
dx -r2 pcwstr

dx empty
dx -r2 hstring

dx out_of_memory_error
dx invalid_argument_error
    ",
    expected_statements = r#"
array            : { len=0xd } [Type: windows_core::array::Array<u8>]
    [<Raw View>]     [Type: windows_core::array::Array<u8>]
    [len]            : 0xd
    [0]              : 0x48 [Type: unsigned char]
    [1]              : 0x65 [Type: unsigned char]
    [2]              : 0x6c [Type: unsigned char]
    [3]              : 0x6c [Type: unsigned char]
    [4]              : 0x6f [Type: unsigned char]
    [5]              : 0x20 [Type: unsigned char]
    [6]              : 0x57 [Type: unsigned char]
    [7]              : 0x6f [Type: unsigned char]
    [8]              : 0x72 [Type: unsigned char]
    [9]              : 0x6c [Type: unsigned char]
    [10]             : 0x64 [Type: unsigned char]
    [11]             : 0x21 [Type: unsigned char]
    [12]             : 0x0 [Type: unsigned char]

pstr             : "This is a PSTR" [Type: windows_core::strings::pstr::PSTR]
    [<Raw View>]     [Type: windows_core::strings::pstr::PSTR]
    [len]            : 0xe
    [chars]
        [0]              : 84 'T' [Type: char]
        [1]              : 104 'h' [Type: char]
        [2]              : 105 'i' [Type: char]
        [3]              : 115 's' [Type: char]
        [4]              : 32 ' ' [Type: char]
        [5]              : 105 'i' [Type: char]
        [6]              : 115 's' [Type: char]
        [7]              : 32 ' ' [Type: char]
        [8]              : 97 'a' [Type: char]
        [9]              : 32 ' ' [Type: char]
        [10]             : 80 'P' [Type: char]
        [11]             : 83 'S' [Type: char]
        [12]             : 84 'T' [Type: char]
        [13]             : 82 'R' [Type: char]

pcstr            : "This is a PCSTR" [Type: windows_core::strings::pcstr::PCSTR]
    [<Raw View>]     [Type: windows_core::strings::pcstr::PCSTR]
    [len]            : 0xf
    [chars]
        [0]              : 84 'T' [Type: char]
        [1]              : 104 'h' [Type: char]
        [2]              : 105 'i' [Type: char]
        [3]              : 115 's' [Type: char]
        [4]              : 32 ' ' [Type: char]
        [5]              : 105 'i' [Type: char]
        [6]              : 115 's' [Type: char]
        [7]              : 32 ' ' [Type: char]
        [8]              : 97 'a' [Type: char]
        [9]              : 32 ' ' [Type: char]
        [10]             : 80 'P' [Type: char]
        [11]             : 67 'C' [Type: char]
        [12]             : 83 'S' [Type: char]
        [13]             : 84 'T' [Type: char]
        [14]             : 82 'R' [Type: char]

pwstr            : "This is a PWSTR" [Type: windows_core::strings::pwstr::PWSTR]
    [<Raw View>]     [Type: windows_core::strings::pwstr::PWSTR]
    [len]            : 0xf
    [chars]
        [0]              : 0x54 'T' [Type: char16_t]
        [1]              : 0x68 'h' [Type: char16_t]
        [2]              : 0x69 'i' [Type: char16_t]
        [3]              : 0x73 's' [Type: char16_t]
        [4]              : 0x20 ' ' [Type: char16_t]
        [5]              : 0x69 'i' [Type: char16_t]
        [6]              : 0x73 's' [Type: char16_t]
        [7]              : 0x20 ' ' [Type: char16_t]
        [8]              : 0x61 'a' [Type: char16_t]
        [9]              : 0x20 ' ' [Type: char16_t]
        [10]             : 0x50 'P' [Type: char16_t]
        [11]             : 0x57 'W' [Type: char16_t]
        [12]             : 0x53 'S' [Type: char16_t]
        [13]             : 0x54 'T' [Type: char16_t]
        [14]             : 0x52 'R' [Type: char16_t]

pcwstr           : "This is a PCWSTR" [Type: windows_core::strings::pcwstr::PCWSTR]
    [<Raw View>]     [Type: windows_core::strings::pcwstr::PCWSTR]
    [len]            : 0x10
    [chars]
        [0]              : 0x54 'T' [Type: char16_t]
        [1]              : 0x68 'h' [Type: char16_t]
        [2]              : 0x69 'i' [Type: char16_t]
        [3]              : 0x73 's' [Type: char16_t]
        [4]              : 0x20 ' ' [Type: char16_t]
        [5]              : 0x69 'i' [Type: char16_t]
        [6]              : 0x73 's' [Type: char16_t]
        [7]              : 0x20 ' ' [Type: char16_t]
        [8]              : 0x61 'a' [Type: char16_t]
        [9]              : 0x20 ' ' [Type: char16_t]
        [10]             : 0x50 'P' [Type: char16_t]
        [11]             : 0x43 'C' [Type: char16_t]
        [12]             : 0x57 'W' [Type: char16_t]
        [13]             : 0x53 'S' [Type: char16_t]
        [14]             : 0x54 'T' [Type: char16_t]
        [15]             : 0x52 'R' [Type: char16_t]

empty            : "" [Type: windows_core::strings::hstring::HSTRING]
    [<Raw View>]     [Type: windows_core::strings::hstring::HSTRING]
    [len]            : 0x0 [Type: unsigned int]

hstring          : "This is an HSTRING" [Type: windows_core::strings::hstring::HSTRING]
    [<Raw View>]     [Type: windows_core::strings::hstring::HSTRING]
    [len]            : 0x12 [Type: unsigned int]
    [ref_count]      : 1 [Type: windows_core::imp::ref_count::RefCount]
    [flags]          : 0x0 [Type: unsigned int]
    [chars]
        [0]              : 0x54 'T' [Type: char16_t]
        [1]              : 0x68 'h' [Type: char16_t]
        [2]              : 0x69 'i' [Type: char16_t]
        [3]              : 0x73 's' [Type: char16_t]
        [4]              : 0x20 ' ' [Type: char16_t]
        [5]              : 0x69 'i' [Type: char16_t]
        [6]              : 0x73 's' [Type: char16_t]
        [7]              : 0x20 ' ' [Type: char16_t]
        [8]              : 0x61 'a' [Type: char16_t]
        [9]              : 0x6e 'n' [Type: char16_t]
        [10]             : 0x20 ' ' [Type: char16_t]
        [11]             : 0x48 'H' [Type: char16_t]
        [12]             : 0x53 'S' [Type: char16_t]
        [13]             : 0x54 'T' [Type: char16_t]
        [14]             : 0x52 'R' [Type: char16_t]
        [15]             : 0x49 'I' [Type: char16_t]
        [16]             : 0x4e 'N' [Type: char16_t]
        [17]             : 0x47 'G' [Type: char16_t]

out_of_memory_error : 0x8007000e (Not enough memory resources are available to complete this operation.) [Type: windows_result::error::Error]
    [<Raw View>]     [Type: windows_result::error::Error]
    [info]           : Some [Type: enum2$<core::option::Option<windows_result::com::ComPtr> >]

invalid_argument_error : 0x80070057 (The parameter is incorrect.) [Type: windows_result::error::Error]
    [<Raw View>]     [Type: windows_result::error::Error]
    [info]           : Some [Type: enum2$<core::option::Option<windows_result::com::ComPtr> >]
    "#
)]
fn test_debugger_visualizer() {
    let string = "Hello World!\0".to_string();
    let mut array = Array::<u8>::with_len(string.len());
    for (i, ch) in string.as_bytes().iter().enumerate() {
        array[i] = *ch;
    }

    // Test debugger visualizations for PSTR
    let mut pstr_string = "This is a PSTR\0".to_string();
    let pstr = PSTR::from_raw(pstr_string.as_mut_ptr());
    unsafe {
        assert_eq!(
            &pstr_string.as_bytes()[..(pstr_string.len() - 1)],
            pstr.as_bytes()
        );
    }

    // Test debugger visualizations for PCSTR
    let pcstr_string = "This is a PCSTR\0".to_string();
    let pcstr = PCSTR::from_raw(pcstr_string.as_ptr());
    unsafe {
        assert_eq!(
            &pcstr_string.as_bytes()[..(pcstr_string.len() - 1)],
            pcstr.as_bytes()
        );
    }

    // Test debugger visualizations for PWSTR
    let mut pwstr_string: Vec<u16> = vec![
        84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 80, 87, 83, 84, 82, 0,
    ];
    let pwstr = PWSTR::from_raw(pwstr_string.as_mut_ptr());
    unsafe {
        assert_eq!(
            &pwstr_string.as_slice()[..(pwstr_string.len() - 1)],
            pwstr.as_wide()
        );
    }

    // Test debugger visualizations for PCWSTR
    let pcwstr_string: Vec<u16> = vec![
        84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 80, 67, 87, 83, 84, 82, 0,
    ];
    let pcwstr = PCWSTR::from_raw(pcwstr_string.as_ptr());
    unsafe {
        assert_eq!(
            &pcwstr_string.as_slice()[..(pcwstr_string.len() - 1)],
            pcwstr.as_wide()
        );
    }

    // Test debugger visualizations for HSTRING
    let empty = HSTRING::new();
    assert!(empty.is_empty());

    let hstring = HSTRING::from("This is an HSTRING");
    assert!(!hstring.is_empty());
    assert!(hstring.len() == 18);

    let test: IErrorInfo = Test.into();

    unsafe {
        // Test debugger visualizations for Error
        let result = test.GetGUID();
        let out_of_memory_error = result.unwrap_err();
        assert_eq!(out_of_memory_error.code(), E_OUTOFMEMORY);
        assert_eq!(out_of_memory_error.message(), "Out of memory message");

        let result = test.GetSource();
        let invalid_argument_error = result.unwrap_err();
        assert_eq!(invalid_argument_error.code(), E_INVALIDARG);
        assert_eq!(invalid_argument_error.message(), "Invalid argument message");
        __break();
    }
}
