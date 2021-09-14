use test_enums::Windows::Win32::System::Console::{
    CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
};

#[test]
fn not() {
    assert_eq!(
        !ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        CONSOLE_MODE(!ENABLE_VIRTUAL_TERMINAL_PROCESSING.0)
    );
}
