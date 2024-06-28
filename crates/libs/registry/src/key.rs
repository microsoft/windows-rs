use super::*;
use core::ptr::{null, null_mut};

/// A registry key.
#[repr(transparent)]
#[derive(Debug)]
pub struct Key(pub(crate) HKEY);

impl Default for Key {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}

impl Key {
    /// Creates a registry key. If the key already exists, the function opens it.
    pub fn create<T: AsRef<str>>(&self, path: T) -> Result<Self> {
        let mut handle = core::ptr::null_mut();

        let result = unsafe {
            RegCreateKeyExW(
                self.0,
                pcwstr(path).as_ptr(),
                0,
                null(),
                REG_OPTION_NON_VOLATILE,
                KEY_READ | KEY_WRITE,
                null(),
                &mut handle,
                null_mut(),
            )
        };

        win32_error(result).map(|_| Self(handle))
    }

    /// Opens a registry key.
    pub fn open<T: AsRef<str>>(&self, path: T) -> Result<Self> {
        let mut handle = core::ptr::null_mut();

        let result =
            unsafe { RegOpenKeyExW(self.0, pcwstr(path).as_ptr(), 0, KEY_READ, &mut handle) };

        win32_error(result).map(|_| Self(handle))
    }

    /// Constructs a registry key from an existing handle.
    ///
    /// # Safety
    ///
    /// This function takes ownership of the handle.
    /// The handle must be owned by the caller and safe to free with `RegCloseKey`.
    pub unsafe fn from_raw(handle: *mut core::ffi::c_void) -> Self {
        Self(handle)
    }

