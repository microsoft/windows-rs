mod blobs;
mod codes;
mod definitions;
mod file;
mod references;
mod strings;
mod tables;

use super::*;
use blobs::*;
use codes::*;
use definitions::*;
use references::*;
use strings::*;
use tables::Tables;

use std::collections::BTreeMap;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub fn write(name: &str, _winrt: bool, items: &[Item], _assemblies: &[&str]) -> Vec<u8> {
    // Build sorted list of definitions.
    let definitions = {
        let mut definitions = Definitions::default();
        items.iter().for_each(|item| definitions.insert(item));
        definitions.stage()
    };

    // Build sorted list of references used by definitions.
    let _references = {
        let mut references = References::default();
        for item in items {
            match item {
                Item::Struct(ty) => ty.fields.iter().for_each(|field| type_reference(&field.ty, &definitions, &mut references)),
                Item::Interface(_ty) => {}
                _ => {}
            }
        }
        references.stage()
    };

    // Now that we have stable type indexes, walk the items and build blobs and index strings.
    let (blobs, strings) = {
        let blobs = Blobs::default();
        let mut strings = Strings::new(name);

        for item in items {
            match item {
                Item::Struct(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                }
                Item::Enum(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                }
                Item::Interface(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                }
            }
        }

        (blobs.stage(), strings.stage())
    };

    // Now that everything is indexed in various heaps, we can write out the table records.
    let tables = {
        let mut tables = Tables::default();
        tables.Module.push(tables::Module { Name: strings.index(name), Mvid: 1, ..Default::default() });
        tables.TypeDef.push(tables::TypeDef { TypeName: strings.index("<Module>"), ..Default::default() });
        let mscorlib = tables.AssemblyRef.push2(tables::AssemblyRef { MajorVersion: 4, Name: strings.index("mscorlib"), ..Default::default() });
        let value_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.index("ValueType"), TypeNamespace: strings.index("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });
        let enum_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.index("Enum"), TypeNamespace: strings.index("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });

        tables
    };

    // With all of the streams prepared, we can write out ECMA-335 file format.
    file::write(tables, strings, blobs)
}

fn type_reference<'a>(ty: &'a Type, definitions: &StagedDefinitions, references: &mut References<'a>) {
    match ty {
        Type::Named((namespace, name)) => {
            if definitions.get(namespace, name).is_none() {
                references.insert(namespace, name);
            }
        }
        _ => {}
    }
}

pub trait Write {
    unsafe fn write_header<T: Sized>(&mut self, value: &T);
    fn write_u8(&mut self, value: u8);
    fn write_u16(&mut self, value: u16);
    fn write_u32(&mut self, value: u32);
    fn write_u64(&mut self, value: u64);
    fn write_code(&mut self, value: u32, size: usize);
    fn write_index(&mut self, index: u32, len: usize);
}

impl Write for Vec<u8> {
    unsafe fn write_header<T: Sized>(&mut self, value: &T) {
        self.extend_from_slice(std::slice::from_raw_parts(value as *const _ as _, std::mem::size_of::<T>()));
    }

    fn write_u8(&mut self, value: u8) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u16(&mut self, value: u16) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u32(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u64(&mut self, value: u64) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_code(&mut self, value: u32, size: usize) {
        if size == 2 {
            self.write_u16(value as _);
        } else {
            self.write_u32(value);
        }
    }

    fn write_index(&mut self, index: u32, len: usize) {
        if len < (1 << 16) {
            self.write_u16(index as u16 + 1);
        } else {
            self.write_u32(index + 1);
        }
    }
}

trait Push2<T> {
    fn push2(&mut self, value: T) -> u32;
}

impl<T> Push2<T> for Vec<T> {
    fn push2(&mut self, value: T) -> u32 {
        self.push(value);
        (self.len() - 1) as _
    }
}
