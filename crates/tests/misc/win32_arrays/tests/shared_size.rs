use windows::{core::*, Win32::Graphics::Gdi::*, Win32::UI::ColorSystem::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let mut results: [u8; 2] = [255; 2];
        let dc = GetDC(None);

        assert!(0 != SetICMMode(dc, ICM_ON));

        let input = [
            RGBTRIPLE {
                rgbtBlue: 1,
                rgbtGreen: 2,
                rgbtRed: 3,
            },
            RGBTRIPLE {
                rgbtBlue: 4,
                rgbtGreen: 5,
                rgbtRed: 6,
            },
        ];

        assert_eq!(results[0], 255);
        assert_eq!(results[1], 255);

        CheckColorsInGamut(dc, input.as_ptr(), results.as_mut_ptr() as _, 2).ok()?;

        assert_eq!(results[0], 0);
        assert_eq!(results[1], 0);

        Ok(())
    }
}
