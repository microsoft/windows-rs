use super::*;

#[derive(Default)]
pub struct Records {
    pub Assembly: Vec<Assembly>,
    pub AssemblyRef: Vec<AssemblyRef>,
    pub Attribute: Vec<Attribute>,
    pub ClassLayout: Vec<ClassLayout>,
    pub Constant: Vec<Constant>,
    pub Field: Vec<Field>,
    pub GenericParam: Vec<GenericParam>,
    pub ImplMap: Vec<ImplMap>,
    pub InterfaceImpl: Vec<InterfaceImpl>,
    pub MemberRef: Vec<MemberRef>,
    pub MethodDef: Vec<MethodDef>,
    pub Module: Vec<Module>,
    pub ModuleRef: Vec<ModuleRef>,
    pub NestedClass: Vec<NestedClass>,
    pub Param: Vec<Param>,
    pub TypeDef: Vec<TypeDef>,
    pub TypeRef: Vec<TypeRef>,
    pub TypeSpec: Vec<TypeSpec>,
}

pub struct TypeSpec {
    pub Signature: id::BlobId,
}

pub struct NestedClass {
    pub NestedClass: u32,
    pub EnclosingClass: u32,
}

pub struct ModuleRef {
    pub Name: id::StringId,
}

#[derive(Default)]
pub struct Assembly {
    pub HashAlgId: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: AssemblyFlags,
    pub PublicKey: u32,
    pub Name: id::StringId,
    pub Culture: u32,
}

#[derive(Copy, Clone)]
pub struct InterfaceImpl {
    pub Class: id::TypeDef,
    pub Interface: TypeDefOrRef,
}

pub struct ImplMap {
    pub MappingFlags: PInvokeAttributes,
    pub MemberForwarded: MemberForwarded,
    pub ImportName: id::StringId,
    pub ImportScope: id::ModuleRef,
}

#[derive(Default)]
pub struct AssemblyRef {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: AssemblyFlags,
    pub PublicKeyOrToken: id::BlobId,
    pub Name: id::StringId,
    pub Culture: u32,
    pub HashValue: u32,
}

pub struct ClassLayout {
    pub PackingSize: u16,
    pub ClassSize: u32,
    pub Parent: u32,
}

#[derive(Copy, Clone)]
pub struct Constant {
    pub Type: u8,
    pub Parent: HasConstant,
    pub Value: id::BlobId,
}

pub struct Field {
    pub Flags: FieldAttributes,
    pub Name: id::StringId,
    pub Signature: id::BlobId,
}

pub struct MethodDef {
    pub RVA: u32,
    pub ImplFlags: MethodImplAttributes,
    pub Flags: MethodAttributes,
    pub Name: id::StringId,
    pub Signature: id::BlobId,
    pub ParamList: u32,
}

#[derive(Default)]
pub struct Module {
    pub Generation: u16,
    pub Name: id::StringId,
    pub Mvid: u32,
    pub EncId: u32,
    pub EncBaseId: u32,
}

#[derive(Copy, Clone)]
pub struct GenericParam {
    pub Number: u16,
    pub Flags: GenericParamAttributes,
    pub Owner: TypeOrMethodDef,
    pub Name: id::StringId,
}

pub struct Param {
    pub Flags: ParamAttributes,
    pub Sequence: u16,
    pub Name: id::StringId,
}

pub struct TypeDef {
    pub Flags: TypeAttributes,
    pub TypeName: id::StringId,
    pub TypeNamespace: id::StringId,
    pub Extends: TypeDefOrRef,
    pub FieldList: u32,
    pub MethodList: u32,
}

pub struct TypeRef {
    pub ResolutionScope: ResolutionScope,
    pub TypeName: id::StringId,
    pub TypeNamespace: id::StringId,
}

