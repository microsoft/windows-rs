use super::*;

mod class;
mod cpp_const;
mod cpp_delegate;
mod cpp_enum;
mod cpp_fn;
mod cpp_interface;
mod cpp_method;
mod cpp_struct;
mod delegate;
mod r#enum;
mod interface;
mod method;
mod r#struct;

pub use class::*;
pub use cpp_const::*;
pub use cpp_delegate::*;
pub use cpp_enum::*;
pub use cpp_fn::*;
pub use cpp_interface::*;
pub use cpp_method::*;
pub use cpp_struct::*;
pub use delegate::*;
pub use interface::*;
pub use method::*;
pub use r#enum::*;
pub use r#struct::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Item {
    Interface(Interface),
    Class(Class),
    Delegate(Delegate),
    Enum(Enum),
    Struct(Struct),

    CppFn(CppFn),
    CppInterface(CppInterface),
    CppConst(CppConst),
    CppEnum(CppEnum),
    CppStruct(CppStruct),
    CppDelegate(CppDelegate),
    // TODO: have psuedo items for the core types like PWSTR so that those can be written out for standalone code gen?
}

// TODO: maybe just order on Item directly
// 1. order functions first
// 2. order everything else by name
// Otherwise it looks weird when you have things like LOAD_LIBRARY_FLAGS sorting before BOOL

// impl Ord for Item {
//     fn cmp(&self, other: &Self) -> Ordering {
//     }
// }

// impl PartialOrd for Item {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

impl Item {
    pub fn expand(&mut self, filter: &NameTree) {
        match self {
            Self::Interface(item) => item.expand(filter),
            Self::CppInterface(item) => item.expand(filter),
            Self::Class(item) => item.expand(filter),
            _ => {}
        }
    }

