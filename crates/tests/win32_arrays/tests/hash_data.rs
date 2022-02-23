use windows::{core::*, Win32::UI::Shell::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let mut output: [u8; 4] = [0; 4];
        HashData(&[1, 2, 3, 4, 5, 6, 7, 8], &mut output)?;
        assert_eq!(output, [0xeb, 0x1f, 0xb9, 0x23]);
        Ok(())
    }
}
