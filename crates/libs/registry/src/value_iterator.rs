use super::*;
use core::ptr::null_mut;

/// An iterator of registry values.
pub struct ValueIterator<'a> {
    key: &'a Key,
    range: core::ops::Range<usize>,
    name: Vec<u16>,
    value: Vec<u8>,
}

impl<'a> ValueIterator<'a> {
    pub(crate) fn new(key: &'a Key) -> Result<Self> {
        let mut count = 0;
        let mut name_max_len = 0;
        let mut value_max_len = 0;

        let result = unsafe {
            RegQueryInfoKeyW(
                key.0,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                &mut count,
                &mut name_max_len,
                &mut value_max_len,
                null_mut(),
                null_mut(),
            )
        };

        win32_error(result).map(|_| Self {
            key,
            range: 0..count as usize,
            name: vec![0; name_max_len as usize + 1],
            value: vec![0; value_max_len as usize],
        })
    }
}

impl<'a> Iterator for ValueIterator<'a> {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().and_then(|index| {
            let mut ty = 0;
            let mut name_len = self.name.len() as u32;
            let mut value_len = self.value.len() as u32;

            let result = unsafe {
                RegEnumValueW(
                    self.key.0,
                    index as u32,
                    self.name.as_mut_ptr(),
                    &mut name_len,
                    core::ptr::null(),
                    &mut ty,
                    self.value.as_mut_ptr(),
                    &mut value_len,
                )
            };

            if result != 0 {
                debug_assert_eq!(result, ERROR_NO_MORE_ITEMS);
                None
            } else {
                let value = match ty {
                    REG_DWORD if value_len == 4 => {
                        Value::U32(u32::from_le_bytes(self.value[0..4].try_into().unwrap()))
                    }
                    REG_QWORD if value_len == 8 => {
                        Value::U64(u64::from_le_bytes(self.value[0..8].try_into().unwrap()))
                    }
                    REG_BINARY => Value::Bytes(self.value[0..value_len as usize].to_vec()),
                    REG_SZ | REG_EXPAND_SZ => {
                        if value_len == 0 {
                            Value::String(String::new())
                        } else {
                            let value = unsafe {
                                // TODO: Vec<u8> does not guarantee alignment for u16
                                core::slice::from_raw_parts(
                                    self.value.as_ptr() as *const u16,
                                    value_len as usize / 2,
                                )
                            };

                            Value::String(String::from_utf16_lossy(trim(value)))
                        }
                    }
                    REG_MULTI_SZ => {
                        if value_len == 0 {
                            Value::MultiString(vec![])
                        } else {
                            let value = unsafe {
                                // TODO: Vec<u8> does not guarantee alignment for u16
                                core::slice::from_raw_parts(
                                    self.value.as_ptr() as *const u16,
                                    value_len as usize / 2,
                                )
                            };

                            Value::MultiString(
                                trim(value)
                                    .split(|c| *c == 0)
                                    .map(String::from_utf16_lossy)
                                    .collect(),
                            )
                        }
                    }
                    rest => Value::Unknown(rest),
                };

                let name = String::from_utf16_lossy(&self.name[0..name_len as usize]);
                Some((name, value))
            }
        })
    }
}
