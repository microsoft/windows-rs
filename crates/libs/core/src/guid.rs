use super::*;

/// A globally unique identifier ([GUID](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid))
/// used to identify COM and WinRT interfaces.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GUID {
    /// Specifies the first 8 hexadecimal digits.
    pub data1: u32,

    /// Specifies the first group of 4 hexadecimal digits.
    pub data2: u16,

    /// Specifies the second group of 4 hexadecimal digits.
    pub data3: u16,

    /// The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits.
    pub data4: [u8; 8],
}

impl GUID {
    /// Creates a unique `GUID` value.
    pub fn new() -> Result<Self> {
        unsafe { imp::CoCreateGuid() }
    }

    /// Creates a `GUID` represented by the all-zero byte-pattern.
    pub const fn zeroed() -> Self {
        Self {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    /// Creates a `GUID` with the given constant values.
    pub const fn from_values(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    /// Creates a `GUID` from a `u128` value.
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: ((uuid >> 80) & 0xffff) as u16,
            data3: ((uuid >> 64) & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }

    /// Converts a `GUID` to a `u128` value.
    pub const fn to_u128(&self) -> u128 {
        ((self.data1 as u128) << 96)
            + ((self.data2 as u128) << 80)
            + ((self.data3 as u128) << 64)
            + u64::from_be_bytes(self.data4) as u128
    }

    /// Creates a `GUID` for a "generic" WinRT type.
    pub const fn from_signature(signature: imp::ConstBuffer) -> Self {
        let data = imp::ConstBuffer::from_slice(&[
            0x11, 0xf4, 0x7a, 0xd5, 0x7b, 0x73, 0x42, 0xc0, 0xab, 0xae, 0x87, 0x8b, 0x1e, 0x16,
            0xad, 0xee,
        ]);

        let data = data.push_other(signature);

        let bytes = imp::sha1(&data).bytes();
        let first = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        let second = u16::from_be_bytes([bytes[4], bytes[5]]);
        let mut third = u16::from_be_bytes([bytes[6], bytes[7]]);
        third = (third & 0x0fff) | (5 << 12);
        let fourth = (bytes[8] & 0x3f) | 0x80;

        Self::from_values(
            first,
            second,
            third,
            [
                fourth, bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ],
        )
    }

    /// Attempts to parse a `GUID` from a string representation (e.g., `"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use windows_core::GUID;
    ///
    /// let guid = GUID::try_from_str("aBcDeFaB-1234-5678-9012-1234567890ab").unwrap();
    /// assert_eq!(format!("{guid:?}"), "ABCDEFAB-1234-5678-9012-1234567890AB");
    ///
    /// let invalid_guid = GUID::try_from_str("invalid-guid-string");
    /// assert!(invalid_guid.is_none());
    /// ```
    pub const fn try_from_str(from: &str) -> Option<Self> {
        if from.len() != 36 {
            return None;
        }

        let mut bytes = from.as_bytes();

        macro_rules! t {
            ($e:expr) => {
                match $e {
                    Some((val, rest)) => {
                        bytes = rest;
                        val
                    }
                    None => {
                        return None;
                    }
                }
            };
        }

        let mut guid = GUID::zeroed();
        guid.data1 = t!(try_u32(bytes, true));
        guid.data2 = t!(try_u16(bytes, true));
        guid.data3 = t!(try_u16(bytes, true));
        guid.data4[0] = t!(try_u8(bytes, false));
        guid.data4[1] = t!(try_u8(bytes, true));
        guid.data4[2] = t!(try_u8(bytes, false));
        guid.data4[3] = t!(try_u8(bytes, false));
        guid.data4[4] = t!(try_u8(bytes, false));
        guid.data4[5] = t!(try_u8(bytes, false));
        guid.data4[6] = t!(try_u8(bytes, false));
        guid.data4[7] = t!(try_u8(bytes, false));

        if !bytes.is_empty() {
            return None;
        }

        Some(guid)
    }
}

impl RuntimeType for GUID {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"g16");
}

impl TypeKind for GUID {
    type TypeKind = CopyType;
}

impl core::fmt::LowerHex for GUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

impl core::fmt::UpperHex for GUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

impl core::fmt::Debug for GUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(self, f)
    }
}

impl TryFrom<&str> for GUID {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self> {
        Self::try_from_str(from).ok_or_else(invalid_guid)
    }
}

impl core::str::FromStr for GUID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from_str(s).ok_or_else(invalid_guid)
    }
}

impl From<u128> for GUID {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<GUID> for u128 {
    fn from(value: GUID) -> Self {
        value.to_u128()
    }
}

fn invalid_guid() -> Error {
    Error::from_hresult(imp::E_INVALIDARG)
}

const fn try_u32(bytes: &[u8], delimiter: bool) -> Option<(u32, &[u8])> {
    next(bytes, 8, delimiter)
}

const fn try_u16(bytes: &[u8], delimiter: bool) -> Option<(u16, &[u8])> {
    if let Some((val, bytes)) = next(bytes, 4, delimiter) {
        Some((val as u16, bytes))
    } else {
        None
    }
}

const fn try_u8(bytes: &[u8], delimiter: bool) -> Option<(u8, &[u8])> {
    if let Some((val, bytes)) = next(bytes, 2, delimiter) {
        Some((val as u8, bytes))
    } else {
        None
    }
}

const fn next(mut bytes: &[u8], len: usize, delimiter: bool) -> Option<(u32, &[u8])> {
    let mut value: u32 = 0;

    let mut i = 0;
    while i < len {
        let digit = match bytes {
            &[digit, ref rest @ ..] => {
                bytes = rest;
                digit
            }
            _ => return None,
        };
        match digit {
            b'0'..=b'9' => value = (value << 4) + (digit - b'0') as u32,
            b'A'..=b'F' => value = (value << 4) + (digit - b'A' + 10) as u32,
            b'a'..=b'f' => value = (value << 4) + (digit - b'a' + 10) as u32,
            _ => return None,
        }

        i += 1;
    }

    if delimiter {
        match bytes {
            &[b'-', ref rest @ ..] => {
                bytes = rest;
            }
            _ => return None,
        }
    }

    Some((value, bytes))
}

/// Creates a `GUID` constant from a string literal.
///
/// # Examples
///
/// ```
/// use windows_core::guid;
///
/// const MY_GUID: windows_core::GUID = guid!("aBcDeFaB-1234-5678-9012-1234567890ab");
/// assert_eq!(format!("{MY_GUID:?}"), "ABCDEFAB-1234-5678-9012-1234567890AB");
/// ```
///
/// ```compile_fail
/// use windows_core::guid;
///
/// // This will fail to compile because the GUID string is invalid.
/// const INVALID_GUID: windows_core::GUID = guid!("invalid-guid-string");
/// ```
#[macro_export]
macro_rules! guid {
    ($guid:literal) => {{
        const GUID: $crate::GUID = match $crate::GUID::try_from_str($guid) {
            Some(guid) => guid,
            None => panic!(concat!("Invalid GUID: ", $guid)),
        };
        GUID
    }};
}