#[derive(Copy, Clone)]
pub struct Attribute {
    pub Parent: HasAttribute,
    pub Type: AttributeType,
    pub Value: id::BlobId,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct MemberRef {
    pub Parent: MemberRefParent,
    pub Name: id::StringId,
    pub Signature: id::BlobId,
}

impl Records {
    pub fn into_stream(self) -> Vec<u8> {
        let resolution_scope = coded_index_size(&[
            self.Module.len(),
            self.ModuleRef.len(),
            self.AssemblyRef.len(),
            self.TypeRef.len(),
        ]);
        let type_def_or_ref =
            coded_index_size(&[self.TypeDef.len(), self.TypeRef.len(), self.TypeSpec.len()]);
        let has_constant = coded_index_size(&[self.Field.len(), self.Param.len(), 0]);

        let type_or_method_def = coded_index_size(&[self.TypeDef.len(), self.MethodDef.len()]);

        let member_ref_parent = coded_index_size(&[
            self.TypeDef.len(),
            self.TypeRef.len(),
            self.ModuleRef.len(),
            self.MethodDef.len(),
            self.TypeSpec.len(),
        ]);
        let custom_attribute_type =
            coded_index_size(&[self.MethodDef.len(), self.MemberRef.len(), 0, 0, 0]);
        let has_custom_attribute = coded_index_size(&[
            self.MethodDef.len(),
            self.Field.len(),
            self.TypeRef.len(),
            self.TypeDef.len(),
            self.Param.len(),
            self.InterfaceImpl.len(),
            self.MemberRef.len(),
            self.Module.len(),
            0,
            0,
            0,
            self.ModuleRef.len(),
            self.TypeSpec.len(),
            0,
            self.AssemblyRef.len(),
            0,
            0,
            0,
            self.GenericParam.len(),
            0,
            0,
        ]);

        let member_forwarded = coded_index_size(&[self.Field.len(), self.MethodDef.len()]);

        let valid_tables: u64 = (1 << 0) | // Module 
        (1 << 0x01) | // TypeRef
        (1 << 0x02) | // TypeDef
        (1 << 0x04) | // Field
        (1 << 0x06) | // MethodDef
        (1 << 0x08) | // Param
        (1 << 0x09) | // InterfaceImpl
        (1 << 0x0A) | // MemberRef
        (1 << 0x0B) | // Constant
        (1 << 0x0C) | // CustomAttribute
        (1 << 0x0F) | // ClassLayout
        (1 << 0x1A) | // ModuleRef
        (1 << 0x1B) | // TypeSpec
        (1 << 0x1C) | // ImplMap
        (1 << 0x20) | // Assembly
        (1 << 0x23) | // AssemblyRef
        (1 << 0x29) | // NestedClass
        (1 << 0x2A); // GenericParam

        // The table stream header...

        let mut buffer = Vec::new();
        buffer.write_u32(0); // Reserved
        buffer.push(2); // MajorVersion
        buffer.push(0); // MinorVersion
        buffer.push(0b111); // HeapSizes
        buffer.push(0); // Reserved
        buffer.write_u64(valid_tables);
        buffer.write_u64(0); // Sorted

        // Followed by the length of each of the valid tables...

        buffer.write_u32(self.Module.len().try_into().unwrap());
        buffer.write_u32(self.TypeRef.len().try_into().unwrap());
        buffer.write_u32(self.TypeDef.len().try_into().unwrap());
        buffer.write_u32(self.Field.len().try_into().unwrap());
        buffer.write_u32(self.MethodDef.len().try_into().unwrap());
        buffer.write_u32(self.Param.len().try_into().unwrap());
        buffer.write_u32(self.InterfaceImpl.len().try_into().unwrap());
        buffer.write_u32(self.MemberRef.len().try_into().unwrap());
        buffer.write_u32(self.Constant.len().try_into().unwrap());
        buffer.write_u32(self.Attribute.len().try_into().unwrap());
        buffer.write_u32(self.ClassLayout.len().try_into().unwrap());
        buffer.write_u32(self.ModuleRef.len().try_into().unwrap());
        buffer.write_u32(self.TypeSpec.len().try_into().unwrap());
        buffer.write_u32(self.ImplMap.len().try_into().unwrap());
        buffer.write_u32(self.Assembly.len().try_into().unwrap());
        buffer.write_u32(self.AssemblyRef.len().try_into().unwrap());
        buffer.write_u32(self.NestedClass.len().try_into().unwrap());
        buffer.write_u32(self.GenericParam.len().try_into().unwrap());

        // Followed by each table's rows...

        for r in &self.Module {
            buffer.write_u16(r.Generation);
            buffer.write_u32(r.Name.0);
            buffer.write_u32(r.Mvid);
            buffer.write_u32(r.EncId);
            buffer.write_u32(r.EncBaseId);
        }

        for r in &self.TypeRef {
            buffer.write_code(r.ResolutionScope.encode(), resolution_scope);
            buffer.write_u32(r.TypeName.0);
            buffer.write_u32(r.TypeNamespace.0);
        }

        for r in &self.TypeDef {
            buffer.write_u32(r.Flags.0);
            buffer.write_u32(r.TypeName.0);
            buffer.write_u32(r.TypeNamespace.0);
            buffer.write_code(r.Extends.encode(), type_def_or_ref);
            buffer.write_index(r.FieldList, self.Field.len());
            buffer.write_index(r.MethodList, self.MethodDef.len());
        }

        for r in &self.Field {
            buffer.write_u16(r.Flags.0);
            buffer.write_u32(r.Name.0);
            buffer.write_u32(r.Signature.0);
        }

        for r in &self.MethodDef {
            buffer.write_u32(r.RVA);
            buffer.write_u16(r.ImplFlags.0);
            buffer.write_u16(r.Flags.0);
            buffer.write_u32(r.Name.0);
            buffer.write_u32(r.Signature.0);
            buffer.write_index(r.ParamList, self.Param.len());
        }

        for r in &self.Param {
            buffer.write_u16(r.Flags.0);
            buffer.write_u16(r.Sequence);
            buffer.write_u32(r.Name.0);
        }

        for r in &self.InterfaceImpl {
            buffer.write_index(r.Class.0, self.TypeDef.len());
            buffer.write_code(r.Interface.encode(), type_def_or_ref);
        }

        for r in &self.MemberRef {
            buffer.write_code(r.Parent.encode(), member_ref_parent);
            buffer.write_u32(r.Name.0);
            buffer.write_u32(r.Signature.0);
        }

        for r in &self.Constant {
            buffer.push(r.Type);
            buffer.push(0);
            buffer.write_code(r.Parent.encode(), has_constant);
            buffer.write_u32(r.Value.0);
        }

        for r in &self.Attribute {
            buffer.write_code(r.Parent.encode(), has_custom_attribute);
            buffer.write_code(r.Type.encode(), custom_attribute_type);
            buffer.write_u32(r.Value.0);
        }

        for r in &self.ClassLayout {
            buffer.write_u16(r.PackingSize);
            buffer.write_u32(r.ClassSize);
            buffer.write_index(r.Parent, self.TypeDef.len());
        }

        for r in &self.ModuleRef {
            buffer.write_u32(r.Name.0);
        }

        for r in &self.TypeSpec {
            buffer.write_u32(r.Signature.0);
        }

        for r in &self.ImplMap {
            buffer.write_u16(r.MappingFlags.0);
            buffer.write_code(r.MemberForwarded.encode(), member_forwarded);
            buffer.write_u32(r.ImportName.0);
            buffer.write_index(r.ImportScope.0, self.ModuleRef.len());
        }

        for r in &self.Assembly {
            buffer.write_u32(r.HashAlgId);
            buffer.write_u16(r.MajorVersion);
            buffer.write_u16(r.MinorVersion);
            buffer.write_u16(r.BuildNumber);
            buffer.write_u16(r.RevisionNumber);
            buffer.write_u32(r.Flags.0);
            buffer.write_u32(r.PublicKey);
            buffer.write_u32(r.Name.0);
            buffer.write_u32(r.Culture);
        }

        for r in &self.AssemblyRef {
            buffer.write_u16(r.MajorVersion);
            buffer.write_u16(r.MinorVersion);
            buffer.write_u16(r.BuildNumber);
            buffer.write_u16(r.RevisionNumber);
            buffer.write_u32(r.Flags.0);
            buffer.write_u32(r.PublicKeyOrToken.0);
            buffer.write_u32(r.Name.0);
            buffer.write_u32(r.Culture);
            buffer.write_u32(r.HashValue);
        }

        for r in &self.NestedClass {
            buffer.write_index(r.NestedClass, self.TypeDef.len());
            buffer.write_index(r.EnclosingClass, self.TypeDef.len());
        }

        for r in &self.GenericParam {
            buffer.write_u16(r.Number);
            buffer.write_u16(r.Flags.0);
            buffer.write_code(r.Owner.encode(), type_or_method_def);
            buffer.write_u32(r.Name.0);
        }

        buffer.into_stream()
    }
}

// A coded index (see codes.rs) is a table index that may refer to different tables. The size of the column in memory
// must therefore be large enough to hold an index for a row in the largest possible table. This function determines
// this size for the given winmd file.
fn coded_index_size(tables: &[usize]) -> usize {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        while {
            value >>= 1;
            value != 0
        } {
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables.iter().all(|table| small(*table, bits_needed)) {
        2
    } else {
        4
    }
}
