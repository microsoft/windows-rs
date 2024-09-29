use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Void,
    Bool,
    Char,
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
    ISize,
    USize,
    String,
    Object,

    // TODO: add other "core" types like BSTR/HRESULT/GUID/IUnknown?
    PSTR,
    PCSTR,
    PWSTR,
    PCWSTR,

    Item(&'static Item),
    Generic(&'static Item, Vec<Self>),
    Param(&'static str),
    PtrMut(Box<Self>, usize),
    PtrConst(Box<Self>, usize),
    ArrayFixed(Box<Self>, usize),
    Array(Box<Self>),
    ArrayRef(Box<Self>),
    ConstRef(Box<Self>),
    PrimitiveOrEnum(Box<Self>, Box<Self>),
}

impl Type {
    pub fn from_element_type(code: usize) -> Option<Self> {
        match code as u8 {
            ELEMENT_TYPE_VOID => Some(Self::Void),
            ELEMENT_TYPE_BOOLEAN => Some(Self::Bool),
            ELEMENT_TYPE_CHAR => Some(Self::Char),
            ELEMENT_TYPE_I1 => Some(Self::I8),
            ELEMENT_TYPE_U1 => Some(Self::U8),
            ELEMENT_TYPE_I2 => Some(Self::I16),
            ELEMENT_TYPE_U2 => Some(Self::U16),
            ELEMENT_TYPE_I4 => Some(Self::I32),
            ELEMENT_TYPE_U4 => Some(Self::U32),
            ELEMENT_TYPE_I8 => Some(Self::I64),
            ELEMENT_TYPE_U8 => Some(Self::U64),
            ELEMENT_TYPE_R4 => Some(Self::F32),
            ELEMENT_TYPE_R8 => Some(Self::F64),
            ELEMENT_TYPE_I => Some(Self::ISize),
            ELEMENT_TYPE_U => Some(Self::USize),
            ELEMENT_TYPE_STRING => Some(Self::String),
            ELEMENT_TYPE_OBJECT => Some(Self::Object),
            _ => None,
        }
    }

    pub fn from_ref(
        code: TypeDefOrRef,
        enclosing: Option<&'static CppStruct>,
        generics: &[Self],
    ) -> Self {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.blob(0);
            return Self::from_blob_impl(&mut blob, None, generics);
        }

        let full_name = code.type_name();

        // TODO: remapping happens here

        // TODO: this needs to be deferred via a TypeName's optional nested type name?
        if let Some(outer) = enclosing {
            if full_name.namespace().is_empty() {
                return Type::Item(&outer.nested[full_name.name()]);
            }
        }

        if let Some(item) = code
            .reader()
            .with_full_name(full_name.namespace(), full_name.name())
            .next()
        {
            Type::Item(item)
        } else {
            todo!()
        }
    }

    pub fn from_blob(
        blob: &mut Blob,
        enclosing: Option<&'static CppStruct>,
        generics: &[Type],
    ) -> Self {
        // Used by WinRT to indicate that a struct input parameter is passed by reference rather than by value on the ABI.
        let is_const = blob
            .read_modifiers()
            .iter()
            .any(|def| def.type_name() == TypeName::IsConst);

        // Used by WinRT to indicate an output parameter, but there are other ways to determine this direction so here
        // it is only used to distinguish between slices and heap-allocated arrays.
        let is_ref = blob.try_read(ELEMENT_TYPE_BYREF as usize);

        if blob.try_read(ELEMENT_TYPE_VOID as usize) {
            return Type::Void;
        }

        let is_array = blob.try_read(ELEMENT_TYPE_SZARRAY as usize); // Used by WinRT to indicate an array

        let mut pointers = 0;

        while blob.try_read(ELEMENT_TYPE_PTR as usize) {
            pointers += 1;
        }

        let kind = Self::from_blob_impl(blob, enclosing, generics);

        if pointers > 0 {
            Type::PtrMut(Box::new(kind), pointers)
        } else if is_const {
            Type::ConstRef(Box::new(kind))
        } else if is_array {
            if is_ref {
                Type::ArrayRef(Box::new(kind))
            } else {
                Type::Array(Box::new(kind))
            }
        } else {
            kind
        }
    }

    fn from_blob_impl(
        blob: &mut Blob,
        enclosing: Option<&'static CppStruct>,
        generics: &[Type],
    ) -> Self {
        let code = blob.read_usize();

        if let Some(code) = Type::from_element_type(code) {
            return code;
        }

        match code as u8 {
            ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS => {
                Self::from_ref(blob.decode(), enclosing, generics)
            }
            ELEMENT_TYPE_VAR => generics
                .get(blob.read_usize())
                .unwrap_or(&Type::Void)
                .clone(),
            ELEMENT_TYPE_ARRAY => {
                let kind = Self::from_blob(blob, enclosing, generics);
                let _rank = blob.read_usize();
                let _count = blob.read_usize();
                let bounds = blob.read_usize();
                Type::ArrayFixed(Box::new(kind), bounds)
            }
            ELEMENT_TYPE_GENERICINST => {
                blob.read_usize(); // ELEMENT_TYPE_VALUETYPE or ELEMENT_TYPE_CLASS

                let type_name = blob.decode::<TypeDefOrRef>().type_name();
                let def = blob
                    .reader()
                    .with_full_name(type_name.namespace(), type_name.name())
                    .next()
                    .unwrap_or_else(|| panic!("Type not found: {}", type_name));
                let mut args = Vec::with_capacity(blob.read_usize());

                for _ in 0..args.capacity() {
                    args.push(Self::from_blob_impl(blob, enclosing, generics));
                }

                Type::Generic(def, args)
            }
            rest => unimplemented!("{rest:?}"),
        }
    }

    pub fn to_const_type(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrMut(Box::new(ty.to_const_type()), *pointers),
            Self::PtrConst(ty, pointers) => Self::PtrConst(Box::new(ty.to_const_type()), *pointers),
            Self::PSTR => Self::PCSTR,
            Self::PWSTR => Self::PCWSTR,
            _ => self.clone(),
        }
    }

    pub fn to_const_ptr(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrConst(ty.clone(), *pointers),
            _ => self.clone(),
        }
    }

    pub fn deref(self) -> Self {
        match self {
            Self::PtrConst(ty, 1) | Self::PtrMut(ty, 1) => {
                if *ty == Self::Void {
                    Self::U8
                } else {
                    *ty
                }
            }
            Self::PtrConst(ty, pointers) => Self::PtrConst(ty.clone(), pointers - 1),
            Self::PtrMut(ty, pointers) => Self::PtrMut(ty.clone(), pointers - 1),
            Self::PSTR | Self::PCSTR => Self::U8,
            Self::PWSTR | Self::PCWSTR => Self::U16,
            rest => unimplemented!("{rest:?}"),
        }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        // First get the underlying type.
        let ty = match self {
            Self::PtrMut(ty, _) => ty,
            Self::PtrConst(ty, _) => ty,
            Self::ArrayFixed(ty, _) => ty,
            Self::Array(ty) => ty,
            Self::ArrayRef(ty) => ty,
            Self::ConstRef(ty) => ty,
            Self::PrimitiveOrEnum(_, ty) => ty,
            _ => self,
        };

        // Then insert its name into the dependencies map.
        match ty {
            Self::String => {
                dependencies.insert("System", "String");
            }
            Self::Object => {
                dependencies.insert("System", "Object");
            }
            Self::PSTR => {
                dependencies.insert("System", "PSTR");
            }
            Self::PCSTR => {
                dependencies.insert("System", "PCSTR");
            }
            Self::PWSTR => {
                dependencies.insert("System", "PWSTR");
            }
            Self::PCWSTR => {
                dependencies.insert("System", "PCWSTR");
            }
            Self::Item(item) => {
                // Only chase dependencies if it was not previously added.
                if dependencies.insert(item.namespace(), item.name()) {
                    item.dependencies(dependencies);
                }
            }
            Self::Generic(item, generics) => {
                // Only chase dependencies if it was not previously added.
                if dependencies.insert(item.namespace(), item.name()) {
                    item.dependencies(dependencies);
                }

                generics.iter().for_each(|ty| ty.dependencies(dependencies));
            }
            _ => {}
        }
    }
}
