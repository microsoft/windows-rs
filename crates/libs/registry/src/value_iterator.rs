use super::*;
use core::ptr::null_mut;

/// An iterator of registry values.
pub struct ValueIterator<'a> {
    key: &'a Key,
    range: core::ops::Range<usize>,
    name: Vec<u16>,
    value: ValueBytes,
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

        win32_error(result)?;

        Ok(Self {
            key,
            range: 0..count as usize,
            name: vec![0; name_max_len as usize + 1],
            value: ValueBytes::new(value_max_len as usize)?,
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

// Minimal `Vec` replacement providing `u16` alignment.
struct ValueBytes(*mut core::ffi::c_void, usize);

impl ValueBytes {
    fn new(len: usize) -> Result<Self> {
        // This pointer will have at least 8 byte alignment.
        let ptr = unsafe { HeapAlloc(GetProcessHeap(), 0, len) };

        if ptr.is_null() {
            Err(Error::from_hresult(HRESULT::from_win32(ERROR_OUTOFMEMORY)))
        } else {
            Ok(Self(ptr, len))
        }
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0 as *mut u8
    }
}

impl Drop for ValueBytes {
    fn drop(&mut self) {
        unsafe {
            HeapFree(GetProcessHeap(), 0, self.0);
        };
    }
}

impl core::ops::Deref for ValueBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.0 as *const u8, self.1) }
    }
}
