#![allow(dead_code)]

use crate::blob::Blob;
use crate::codes::{Decode, TypeDefOrRef};
use crate::element_type::ElementType;

pub struct CustomModSig {
    pub cmod: ElementType,
    type_def_or_ref: TypeDefOrRef,
}

impl CustomModSig {
    pub fn from_blob(blob: &mut Blob) -> CustomModSig {
        CustomModSig {
            cmod: ElementType::from_blob(blob),
            type_def_or_ref: TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index),
        }
    }
}

pub struct GenericTypeInstSig {
    pub class_or_value: Box<ElementType>,
    pub type_def_or_ref: TypeDefOrRef,
    pub generic_arg_count: u32,
    pub generic_args: Vec<TypeSig>,
}

impl GenericTypeInstSig {
    pub fn from_blob(blob: &mut Blob) -> GenericTypeInstSig {
        let class_or_value = Box::new(ElementType::from_blob(blob));
        let type_def_or_ref = TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index);
        let generic_arg_count = blob.read_unsigned();

        let mut generic_args = Vec::with_capacity(generic_arg_count as usize);
        for _ in 0..generic_arg_count - 1 {
            generic_args.push(TypeSig::from_blob(blob));
        }

        GenericTypeInstSig {
            class_or_value,
            type_def_or_ref,
            generic_arg_count,
            generic_args,
        }
    }
}

pub enum TypeSigValueType {
    ElementType(Box<ElementType>),
    GenericTypeIndex(u32),
    GenericTypeInstSig(GenericTypeInstSig),
    GenericMethodTypeIndex(u32),
}

pub struct TypeSig {
    pub is_szarray: bool,
    pub cmod: Vec<CustomModSig>,
    pub element_type: Box<ElementType>,
    pub value_type: TypeSigValueType,
}

impl TypeSig {
    pub fn from_blob(blob: &mut Blob) -> TypeSig {
        let mut element_type = ElementType::from_blob(blob);
        let is_szarray = match element_type {
            ElementType::SZArray => true,
            _ => false,
        };

        let mut cmod = Vec::new();
        loop {
            match element_type {
                ElementType::CModReqd(_) | ElementType::CModOpt(_) => {
                    let cmod_sig = CustomModSig::from_blob(blob);
                    cmod.push(cmod_sig);
                    element_type = ElementType::from_blob(blob);
                }
                _ => break,
            }
        }

        let element_type = ElementType::from_blob(blob);

        let value_type_element_type = ElementType::from_blob(blob);
        let value_type = match value_type_element_type {
            ElementType::Boolean
            | ElementType::Char
            | ElementType::I1
            | ElementType::U1
            | ElementType::I2
            | ElementType::U2
            | ElementType::I4
            | ElementType::U4
            | ElementType::I8
            | ElementType::U8
            | ElementType::R4
            | ElementType::R8
            | ElementType::String
            | ElementType::Object
            | ElementType::U
            | ElementType::I
            | ElementType::Class(_)
            | ElementType::ValueType(_) => {
                TypeSigValueType::ElementType(Box::new(value_type_element_type))
            }
            ElementType::GenericInst => {
                TypeSigValueType::GenericTypeInstSig(GenericTypeInstSig::from_blob(blob))
            }
            ElementType::Var => TypeSigValueType::GenericTypeIndex(blob.read_unsigned()),
            ElementType::MVar => TypeSigValueType::GenericMethodTypeIndex(blob.read_unsigned()),
            _ => panic!("Unrecognized ELEMENT_TYPE encountered"),
        };

        TypeSig {
            is_szarray: is_szarray,
            cmod: cmod,
            element_type: Box::new(element_type),
            value_type: value_type,
        }
    }
}
