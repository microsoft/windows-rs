use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::RwLock,
};

mod abi;
mod analysis;
mod cache;
mod lower;
mod metadata;
mod projection;

#[derive(Default)]
pub(super) struct DependencyCache {
    values: RwLock<BTreeMap<(String, String), BTreeSet<(String, String)>>>,
    interfaces: RwLock<BTreeMap<(String, String), InterfaceDependencies>>,
    interface_bases: BTreeMap<(String, String), BTreeSet<(String, String)>>,
    sys_namespaces: BTreeSet<String>,
}

#[derive(Clone, Default)]
pub(super) struct InterfaceDependencies {
    pub(super) package: BTreeSet<(String, String)>,
    pub(super) manifest: BTreeSet<(String, String)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Type {
    Void,
    Boolean,
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
    String,
    ISize,
    USize,
    Array {
        element: Box<Self>,
        len: usize,
    },
    Pointer {
        mutable: bool,
        element: Box<Self>,
    },
    Interface {
        namespace: String,
        name: String,
        arguments: Vec<ty::Type>,
    },
    Named {
        namespace: String,
        name: String,
        canonical: Option<canonical::Type>,
    },
}

#[derive(Clone, Copy)]
pub(super) struct TraitSupport {
    pub(super) copy: bool,
    pub(super) debug: bool,
    pub(super) partial_eq: bool,
    pub(super) eq: bool,
}

impl TraitSupport {
    pub(super) const NONE: Self = Self {
        copy: false,
        debug: false,
        partial_eq: false,
        eq: false,
    };

    pub(super) const ALL: Self = Self {
        copy: true,
        debug: true,
        partial_eq: true,
        eq: true,
    };

    pub(super) fn combine(&mut self, other: Self) {
        self.copy &= other.copy;
        self.debug &= other.debug;
        self.partial_eq &= other.partial_eq;
        self.eq &= other.eq;
    }
}

impl Type {
    fn canonical(&self) -> Option<canonical::Type> {
        match self {
            Self::Named { canonical, .. } => *canonical,
            _ => None,
        }
    }
}

pub(super) fn metadata_has_oversized_member(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
) -> Result<bool, Error> {
    metadata::metadata_has_oversized_member(database, file, ty)
}

pub(super) fn metadata_exceeds_retval_limit(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
) -> Result<bool, Error> {
    metadata::metadata_exceeds_retval_limit(database, file, ty)
}

pub(super) fn is_core_projection(namespace: &str, name: &str) -> bool {
    metadata::is_core_projection(namespace, name)
}

pub(super) fn core_projection(namespace: &str, name: &str) -> Option<TokenStream> {
    metadata::core_projection(namespace, name)
}

fn align_up(value: usize, align: usize) -> usize {
    value.saturating_add(align - 1) & !(align - 1)
}

pub(super) fn write_value(ty: &Type, value: &ConstantValue) -> TokenStream {
    match (ty, value) {
        (Type::USize, ConstantValue::USize(value) | ConstantValue::U64(value))
            if *value > u32::MAX as u64 =>
        {
            let value = Literal::u64_suffixed(*value);
            return quote! { #value as usize };
        }
        (Type::USize, ConstantValue::I32(value)) => {
            let value = Literal::i32_suffixed(*value);
            return quote! { #value as usize };
        }
        (Type::USize, ConstantValue::I64(value)) => {
            let value = Literal::i64_suffixed(*value);
            return quote! { #value as usize };
        }
        (Type::ISize, ConstantValue::ISize(value) | ConstantValue::I64(value))
            if !(i32::MIN as i64..=i32::MAX as i64).contains(value) =>
        {
            let value = Literal::i64_suffixed(*value);
            return quote! { #value as isize };
        }
        (Type::ISize, ConstantValue::U32(value)) => {
            let value = Literal::u32_suffixed(*value);
            return quote! { #value as isize };
        }
        (Type::ISize, ConstantValue::U64(value)) => {
            let value = Literal::u64_suffixed(*value);
            return quote! { #value as isize };
        }
        _ => {}
    }
    let literal = match value {
        ConstantValue::Boolean(value) => return quote! { #value },
        ConstantValue::Char(value) | ConstantValue::U16(value) => Literal::u16_unsuffixed(*value),
        ConstantValue::I8(value) => Literal::i8_unsuffixed(*value),
        ConstantValue::U8(value) => Literal::u8_unsuffixed(*value),
        ConstantValue::I16(value) => Literal::i16_unsuffixed(*value),
        ConstantValue::I32(value) => Literal::i32_unsuffixed(*value),
        ConstantValue::U32(value) => Literal::u32_unsuffixed(*value),
        ConstantValue::I64(value) | ConstantValue::ISize(value) => Literal::i64_unsuffixed(*value),
        ConstantValue::U64(value) | ConstantValue::USize(value) => Literal::u64_unsuffixed(*value),
        ConstantValue::F32(value) => Literal::f32_unsuffixed(*value),
        ConstantValue::F64(value) => Literal::f64_unsuffixed(*value),
        ConstantValue::String(_) | ConstantValue::Null => unreachable!(),
    };
    quote! { #literal }
}

fn integer(value: &ConstantValue) -> bool {
    matches!(
        value,
        ConstantValue::I8(_)
            | ConstantValue::U8(_)
            | ConstantValue::I16(_)
            | ConstantValue::U16(_)
            | ConstantValue::I32(_)
            | ConstantValue::U32(_)
            | ConstantValue::I64(_)
            | ConstantValue::U64(_)
            | ConstantValue::ISize(_)
            | ConstantValue::USize(_)
    )
}
