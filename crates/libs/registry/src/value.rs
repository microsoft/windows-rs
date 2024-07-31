use super::*;

/// A registry value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Value {
    pub(crate) value: ValueBytes,
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
}

impl core::ops::Deref for Value {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.value
    }
}

impl AsRef<[u8]> for Value {
    fn as_ref(&self) -> &[u8] {
        &self.value
    }
}

impl TryFrom<u32> for Value {
    type Error = Error;
    fn try_from(from: u32) -> Result<Self> {
        Ok(Self {
            value: from.to_le_bytes().try_into()?,
            ty: Type::U32,
        })
    }
}

impl TryFrom<Value> for u32 {
    type Error = Error;
    fn try_from(from: Value) -> Result<Self> {
        Ok(from_le_bytes(from.ty, &from)?.try_into()?)
    }
}

impl TryFrom<u64> for Value {
    type Error = Error;
    fn try_from(from: u64) -> Result<Self> {
        Ok(Self {
            value: from.to_le_bytes().try_into()?,
            ty: Type::U64,
        })
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
            Type::String | Type::ExpandString => Ok(Self::from_utf16(from.value.as_wide())?),
            _ => Err(invalid_data()),
        }
    }
}

impl TryFrom<&str> for Value {
    type Error = Error;
    fn try_from(from: &str) -> Result<Self> {
        Ok(Self {
            value: ValueBytes::from_slice(pcwstr(from).as_bytes())?,
            ty: Type::String,
        })
    }
}

impl TryFrom<Value> for Vec<String> {
    type Error = Error;
    fn try_from(from: Value) -> Result<Self> {
        match from.ty {
            Type::MultiString => Ok(from
                .value
                .as_wide()
                .split(|c| *c == 0)
                .map(String::from_utf16_lossy)
                .collect()),
            _ => Ok(vec![String::try_from(from)?]),
        }
    }
}
