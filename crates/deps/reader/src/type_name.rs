pub trait HasTypeName: Copy {
    fn namespace(&self) -> &str;
    fn name(&self) -> &str;
}

impl HasTypeName for TypeName {
    fn namespace(&self) -> &str {
        self.namespace
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl HasTypeName for (&str, &str) {
    fn namespace(&self) -> &str {
        self.0
    }

    fn name(&self) -> &str {
        self.1
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeName {
    pub namespace: &'static str,
    pub name: &'static str,
}

#[allow(non_upper_case_globals)]
impl TypeName {
    pub const None: Self = Self::from_const("", "");
    pub const Enum: Self = Self::from_const("System", "Enum");
    pub const Delegate: Self = Self::from_const("System", "MulticastDelegate");
    pub const Struct: Self = Self::from_const("System", "ValueType");
    pub const Object: Self = Self::from_const("System", "Object");
    pub const GUID: Self = Self::from_const("System", "Guid");
    pub const Type: Self = Self::from_const("System", "Type");
    pub const Attribute: Self = Self::from_const("System", "Attribute");
    pub const IsConst: Self = Self::from_const("System.Runtime.CompilerServices", "IsConst");

    pub const TimeSpan: Self = Self::from_const("Windows.Foundation", "TimeSpan");
    pub const HResult: Self = Self::from_const("Windows.Foundation", "HResult");
    pub const IAsyncAction: Self = Self::from_const("Windows.Foundation", "IAsyncAction");
    pub const IAsyncActionWithProgress: Self = Self::from_const("Windows.Foundation", "IAsyncActionWithProgress");
    pub const IAsyncOperation: Self = Self::from_const("Windows.Foundation", "IAsyncOperation");
    pub const IAsyncOperationWithProgress: Self = Self::from_const("Windows.Foundation", "IAsyncOperationWithProgress");

    pub const Vector2: Self = Self::from_const("Windows.Foundation.Numerics", "Vector2");
    pub const Vector3: Self = Self::from_const("Windows.Foundation.Numerics", "Vector3");
    pub const Vector4: Self = Self::from_const("Windows.Foundation.Numerics", "Vector4");
    pub const Matrix3x2: Self = Self::from_const("Windows.Foundation.Numerics", "Matrix3x2");
    pub const Matrix4x4: Self = Self::from_const("Windows.Foundation.Numerics", "Matrix4x4");

    pub const IIterable: Self = Self::from_const("Windows.Foundation.Collections", "IIterable");
    pub const IIterator: Self = Self::from_const("Windows.Foundation.Collections", "IIterator");
    pub const IVectorView: Self = Self::from_const("Windows.Foundation.Collections", "IVectorView");
    pub const IVector: Self = Self::from_const("Windows.Foundation.Collections", "IVector");

    pub const WIN32_ERROR: Self = Self::from_const("Windows.Win32.Foundation", "WIN32_ERROR");
    pub const NTSTATUS: Self = Self::from_const("Windows.Win32.Foundation", "NTSTATUS");
    pub const BOOL: Self = Self::from_const("Windows.Win32.Foundation", "BOOL");
    pub const PWSTR: Self = Self::from_const("Windows.Win32.Foundation", "PWSTR");
    pub const PSTR: Self = Self::from_const("Windows.Win32.Foundation", "PSTR");
    pub const BSTR: Self = Self::from_const("Windows.Win32.Foundation", "BSTR");
    pub const HANDLE: Self = Self::from_const("Windows.Win32.Foundation", "HANDLE");
    pub const HRESULT: Self = Self::from_const("Windows.Win32.Foundation", "HRESULT");
    pub const SysStringLen: Self = Self::from_const("Windows.Win32.Foundation", "SysStringLen");
    pub const SysAllocStringLen: Self = Self::from_const("Windows.Win32.Foundation", "SysAllocStringLen");
    pub const SysFreeString: Self = Self::from_const("Windows.Win32.Foundation", "SysFreeString");
    pub const D2D_MATRIX_3X2_F: Self = Self::from_const("Windows.Win32.Graphics.Direct2D.Common", "D2D_MATRIX_3X2_F");
    pub const IUnknown: Self = Self::from_const("Windows.Win32.System.Com", "IUnknown");
    pub const HSTRING: Self = Self::from_const("Windows.Win32.System.WinRT", "HSTRING");
    pub const IInspectable: Self = Self::from_const("Windows.Win32.System.WinRT", "IInspectable");
    pub const LARGE_INTEGER: Self = Self::from_const("Windows.Win32.Foundation", "LARGE_INTEGER");
    pub const ULARGE_INTEGER: Self = Self::from_const("Windows.Win32.Foundation", "ULARGE_INTEGER");
    pub const IRestrictedErrorInfo: Self = Self::from_const("Windows.Win32.System.WinRT", "IRestrictedErrorInfo");
    pub const IDispatch: Self = Self::from_const("Windows.Win32.System.Com", "IDispatch");

    const fn from_const(namespace: &'static str, name: &'static str) -> Self {
        Self { namespace, name }
    }

    pub fn new(namespace: &'static str, name: &'static str) -> Self {
        Self { namespace, name: trim_tick(name) }
    }

    pub fn parse(full_name: &str) -> (&str, &str) {
        let index = full_name.rfind('.').expect("Expected full name separated with `.`");
        (&full_name[0..index], &full_name[index + 1..])
    }
}

fn trim_tick(name: &'static str) -> &'static str {
    let len = name.len() - 2;
    match name.as_bytes().get(len) {
        Some(c) if *c == b'`' => &name[..len],
        _ => name,
    }
}

impl core::fmt::Display for TypeName {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(fmt, "{}.{}", self.namespace, self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let reader = TypeReader::get();
        reader.get_type_entry(TypeName::WIN32_ERROR).unwrap(); // TODO: remove
        reader.get_type_entry(TypeName::NTSTATUS).unwrap();
        reader.get_type_entry(TypeName::BOOL).unwrap();
        reader.get_type_entry(TypeName::PWSTR).unwrap();
        reader.get_type_entry(TypeName::PSTR).unwrap();
        reader.get_type_entry(TypeName::BSTR).unwrap();
        reader.get_type_entry(TypeName::HANDLE).unwrap();
        reader.get_type_entry(TypeName::SysStringLen).unwrap();
        reader.get_type_entry(TypeName::SysAllocStringLen).unwrap();
        reader.get_type_entry(TypeName::SysFreeString).unwrap();
        reader.get_type_entry(TypeName::IRestrictedErrorInfo).unwrap();
        reader.get_type_entry(TypeName::IDispatch).unwrap();
    }
}
