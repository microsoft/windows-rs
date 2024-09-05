use super::*;

/// An iterator of registry key names.
pub struct KeyIterator<'a> {
    key: &'a Key,
    idx: u32,
    name: Vec<u16>,
}

impl<'a> KeyIterator<'a> {
    pub(crate) fn new(key: &'a Key) -> Result<Self> {
        let info = get_key_info(key)?;

        Ok(Self {
            key,
            idx: 0,
            name: vec![0; (info.subkey_name_max + 1) as usize],
        })
    }

    fn resize(&mut self) -> Result<()> {
        let info = get_key_info(self.key)?;
        self.name.resize((info.subkey_name_max + 1) as usize, 0);
        Ok(())
    }
}

impl<'a> Iterator for KeyIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut len = self.name.len() as u32;
        let result = unsafe {
            RegEnumKeyExW(
                self.key.0,
                self.idx,
                self.name.as_mut_ptr(),
                &mut len,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };

        if result == ERROR_MORE_DATA {
            if self.resize().is_err() {
                return None;
            }
            return self.next();
        }

        if result != ERROR_SUCCESS {
            debug_assert_eq!(result, ERROR_NO_MORE_ITEMS);
            return None;
        }

        self.idx += 1;
        Some(String::from_utf16_lossy(&self.name[0..len as usize]))
    }
}
