#![allow(non_upper_case_globals)]

#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TypeName {
    namespace: &'static str,
    name: &'static str,
}

impl TypeName {
    pub const Enum: Self = Self::new("System", "Enum");
    pub const Delegate: Self = Self::new("System", "MulticastDelegate");
    pub const Struct: Self = Self::new("System", "ValueType");
    pub const Object: Self = Self::new("System", "Object");
    pub const GUID: Self = Self::new("System", "Guid");
    pub const Type: Self = Self::new("System", "Type");
    pub const Attribute: Self = Self::new("System", "Attribute");
    pub const IsConst: Self = Self::new("System.Runtime.CompilerServices", "IsConst");

    pub const HResult: Self = Self::new("Windows.Foundation", "HResult");
    pub const IAsyncAction: Self = Self::new("Windows.Foundation", "IAsyncAction");
    pub const IAsyncActionWithProgress: Self = Self::new("Windows.Foundation", "IAsyncActionWithProgress");
    pub const IAsyncOperation: Self = Self::new("Windows.Foundation", "IAsyncOperation");
    pub const IAsyncOperationWithProgress: Self = Self::new("Windows.Foundation", "IAsyncOperationWithProgress");

    pub const Matrix3x2: Self = Self::new("Windows.Foundation.Numerics", "Matrix3x2");
    pub const Matrix4x4: Self = Self::new("Windows.Foundation.Numerics", "Matrix4x4");

    pub const IIterable: Self = Self::new("Windows.Foundation.Collections", "IIterable");
    pub const IIterator: Self = Self::new("Windows.Foundation.Collections", "IIterator");
    pub const IVectorView: Self = Self::new("Windows.Foundation.Collections", "IVectorView");
    pub const IVector: Self = Self::new("Windows.Foundation.Collections", "IVector");

    pub const PWSTR: Self = Self::new("Windows.Win32.Foundation", "PWSTR");
    pub const PSTR: Self = Self::new("Windows.Win32.Foundation", "PSTR");
    pub const BSTR: Self = Self::new("Windows.Win32.Foundation", "BSTR");
    pub const HANDLE: Self = Self::new("Windows.Win32.Foundation", "HANDLE");
    pub const HRESULT: Self = Self::new("Windows.Win32.Foundation", "HRESULT");
    pub const CHAR: Self = Self::new("Windows.Win32.Foundation", "CHAR");
    pub const BOOL: Self = Self::new("Windows.Win32.Foundation", "BOOL");
    pub const WIN32_ERROR: Self = Self::new("Windows.Win32.Foundation", "WIN32_ERROR");
    pub const NTSTATUS: Self = Self::new("Windows.Win32.Foundation", "NTSTATUS");
    pub const RPC_STATUS: Self = Self::new("Windows.Win32.System.Rpc", "RPC_STATUS");

    pub const D2D_MATRIX_3X2_F: Self = Self::new("Windows.Win32.Graphics.Direct2D.Common", "D2D_MATRIX_3X2_F");
    pub const D3DMATRIX: Self = Self::new("Windows.Win32.Graphics.Direct3D", "D3DMATRIX");
    pub const IUnknown: Self = Self::new("Windows.Win32.System.Com", "IUnknown");
    pub const HSTRING: Self = Self::new("Windows.Win32.System.WinRT", "HSTRING");
    pub const IInspectable: Self = Self::new("Windows.Win32.System.WinRT", "IInspectable");
    pub const IRestrictedErrorInfo: Self = Self::new("Windows.Win32.System.WinRT", "IRestrictedErrorInfo");
    pub const IDispatch: Self = Self::new("Windows.Win32.System.Com", "IDispatch");

    pub const VARIANT: Self = Self::new("Windows.Win32.System.Variant", "VARIANT");
    pub const PROPVARIANT: Self = Self::new("Windows.Win32.System.Com.StructuredStorage", "PROPVARIANT");

    pub const fn new(namespace: &'static str, name: &'static str) -> Self {
        Self { namespace, name }
    }

    pub fn parse(full_name: &'static str) -> Self {
        let index = full_name.rfind('.').expect("Expected full name separated with `.`");
        Self::new(&full_name[0..index], &full_name[index + 1..])
    }

    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}.{}", self.namespace, self.name)
    }
}
