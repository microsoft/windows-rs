fn main() {
    let mut bits = MixedBits { _bitfield: 0 };
    bits.set_positive(7);
    bits.set_negative(-16);
    bits.set_flag(1);
    bits.set_sign(-1);
    bits.set_mode(Mode::Negative);
    bits.set_state(1);
    bits.set_alias(-7);
    bits.set_alternate(Mode::Positive);
    assert_eq!(bits.positive(), 7u32);
    assert_eq!(bits.negative(), -16i32);
    assert_eq!(bits.flag(), 1u32);
    assert_eq!(bits.sign(), -1i32);
    assert_eq!(bits.mode(), Mode::Negative);
    assert_eq!(bits.state(), 1 as State);
    assert_eq!(bits.alias(), -7i32);
    assert_eq!(bits.alternate(), Mode::Positive);
    let before = bits._bitfield;
    bits.set_negative(31);
    assert_eq!(bits.negative(), -1);
    assert_eq!(bits._bitfield & !(31 << 3), before & !(31 << 3));
    bits.set_flag(2);
    bits.set_sign(0);
    assert_eq!(bits.flag(), 0);
    assert_eq!(bits.sign(), 0);
    bits.set_mode(Mode(3));
    assert_eq!(bits.mode(), Mode(3));

    let mut reverse = ReverseBits { _bitfield: -1 };
    reverse.set_positive(3);
    assert_eq!(reverse.negative(), -1);
    assert_eq!(reverse.positive(), 3u32);
    reverse.set_negative(-16);
    assert_eq!(reverse.negative(), -16);
    assert_eq!(reverse.positive(), 3u32);

    let mut flags = BoolBits { _bitfield: 0 };
    flags.set_enabled(true);
    flags.set_wide(true);
    flags.set_other(15);
    assert!(flags.enabled());
    assert!(flags.wide());
    assert_eq!(flags.other(), 15u8);
    flags.set_enabled(false);
    flags.set_wide(false);
    assert_eq!(flags._bitfield, 0xf0);

    let mut full = FullSignedBits { _bitfield: 0 };
    for value in [i64::MIN, -1, 0, i64::MAX] {
        full.set_value(value);
        assert_eq!(full.value(), value);
    }

    let mut union = TypedUnion { storage: u32::MAX };
    union.set_mode(Mode::Positive);
    assert_eq!(union.mode(), Mode::Positive);
    assert_eq!(unsafe { union.storage }, (u32::MAX & !7) | 1);
    union.set_sign(0);
    assert_eq!(union.sign(), 0i32);
    assert_eq!(unsafe { union.storage }, u32::MAX & !7);
}
