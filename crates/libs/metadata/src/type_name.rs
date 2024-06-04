#![allow(non_upper_case_globals)]

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TypeName(pub &'static str, pub &'static str);

impl TypeName {
    pub const Enum: Self = Self("System", "Enum");
    pub const Delegate: Self = Self("System", "MulticastDelegate");
    pub const Struct: Self = Self("System", "ValueType");
    pub const Object: Self = Self("System", "Object");
    pub const GUID: Self = Self("System", "Guid");
    pub const Type: Self = Self("System", "Type");
    pub const Attribute: Self = Self("System", "Attribute");
    pub const IsConst: Self = Self("System.Runtime.CompilerServices", "IsConst");

    pub const HResult: Self = Self("Windows.Foundation", "HResult");
    pub const IAsyncAction: Self = Self("Windows.Foundation", "IAsyncAction");
    pub const IAsyncActionWithProgress: Self =
        Self("Windows.Foundation", "IAsyncActionWithProgress");
    pub const IAsyncOperation: Self = Self("Windows.Foundation", "IAsyncOperation");
    pub const IAsyncOperationWithProgress: Self =
        Self("Windows.Foundation", "IAsyncOperationWithProgress");

    pub const Matrix3x2: Self = Self("Windows.Foundation.Numerics", "Matrix3x2");
    pub const Matrix4x4: Self = Self("Windows.Foundation.Numerics", "Matrix4x4");

    pub const IIterable: Self = Self("Windows.Foundation.Collections", "IIterable");
    pub const IIterator: Self = Self("Windows.Foundation.Collections", "IIterator");
    pub const IVectorView: Self = Self("Windows.Foundation.Collections", "IVectorView");
    pub const IVector: Self = Self("Windows.Foundation.Collections", "IVector");

    pub const PWSTR: Self = Self("Windows.Win32.Foundation", "PWSTR");
    pub const PSTR: Self = Self("Windows.Win32.Foundation", "PSTR");
    pub const BSTR: Self = Self("Windows.Win32.Foundation", "BSTR");
    pub const HANDLE: Self = Self("Windows.Win32.Foundation", "HANDLE");
    pub const HRESULT: Self = Self("Windows.Win32.Foundation", "HRESULT");
    pub const CHAR: Self = Self("Windows.Win32.Foundation", "CHAR");
    pub const BOOL: Self = Self("Windows.Win32.Foundation", "BOOL");
    pub const WIN32_ERROR: Self = Self("Windows.Win32.Foundation", "WIN32_ERROR");
    pub const NTSTATUS: Self = Self("Windows.Win32.Foundation", "NTSTATUS");
    pub const RPC_STATUS: Self = Self("Windows.Win32.System.Rpc", "RPC_STATUS");

    pub const D2D_MATRIX_3X2_F: Self =
        Self("Windows.Win32.Graphics.Direct2D.Common", "D2D_MATRIX_3X2_F");
    pub const D3DMATRIX: Self = Self("Windows.Win32.Graphics.Direct3D", "D3DMATRIX");
    pub const IUnknown: Self = Self("Windows.Win32.System.Com", "IUnknown");
    pub const HSTRING: Self = Self("Windows.Win32.System.WinRT", "HSTRING");
    pub const IInspectable: Self = Self("Windows.Win32.System.WinRT", "IInspectable");
    pub const IRestrictedErrorInfo: Self =
        Self("Windows.Win32.System.WinRT", "IRestrictedErrorInfo");
    pub const IDispatch: Self = Self("Windows.Win32.System.Com", "IDispatch");

    pub const VARIANT: Self = Self("Windows.Win32.System.Variant", "VARIANT");
    pub const PROPVARIANT: Self = Self("Windows.Win32.System.Com.StructuredStorage", "PROPVARIANT");

    pub fn parse(full_name: &'static str) -> Self {
        let index = full_name
            .rfind('.')
            .expect("Expected full name separated with `.`");
        Self(&full_name[0..index], &full_name[index + 1..])
    }

    pub fn namespace(&self) -> &'static str {
        self.0
    }

    pub fn name(&self) -> &'static str {
        self.1
    }
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}.{}", self.0, self.1)
    }
}
