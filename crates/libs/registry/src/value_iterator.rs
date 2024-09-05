use super::*;

/// An iterator of registry values.
pub struct ValueIterator<'a> {
    key: &'a Key,
    idx: u32,
    name: Vec<u16>,
    data: Data,
}

impl<'a> ValueIterator<'a> {
    pub(crate) fn new(key: &'a Key) -> Result<Self> {
        let info = get_key_info(key)?;

        Ok(Self {
            key,
            idx: 0,
            name: vec![0; (info.value_name_max + 1) as usize],
            data: Data::new(info.value_data_max as usize),
        })
    }

    fn resize(&mut self) -> Result<()> {
        let info = get_key_info(self.key)?;
        self.name.resize((info.value_name_max + 1) as usize, 0);
        self.data = Data::new(info.value_data_max as usize);
        Ok(())
    }
}

impl<'a> Iterator for ValueIterator<'a> {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ty = 0;
        let mut name_len = self.name.len() as u32;
        let mut data_len = self.data.len() as u32;

        let result = unsafe {
            RegEnumValueW(
                self.key.0,
                self.idx,
                self.name.as_mut_ptr(),
                &mut name_len,
                core::ptr::null(),
                &mut ty,
                self.data.as_mut_ptr(),
                &mut data_len,
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
        let name = String::from_utf16_lossy(&self.name[0..name_len as usize]);
        Some((
            name,
            Value {
                data: Data::from_slice(&self.data[0..data_len as usize]),
                ty: ty.into(),
            },
        ))
    }
}
