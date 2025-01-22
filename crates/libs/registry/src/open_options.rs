use super::*;

/// Options and flags used to configure how a registry key is opened.
pub struct OpenOptions<'a> {
    parent: &'a Key,
    read: bool,
    write: bool,
    create: bool,
    transaction: Option<&'a Transaction>,
}

impl<'a> OpenOptions<'a> {
    pub(crate) fn new(parent: &'a Key) -> Self {
        Self {
            parent,
            read: false,
            write: false,
            create: false,
            transaction: None,
        }
    }

    /// Sets the option for read access.
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    /// Sets the option for write access.
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    /// Sets the option to create a new registry key, or open it if it already exists.
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    /// Associate the registry key with a transaction.
    pub fn transaction(&mut self, transaction: &'a Transaction) -> &mut Self {
        self.transaction = Some(transaction);
        self
    }

    /// Opens a registry key with the options provided by `self`.
    pub fn open<T: AsRef<str>>(&self, path: T) -> Result<Key> {
        let mut flags = 0;

        if self.read {
            flags |= KEY_READ;
        }

        if self.write {
            flags |= KEY_WRITE;
        }

        let mut handle = null_mut();

        let result = unsafe {
            if let Some(transaction) = self.transaction {
                if self.create {
                    RegCreateKeyTransactedW(
                        self.parent.0,
                        pcwstr(path).as_ptr(),
                        0,
                        null(),
                        REG_OPTION_NON_VOLATILE,
                        flags,
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
                        flags,
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
                    REG_OPTION_NON_VOLATILE,
                    flags,
                    null(),
                    &mut handle,
                    null_mut(),
                )
            } else {
                RegOpenKeyExW(self.parent.0, pcwstr(path).as_ptr(), 0, flags, &mut handle)
            }
        };

        win32_error(result).map(|_| Key(handle))
    }
}
