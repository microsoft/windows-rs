use super::*;

/// A registry value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Value {
    pub(crate) data: Data,
    pub(crate) ty: Type,
}

impl Value {
    /// Gets the type of the registry value.
    pub fn ty(&self) -> Type {
        self.ty
    }

    /// Sets the type of the registry value. This does not change the value.
    pub fn set_ty(&mut self, ty: Type) {
        self.ty = ty;
    }

    /// Gets the value as a slice of u16 for raw wide characters.
    pub fn as_wide(&self) -> &[u16] {
        self.data.as_wide()
    }
}

impl core::ops::Deref for Value {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<[u8]> for Value {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl From<u32> for Value {
    fn from(from: u32) -> Self {
        Self {
            data: from.to_le_bytes().into(),
            ty: Type::U32,
        }
    }
}

impl TryFrom<Value> for u32 {
    type Error = Error;
    fn try_from(from: Value) -> Result<Self> {
        Ok(from_le_bytes(from.ty, &from)?.try_into()?)
    }
}

impl From<u64> for Value {
    fn from(from: u64) -> Self {
        Self {
            data: from.to_le_bytes().into(),
            ty: Type::U64,
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = Error;
    fn try_from(from: Value) -> Result<Self> {
        from_le_bytes(from.ty, &from)
    }
}

impl TryFrom<Value> for String {
    type Error = Error;
    fn try_from(from: Value) -> Result<Self> {
        match from.ty {
            Type::String | Type::ExpandString => Ok(Self::from_utf16(trim(from.data.as_wide()))?),
            _ => Err(invalid_data()),
        }
    }
}

impl From<&str> for Value {
    fn from(from: &str) -> Self {
        Self {
            data: Data::from_slice(pcwstr(from).as_bytes()),
            ty: Type::String,
        }
    }
}

impl TryFrom<Value> for Vec<String> {
    type Error = Error;
    fn try_from(from: Value) -> Result<Self> {
        match from.ty {
            Type::MultiString => {
                let wide = from.data.as_wide();
                // REG_MULTI_SZ must end with a null terminator. A non-empty list
                // requires a double-null (each string is null-terminated, plus
                // the final list terminator). An empty list is just a single null.
                if wide.is_empty() || wide[wide.len() - 1] != 0 {
                    return Err(invalid_data());
                }
                // If there's content before the final null, the second-to-last
                // must also be null (the double-null terminator).
                if wide.len() >= 2 && wide[wide.len() - 2] != 0 {
                    return Err(invalid_data());
                }
                wide.split(|c| *c == 0)
                    .take_while(|s| !s.is_empty())
                    .map(|s| String::from_utf16(s).map_err(|_| invalid_data()))
                    .collect()
            }
            _ => Ok(vec![String::try_from(from)?]),
        }
    }
}

impl TryFrom<Value> for HSTRING {
    type Error = Error;
    fn try_from(from: Value) -> Result<Self> {
        match from.ty {
            Type::String | Type::ExpandString => Ok(Self::from_wide(trim(from.data.as_wide()))),
            _ => Err(invalid_data()),
        }
    }
}

impl From<&HSTRING> for Value {
    fn from(from: &HSTRING) -> Self {
        Self {
            data: Data::from_slice(as_bytes(from)),
            ty: Type::String,
        }
    }
}

impl From<&[u8]> for Value {
    fn from(from: &[u8]) -> Self {
        Self {
            data: Data::from_slice(from),
            ty: Type::Bytes,
        }
    }
}

impl<const N: usize> From<[u8; N]> for Value {
    fn from(from: [u8; N]) -> Self {
        Self {
            data: Data::from_slice(&from),
            ty: Type::Bytes,
        }
    }
}

fn trim(mut wide: &[u16]) -> &[u16] {
    while wide.last() == Some(&0) {
        wide = &wide[..wide.len() - 1];
    }

    wide
}