    pub fn write(&self, writer: &Writer) -> TokenStream {
        match self {
            Self::Struct(item) => item.write(writer),
            Self::Enum(item) => item.write(writer),
            Self::Interface(item) => item.write(writer),
            Self::CppStruct(item) => item.write(writer),
            Self::CppEnum(item) => item.write(writer),
            Self::CppFn(item) => item.write(writer),
            Self::CppConst(item) => item.write(writer),
            Self::CppDelegate(item) => item.write(writer),
            Self::Delegate(item) => item.write(writer),
            Self::Class(item) => item.write(writer),
            Self::CppInterface(item) => item.write(writer),
        }
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        match self {
            Self::CppInterface(_) if writer.config.sys => quote! { *mut core::ffi::c_void },
            Self::Interface(item) => item.write_name(writer),
            Self::Delegate(item) => item.write_name(writer),
            _ => {
                let name = to_ident(self.name());
                let namespace = writer.write_namespace(self.namespace());
                quote! { #namespace #name }
            }
        }
    }

    pub fn generics(&self) -> &[Type] {
        match self {
            Self::Class(item) => &item.generics,
            Self::Interface(item) => &item.generics,
            Self::Delegate(item) => &item.generics,
            _ => &[],
        }
    }

    pub fn set_generics(&mut self, generics: Vec<Type>) {
        match self {
            Self::Class(item) => item.generics = generics,
            Self::Interface(item) => item.generics = generics,
            Self::Delegate(item) => item.generics = generics,
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn namespace(&self) -> &'static str {
        match self {
            Self::Class(item) => item.def.namespace(),
            Self::Delegate(item) => item.def.namespace(),
            Self::Enum(item) => item.def.namespace(),
            Self::Interface(item) => item.def.namespace(),
            Self::Struct(item) => item.def.namespace(),
            Self::CppDelegate(item) => item.def.namespace(),
            Self::CppEnum(item) => item.def.namespace(),
            Self::CppInterface(item) => item.def.namespace(),
            Self::CppStruct(item) => item.def.namespace(),
            Self::CppConst(item) => item.def.namespace(),
            Self::CppFn(item) => item.def.namespace(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Class(item) => item.def.name(),
            Self::Delegate(item) => item.def.name(),
            Self::Enum(item) => item.def.name(),
            Self::Interface(item) => item.def.name(),
            Self::Struct(item) => item.def.name(),
            Self::CppDelegate(item) => item.def.name(),
            Self::CppEnum(item) => item.def.name(),
            Self::CppInterface(item) => item.def.name(),
            Self::CppStruct(item) => item.name(),
            Self::CppConst(item) => item.field.name(),
            Self::CppFn(item) => item.method.name(),
        }
    }

    pub fn underlying_type(&self) -> Type {
        match self {
            Self::Struct(item) => item.def.underlying_type(),
            Self::CppEnum(item) => item.def.underlying_type(),
            Self::Enum(item) => item.def.underlying_type(),
            Self::CppStruct(item) => item.def.underlying_type(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn is_nullable(&self) -> bool {
        matches!(
            self,
            Self::Class(_) | Self::Interface(_) | Self::Delegate(_) | Self::CppInterface(_)
        )
    }

    pub fn is_copyable(&self) -> bool {
        match self {
            Self::Struct(item) => item.is_copyable(),
            Self::CppStruct(item) => item.is_copyable(),
            Self::Enum(_) => true,
            Self::CppEnum(_) => true,
            Self::CppDelegate(_) => true,
            _ => false,
        }
    }

    pub fn is_dropped(&self) -> bool {
        match self {
            Self::Struct(item) => !item.is_copyable(),
            Self::CppInterface(..) => true,
            // TODO: others
            _ => false,
        }
    }

    pub fn is_convertible(&self) -> bool {
        match self {
            Self::CppStruct(item) => item.is_convertible(),
            Self::Delegate(..) | Self::Interface(..) | Self::Class(..) | Self::CppInterface(..) => {
                true
            }
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Enum(_) | Self::CppEnum(_) | Self::CppDelegate(_) => true,
            Self::CppStruct(item) => item.is_handle(),
            _ => false,
        }
    }

    pub fn runtime_signature(&self) -> String {
        match self {
            Self::Class(item) => item.runtime_signature(),
            Self::Delegate(item) => item.runtime_signature(),
            Self::Enum(item) => item.runtime_signature(),
            Self::Interface(item) => item.runtime_signature(),
            Self::Struct(item) => item.runtime_signature(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        match self {
            Self::Class(item) => item.dependencies(dependencies),
            Self::Delegate(item) => item.dependencies(dependencies),
            Self::Enum(item) => item.dependencies(dependencies),
            Self::Interface(item) => item.dependencies(dependencies),
            Self::Struct(item) => item.dependencies(dependencies),
            Self::CppConst(item) => item.dependencies(dependencies),
            Self::CppDelegate(item) => item.dependencies(dependencies),
            Self::CppFn(item) => item.dependencies(dependencies),
            Self::CppInterface(item) => item.dependencies(dependencies),
            Self::CppStruct(item) => item.dependencies(dependencies),
            Self::CppEnum(item) => item.dependencies(dependencies),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::CppStruct(item) => item.size(),
            Self::Struct(item) => item.size(),
            Self::CppEnum(item) => item.size(),
            Self::CppInterface(..) => 4,
            Self::CppDelegate(..) => 4,
            Self::Interface(..) => 4,
            Self::Class(..) => 4,
            rest => unimplemented!("{rest:?}"),
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Self::CppStruct(item) => item.align(),
            Self::Struct(item) => item.align(),
            Self::CppEnum(item) => item.align(),
            Self::CppInterface(..) => 4,
            Self::CppDelegate(..) => 4,
            Self::Interface(..) => 4,
            Self::Class(..) => 4,
            rest => unimplemented!("{rest:?}"),
        }
    }
}

fn interface_signature(def: TypeDef, generics: &[Type]) -> String {
    if generics.is_empty() {
        let guid = def.guid_attribute().unwrap();
        format!("{{{guid}}}")
    } else {
        let guid = def.guid_attribute().unwrap();
        let mut signature = format!("pinterface({{{guid}}}");

        for generic in generics {
            signature.push(';');
            signature.push_str(&generic.runtime_signature())
        }

        signature.push(')');
        signature
    }
}
