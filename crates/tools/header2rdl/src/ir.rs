/// Intermediate representation for C/C++ declarations, used between the Clang AST
/// walker (`collect.rs`) and the RDL text emitter (`emit.rs`).

/// All top-level items produced from a parsed translation unit.
#[derive(Debug)]
pub enum IrItem {
    Struct(IrStruct),
    Enum(IrEnum),
    Function(IrFunction),
    Callback(IrCallback),
    Const(IrConst),
    Interface(IrInterface),
}

/// A struct or union record.
#[derive(Debug)]
pub struct IrStruct {
    pub name: String,
    pub fields: Vec<IrField>,
    pub is_union: bool,
    /// Packing alignment in bytes, if `#pragma pack` or `__attribute__((packed))` applies.
    pub pack: Option<u32>,
}

/// A single field inside a struct or union.
#[derive(Debug)]
pub struct IrField {
    pub name: String,
    pub ty: IrType,
}

/// An enum declaration.
#[derive(Debug)]
pub struct IrEnum {
    pub name: String,
    pub underlying: IrPrimitive,
    /// True when all values appear to be power-of-two flags.
    pub is_flags: bool,
    pub variants: Vec<IrVariant>,
}

#[derive(Debug)]
pub struct IrVariant {
    pub name: String,
    pub value: i64,
}

/// An external function imported from a DLL.
#[derive(Debug)]
pub struct IrFunction {
    pub name: String,
    /// DLL name supplied via `--library` or detected from `__declspec(dllimport)`.
    pub library: String,
    pub abi: IrAbi,
    pub params: Vec<IrParam>,
    pub ret: IrType,
    /// Whether the function is expected to set `GetLastError`.
    pub last_error: bool,
}

/// A function-pointer type (typedef to a function prototype).
#[derive(Debug)]
pub struct IrCallback {
    pub name: String,
    pub abi: IrAbi,
    pub params: Vec<IrParam>,
    pub ret: IrType,
}

/// A COM-style interface (C++ abstract class with vtable).
#[derive(Debug)]
pub struct IrInterface {
    pub name: String,
    /// Optional GUID string, e.g. `"00000000-0000-0000-C000-000000000046"`.
    pub guid: Option<String>,
    /// Optional base interface name.
    pub base: Option<String>,
    pub methods: Vec<IrMethod>,
}

#[derive(Debug)]
pub struct IrMethod {
    pub name: String,
    pub params: Vec<IrParam>,
    pub ret: IrType,
}

/// A named constant (`#define` integer literal or `const` variable).
#[derive(Debug)]
pub struct IrConst {
    pub name: String,
    pub ty: IrPrimitive,
    pub value: ConstValue,
}

#[derive(Debug)]
pub enum ConstValue {
    Int(i64),
    Uint(u64),
    Float(f64),
}

/// A function or method parameter.
#[derive(Debug)]
pub struct IrParam {
    pub name: String,
    pub ty: IrType,
    pub modifiers: Vec<ParamModifier>,
}

/// SAL-derived parameter direction/option attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamModifier {
    In,
    Out,
    Opt,
}

/// Calling convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrAbi {
    /// `stdcall` on x86, `win64` on x64 — the default Windows calling convention.
    /// Emitted as bare `extern fn` (no explicit ABI string).
    System,
    /// `cdecl` — emitted as `extern "C" fn`.
    C,
    /// x86 `__fastcall` — emitted as `extern "fastcall" fn`.
    Fastcall,
}

/// The resolved RDL type for a field, parameter, or return position.
#[derive(Debug, Clone)]
pub enum IrType {
    Void,
    Primitive(IrPrimitive),
    /// A named type that comes from the same translation unit (struct, enum, callback…).
    Named(String),
    Ptr {
        is_const: bool,
        pointee: Box<IrType>,
    },
    /// Fixed-size array `[T; N]` (only meaningful in Win32 structs).
    Array {
        elem: Box<IrType>,
        size: u64,
    },
}

/// Primitive numeric types that RDL supports directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrPrimitive {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    /// Pointer-sized signed integer (`isize`).
    Isize,
    /// Pointer-sized unsigned integer (`usize`).
    Usize,
}

impl IrPrimitive {
    pub fn rdl_name(self) -> &'static str {
        match self {
            IrPrimitive::Bool => "bool",
            IrPrimitive::I8 => "i8",
            IrPrimitive::U8 => "u8",
            IrPrimitive::I16 => "i16",
            IrPrimitive::U16 => "u16",
            IrPrimitive::I32 => "i32",
            IrPrimitive::U32 => "u32",
            IrPrimitive::I64 => "i64",
            IrPrimitive::U64 => "u64",
            IrPrimitive::F32 => "f32",
            IrPrimitive::F64 => "f64",
            IrPrimitive::Isize => "isize",
            IrPrimitive::Usize => "usize",
        }
    }

    /// Whether this is a signed type.
    pub fn is_signed(self) -> bool {
        matches!(
            self,
            IrPrimitive::I8
                | IrPrimitive::I16
                | IrPrimitive::I32
                | IrPrimitive::I64
                | IrPrimitive::Isize
        )
    }
}
