use super::*;

#[test]
fn pack_version_uses_sixteen_bits_per_part() {
    assert_eq!(pack_version("2.6.4"), 0x0002_0006_0004_0000);
    assert_eq!(pack_version("1.65535.3.4"), 0x0001_ffff_0003_0004);
}
