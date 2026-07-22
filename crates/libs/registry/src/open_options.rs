use super::*;

/// Options and flags used to configure how a registry key is opened.
#[derive(Debug)]
pub struct OpenOptions<'a> {
    parent: &'a Key,
    access: u32,
    create: bool,
    transaction: Option<&'a Transaction>,
    options: u32,
}

impl<'a> OpenOptions<'a> {
    pub(crate) fn new(parent: &'a Key) -> Self {
        Self {
            parent,
            access: 0,
            create: false,
            transaction: None,
            options: REG_OPTION_NON_VOLATILE,
        }
    }

    /// Sets the option for read access.
    pub fn read(&mut self) -> &mut Self {
        self.access |= KEY_READ;
        self
    }

    /// Sets the option for write access.
    pub fn write(&mut self) -> &mut Self {
        self.access |= KEY_WRITE;
        self
    }

    /// Sets additional access rights.
    pub fn access(&mut self, access: u32) -> &mut Self {
        self.access |= access;
        self
    }

    /// Sets the option to create a new registry key, or open it if it already exists.
    pub fn create(&mut self) -> &mut Self {
        self.create = true;
        self
    }

    /// Associate the registry key with a transaction.
    pub fn transaction(&mut self, transaction: &'a Transaction) -> &mut Self {
        self.transaction = Some(transaction);
        self
    }

    /// Sets the option to create a volatile registry key that is not preserved when the system restarts.
    pub fn volatile(&mut self) -> &mut Self {
        self.options |= REG_OPTION_VOLATILE;
        self
    }

    /// Accesses the 32-bit view of the registry on a 64-bit system (`KEY_WOW64_32KEY`).
    ///
    /// This lets a 64-bit process open a key under the `WOW6432Node` redirected
    /// view, or a 32-bit process explicitly target that view. Mutually exclusive
    /// with [`wow64_64`](Self::wow64_64); the last call wins.
    pub fn wow64_32(&mut self) -> &mut Self {
        self.access = (self.access & !KEY_WOW64_64KEY) | KEY_WOW64_32KEY;
        self
    }

    /// Accesses the 64-bit view of the registry on a 64-bit system (`KEY_WOW64_64KEY`).
    ///
    /// This lets a 32-bit (WOW64) process bypass registry redirection and open a
    /// key in the native 64-bit view. Mutually exclusive with
    /// [`wow64_32`](Self::wow64_32); the last call wins.
    pub fn wow64_64(&mut self) -> &mut Self {
        self.access = (self.access & !KEY_WOW64_32KEY) | KEY_WOW64_64KEY;
        self
    }

    /// Opens a registry key with the options provided by `self`.
    pub fn open<T: AsRef<str>>(&self, path: T) -> Result<Key> {
        let mut handle = null_mut();

        let result = unsafe {
            if let Some(transaction) = self.transaction {
                if self.create {
                    RegCreateKeyTransactedW(
                        self.parent.0,
                        pcwstr(path).as_ptr(),
                        0,
                        null(),
                        self.options,
                        self.access,
                        null(),
                        &mut handle,
                        null_mut(),
                        transaction.0,
                        null(),
                    )
                } else {
                    RegOpenKeyTransactedW(
                        self.parent.0,
                        pcwstr(path).as_ptr(),
                        0,
                        self.access,
                        &mut handle,
                        transaction.0,
                        null(),
                    )
                }
            } else if self.create {
                RegCreateKeyExW(
                    self.parent.0,
                    pcwstr(path).as_ptr(),
                    0,
                    null(),
                    self.options,
                    self.access,
                    null(),
                    &mut handle,
                    null_mut(),
                )
            } else {
                RegOpenKeyExW(
                    self.parent.0,
                    pcwstr(path).as_ptr(),
                    0,
                    self.access,
                    &mut handle,
                )
            }
        };

        win32_error(result).map(|_| Key(handle))
    }
}
