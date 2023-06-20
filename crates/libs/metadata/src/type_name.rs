#![allow(non_upper_case_globals)]

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeName<'a> {
    pub namespace: &'a str,
    pub name: &'a str,
}

impl<'a> TypeName<'a> {
    pub const Enum: Self = Self::from_const("System", "Enum");
    pub const Delegate: Self = Self::from_const("System", "MulticastDelegate");
    pub const Struct: Self = Self::from_const("System", "ValueType");
    pub const Object: Self = Self::from_const("System", "Object");
    pub const GUID: Self = Self::from_const("System", "Guid");
    pub const Type: Self = Self::from_const("System", "Type");
    pub const Attribute: Self = Self::from_const("System", "Attribute");
    pub const IsConst: Self = Self::from_const("System.Runtime.CompilerServices", "IsConst");

    pub const HResult: Self = Self::from_const("Windows.Foundation", "HResult");
    pub const IAsyncAction: Self = Self::from_const("Windows.Foundation", "IAsyncAction");
    pub const IAsyncActionWithProgress: Self = Self::from_const("Windows.Foundation", "IAsyncActionWithProgress");
    pub const IAsyncOperation: Self = Self::from_const("Windows.Foundation", "IAsyncOperation");
    pub const IAsyncOperationWithProgress: Self = Self::from_const("Windows.Foundation", "IAsyncOperationWithProgress");

    pub const Matrix3x2: Self = Self::from_const("Windows.Foundation.Numerics", "Matrix3x2");
    pub const Matrix4x4: Self = Self::from_const("Windows.Foundation.Numerics", "Matrix4x4");

    pub const IIterable: Self = Self::from_const("Windows.Foundation.Collections", "IIterable");
    pub const IIterator: Self = Self::from_const("Windows.Foundation.Collections", "IIterator");
    pub const IVectorView: Self = Self::from_const("Windows.Foundation.Collections", "IVectorView");
    pub const IVector: Self = Self::from_const("Windows.Foundation.Collections", "IVector");

    pub const NTSTATUS: Self = Self::from_const("Windows.Win32.Foundation", "NTSTATUS");
    pub const PWSTR: Self = Self::from_const("Windows.Win32.Foundation", "PWSTR");
    pub const PSTR: Self = Self::from_const("Windows.Win32.Foundation", "PSTR");
    pub const BSTR: Self = Self::from_const("Windows.Win32.Foundation", "BSTR");
    pub const HANDLE: Self = Self::from_const("Windows.Win32.Foundation", "HANDLE");
    pub const HRESULT: Self = Self::from_const("Windows.Win32.Foundation", "HRESULT");
    pub const CHAR: Self = Self::from_const("Windows.Win32.Foundation", "CHAR");
    pub const BOOL: Self = Self::from_const("Windows.Win32.Foundation", "BOOL");
    pub const WIN32_ERROR: Self = Self::from_const("Windows.Win32.Foundation", "WIN32_ERROR");

    pub const D2D_MATRIX_3X2_F: Self = Self::from_const("Windows.Win32.Graphics.Direct2D.Common", "D2D_MATRIX_3X2_F");
    pub const D3DMATRIX: Self = Self::from_const("Windows.Win32.Graphics.Direct3D", "D3DMATRIX");
    pub const IUnknown: Self = Self::from_const("Windows.Win32.System.Com", "IUnknown");
    pub const HSTRING: Self = Self::from_const("Windows.Win32.System.WinRT", "HSTRING");
    pub const IInspectable: Self = Self::from_const("Windows.Win32.System.WinRT", "IInspectable");
    pub const IRestrictedErrorInfo: Self = Self::from_const("Windows.Win32.System.WinRT", "IRestrictedErrorInfo");
    pub const IDispatch: Self = Self::from_const("Windows.Win32.System.Com", "IDispatch");

    const fn from_const(namespace: &'static str, name: &'static str) -> Self {
        Self { namespace, name }
    }

    pub fn new(namespace: &'a str, name: &'a str) -> Self {
        Self { namespace, name: crate::trim_tick(name) }
    }

    pub fn parse(full_name: &'a str) -> Self {
        let index = full_name.rfind('.').expect("Expected full name separated with `.`");
        Self { namespace: &full_name[0..index], name: &full_name[index + 1..] }
    }
}

impl<'a> std::fmt::Display for TypeName<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}.{}", self.namespace, self.name)
    }
}