    /// Returns the underlying registry key handle.
    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        self.0
    }

    /// Removes the registry keys and values of the specified key recursively.
    pub fn remove_tree<T: AsRef<str>>(&self, path: T) -> Result<()> {
        let result = unsafe { RegDeleteTreeW(self.0, pcwstr(path).as_ptr()) };
        win32_error(result)
    }

    /// Removes the registry value.
    pub fn remove_value<T: AsRef<str>>(&self, name: T) -> Result<()> {
        let result = unsafe { RegDeleteValueW(self.0, pcwstr(name).as_ptr()) };
        win32_error(result)
    }

    /// Creates an iterator of registry key names.
    pub fn keys(&self) -> Result<KeyIterator<'_>> {
        KeyIterator::new(self)
    }

    /// Creates an iterator of registry values.
    pub fn values(&self) -> Result<ValueIterator<'_>> {
        ValueIterator::new(self)
    }

    /// Sets the name and value in the registry key.
    pub fn set_u32<T: AsRef<str>>(&self, name: T, value: u32) -> Result<()> {
        unsafe { self.set_value(name, REG_DWORD, &value as *const _ as _, 4) }
    }

    /// Sets the name and value in the registry key.
    pub fn set_u64<T: AsRef<str>>(&self, name: T, value: u64) -> Result<()> {
        unsafe { self.set_value(name, REG_QWORD, &value as *const _ as _, 8) }
    }

    /// Sets the name and value in the registry key.
    pub fn set_string<T: AsRef<str>>(&self, name: T, value: T) -> Result<()> {
        let value = pcwstr(value);

        unsafe { self.set_value(name, REG_SZ, value.as_ptr() as _, value.len() * 2) }
    }

    /// Sets the name and value in the registry key.
    pub fn set_hstring<T: AsRef<str>>(
        &self,
        name: T,
        value: &windows_strings::HSTRING,
    ) -> Result<()> {
        unsafe { self.set_value(name, REG_SZ, value.as_ptr() as _, value.len() * 2) }
    }

    /// Sets the name and value in the registry key.
    pub fn set_multi_string<T: AsRef<str>>(&self, name: T, value: &[T]) -> Result<()> {
        let mut packed = value.iter().fold(vec![0u16; 0], |mut packed, value| {
            packed.append(&mut pcwstr(value));
            packed
        });

        packed.push(0);

        unsafe { self.set_value(name, REG_MULTI_SZ, packed.as_ptr() as _, packed.len() * 2) }
    }

    /// Sets the name and value in the registry key.
    pub fn set_bytes<T: AsRef<str>>(&self, name: T, value: &[u8]) -> Result<()> {
        unsafe { self.set_value(name, REG_BINARY, value.as_ptr() as _, value.len()) }
    }

    /// Gets the type for the name in the registry key.
    pub fn get_type<T: AsRef<str>>(&self, name: T) -> Result<Type> {
        let name = pcwstr(name);
        let mut ty = 0;

        let result = unsafe {
            RegQueryValueExW(
                self.0,
                name.as_ptr(),
                null(),
                &mut ty,
                null_mut(),
                null_mut(),
            )
        };

        win32_error(result)?;

        Ok(match ty {
            REG_DWORD => Type::U32,
            REG_QWORD => Type::U64,
            REG_BINARY => Type::Bytes,
            REG_SZ | REG_EXPAND_SZ => Type::String,
            REG_MULTI_SZ => Type::MultiString,
            rest => Type::Unknown(rest),
        })
    }

    /// Gets the value for the name in the registry key.
    pub fn get_value<T: AsRef<str>>(&self, name: T) -> Result<Value> {
        let name = pcwstr(name);
        let mut ty = 0;
        let mut len = 0;

        let result = unsafe {
            RegQueryValueExW(self.0, name.as_ptr(), null(), &mut ty, null_mut(), &mut len)
        };

        win32_error(result)?;

        match ty {
            REG_DWORD if len == 4 => {
                let mut value = 0u32;

                let result = unsafe {
                    RegQueryValueExW(
                        self.0,
                        name.as_ptr(),
                        null(),
                        null_mut(),
                        &mut value as *mut _ as _,
                        &mut len,
                    )
                };

                win32_error(result)?;
                Ok(Value::U32(value))
            }
            REG_QWORD if len == 8 => {
                let mut value = 0u64;

                let result = unsafe {
                    RegQueryValueExW(
                        self.0,
                        name.as_ptr(),
                        null(),
                        null_mut(),
                        &mut value as *mut _ as _,
                        &mut len,
                    )
                };

                win32_error(result)?;
                Ok(Value::U64(value))
            }
            REG_SZ | REG_EXPAND_SZ => {
                let mut value = vec![0u16; len as usize / 2];

                let result = unsafe {
                    RegQueryValueExW(
                        self.0,
                        name.as_ptr(),
                        null(),
                        null_mut(),
                        value.as_mut_ptr() as _,
                        &mut len,
                    )
                };

                win32_error(result)?;
                Ok(Value::String(String::from_utf16_lossy(trim(&value))))
            }
            REG_MULTI_SZ => {
                let mut value = vec![0u16; len as usize / 2];

                let result = unsafe {
                    RegQueryValueExW(
                        self.0,
                        name.as_ptr(),
                        null(),
                        null_mut(),
                        value.as_mut_ptr() as _,
                        &mut len,
                    )
                };

                win32_error(result)?;

                Ok(Value::MultiString(
                    trim(&value)
                        .split(|c| *c == 0)
                        .map(String::from_utf16_lossy)
                        .collect(),
                ))
            }
            REG_BINARY => {
                let mut value = vec![0u8; len as usize];

                let result = unsafe {
                    RegQueryValueExW(
                        self.0,
                        name.as_ptr(),
                        null(),
                        null_mut(),
                        value.as_mut_ptr() as _,
                        &mut len,
                    )
                };

                win32_error(result)?;
                Ok(Value::Bytes(value))
            }
            _ => Err(invalid_data()),
        }
    }

    /// Gets the value for the name in the registry key.
    pub fn get_u32<T: AsRef<str>>(&self, name: T) -> Result<u32> {
        if let Value::U32(value) = self.get_value(name)? {
            Ok(value)
        } else {
            Err(invalid_data())
        }
    }

    /// Gets the value for the name in the registry key.
    pub fn get_u64<T: AsRef<str>>(&self, name: T) -> Result<u64> {
        if let Value::U64(value) = self.get_value(name)? {
            Ok(value)
        } else {
            Err(invalid_data())
        }
    }

    /// Gets the value for the name in the registry key.
    pub fn get_string<T: AsRef<str>>(&self, name: T) -> Result<String> {
        if let Value::String(value) = self.get_value(name)? {
            Ok(value)
        } else {
            Err(invalid_data())
        }
    }

    /// Gets the value for the name in the registry key.
    pub fn get_hstring<T: AsRef<str>>(&self, name: T) -> Result<HSTRING> {
        let name = pcwstr(name);
        let mut ty = 0;
        let mut len = 0;

        let result = unsafe {
            RegQueryValueExW(self.0, name.as_ptr(), null(), &mut ty, null_mut(), &mut len)
        };

        win32_error(result)?;

        if !matches!(ty, REG_SZ | REG_EXPAND_SZ) {
            return Err(invalid_data());
        }

        let mut value = HStringBuilder::new(len as usize / 2)?;

        let result = unsafe {
            RegQueryValueExW(
                self.0,
                name.as_ptr(),
                null(),
                null_mut(),
                value.as_mut_ptr() as _,
                &mut len,
            )
        };

        win32_error(result)?;
        value.trim_end();
        Ok(value.into())
    }

    /// Gets the value for the name in the registry key.
    pub fn get_bytes<T: AsRef<str>>(&self, name: T) -> Result<Vec<u8>> {
        let name = pcwstr(name);
        let mut len = 0;

        let result = unsafe {
            RegQueryValueExW(
                self.0,
                name.as_ptr(),
                null(),
                null_mut(),
                null_mut(),
                &mut len,
            )
        };

        win32_error(result)?;
        let mut value = vec![0u8; len as usize];

        let result = unsafe {
            RegQueryValueExW(
                self.0,
                name.as_ptr(),
                null(),
                null_mut(),
                value.as_mut_ptr() as _,
                &mut len,
            )
        };

        win32_error(result).map(|_| value)
    }

    /// Gets the value for the name in the registry key.
    pub fn get_multi_string<T: AsRef<str>>(&self, name: T) -> Result<Vec<String>> {
        if let Value::MultiString(value) = self.get_value(name)? {
            Ok(value)
        } else {
            Err(invalid_data())
        }
    }

    unsafe fn set_value<T: AsRef<str>>(
        &self,
        name: T,
        ty: REG_VALUE_TYPE,
        ptr: *const u8,
        len: usize,
    ) -> Result<()> {
        let result = RegSetValueExW(self.0, pcwstr(name).as_ptr(), 0, ty, ptr, len.try_into()?);

        win32_error(result)
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        unsafe {
            RegCloseKey(self.0);
        }
    }
}
