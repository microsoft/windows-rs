use super::*;
use Primitives::*;

const GUID: GUID = GUID::from_u128(0x00112233_4455_6677_8899_aabbccddeeff);
const GUID_TEXT: &str = "00112233-4455-6677-8899-AABBCCDDEEFF";

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert!(!widget.NotBool(true)?);
    assert!(widget.NotBool(false)?);
    assert_eq!(widget.NextChar16(u16::from(b'a'))?, u16::from(b'b'));
    assert_eq!(widget.NextChar16(0xfffe)?, 0xffff);
    assert_eq!(widget.NextU8(0xfe)?, 0xff);
    assert_eq!(widget.NextI16(i16::MIN)?, i16::MIN + 1);
    assert_eq!(widget.NextU16(0xfffe)?, 0xffff);
    assert_eq!(widget.NextI32(i32::MIN)?, i32::MIN + 1);
    assert_eq!(widget.NextU32(0xffff_fffe)?, 0xffff_ffff);
    assert_eq!(widget.NextI64(i64::MIN)?, i64::MIN + 1);
    assert_eq!(widget.NextU64(u64::MAX - 1)?, u64::MAX);
    assert_eq!(widget.DoubleF32(1.5)?, 3.0);
    assert_eq!(widget.DoubleF64(1e300)?, 2e300);

    assert_eq!(widget.FormatGuid(GUID)?, GUID_TEXT);
    assert_eq!(
        widget.ParseGuid(h!("00112233-4455-6677-8899-AABBCCDDEEFF"))?,
        GUID
    );

    Ok(())
}
