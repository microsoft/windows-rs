//#![allow(exceeding_bitshifts)]

#[repr(C)]
#[derive(Default, PartialEq)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    pub fn from_values(data1: u32, data2: u16, data3: u16, data4: &[u8; 8]) -> Guid {
        Guid {
            data1,
            data2,
            data3,
            data4: *data4,
        }
    }
}

impl From<&str> for Guid {
    fn from(value: &str) -> Guid {
        assert!(value.len() == 36);
        let mut bytes = value.bytes();

        let a = ((hex_u32(bytes.next().unwrap()) * 16 + hex_u32(bytes.next().unwrap())) << 24)
            + ((hex_u32(bytes.next().unwrap()) * 16 + hex_u32(bytes.next().unwrap())) << 16)
            + ((hex_u32(bytes.next().unwrap()) * 16 + hex_u32(bytes.next().unwrap())) << 8)
            + (hex_u32(bytes.next().unwrap()) * 16 + hex_u32(bytes.next().unwrap()));

        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");

        let b = ((hex_u16(bytes.next().unwrap()) * 16 + hex_u16(bytes.next().unwrap())) << 8)
            + (hex_u16(bytes.next().unwrap()) * 16 + hex_u16(bytes.next().unwrap()));

        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");

        let c = ((hex_u16(bytes.next().unwrap()) * 16 + hex_u16(bytes.next().unwrap())) << 8)
            + (hex_u16(bytes.next().unwrap()) * 16 + hex_u16(bytes.next().unwrap()));

        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");

        let d = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());
        let e = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());

        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");

        let f = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());
        let g = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());
        let h = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());
        let i = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());
        let j = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());
        let k = hex_u8(bytes.next().unwrap()) * 16 + hex_u8(bytes.next().unwrap());

        Guid::from_values(a, b, c, &[d, e, f, g, h, i, j, k])
    }
}

fn hex_u32(value: u8) -> u32 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'A'..=b'F' => 10 + value - b'A',
        b'a'..=b'f' => 10 + value - b'a',
        _ => panic!("Invalid GUID string"),
    }
    .into()
}

fn hex_u16(value: u8) -> u16 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'A'..=b'F' => 10 + value - b'A',
        b'a'..=b'f' => 10 + value - b'a',
        _ => panic!("Invalid GUID string"),
    }
    .into()
}

fn hex_u8(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'A'..=b'F' => 10 + value - b'A',
        b'a'..=b'f' => 10 + value - b'a',
        _ => panic!("Invalid GUID string"),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn guid_from_string() {
        let a = Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99");

        let b = Guid::from_values(
            0xCFF52E04,
            0xCCA6,
            0x4614,
            &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99],
        );

        assert!(a == b);
    }
}
