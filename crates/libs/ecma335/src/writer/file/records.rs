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
    pub Property: Vec<Property>,
    pub TypeDef: Vec<TypeDef>,
    pub TypeRef: Vec<TypeRef>,
    pub TypeSpec: Vec<TypeSpec>,
}

pub struct TypeSpec {
    pub Signature: u32,
}

pub struct Property {
    pub Flags: u16,
    pub Name: u32,
    pub Type: u32,
}

pub struct NestedClass {
    pub NestedClass: u32,
    pub EnclosingClass: u32,
}

pub struct ModuleRef {
    pub Name: u32,
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
    pub Name: u32,
    pub Culture: u32,
}

#[derive(Copy, Clone)]
pub struct InterfaceImpl {
    pub Class: u32,
    pub Interface: TypeDefOrRef,
}

pub struct ImplMap {
    pub MappingFlags: u16,
    pub MemberForwarded: u32,
    pub ImportName: u32,
    pub ImportScope: u32,
}

#[derive(Default)]
pub struct AssemblyRef {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: AssemblyFlags,
    pub PublicKeyOrToken: u32,
    pub Name: u32,
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
    pub Value: ConstantValue,
}

pub struct Field {
    pub Flags: FieldAttributes,
    pub Name: u32,
    pub Signature: FieldSig,
}

pub struct MethodDef {
    pub RVA: u32,
    pub ImplFlags: MethodImplAttributes,
    pub Flags: MethodAttributes,
    pub Name: u32,
    pub Signature: MethodDefSig,
    pub ParamList: u32,
}

#[derive(Default)]
pub struct Module {
    pub Generation: u16,
    pub Name: u32,
    pub Mvid: u32,
    pub EncId: u32,
    pub EncBaseId: u32,
}

#[derive(Copy, Clone)]
pub struct GenericParam {
    pub Number: u16,
    pub Flags: GenericParamAttributes,
    pub Owner: TypeOrMethodDef,
    pub Name: u32,
}

pub struct Param {
    pub Flags: ParamAttributes,
    pub Sequence: u16,
    pub Name: u32,
}

pub struct TypeDef {
    pub Flags: TypeAttributes,
    pub TypeName: u32,
    pub TypeNamespace: u32,
    pub Extends: TypeDefOrRef,
    pub FieldList: u32,
    pub MethodList: u32,
}

pub struct TypeRef {
    pub ResolutionScope: ResolutionScope,
    pub TypeName: u32,
    pub TypeNamespace: u32,
}

#[derive(Copy, Clone)]
pub struct Attribute {
    pub Parent: HasAttribute,
    pub Type: AttributeType,
    pub Value: AttributeValue,
}

pub struct MemberRef {
    pub Parent: MemberRefParent,
    pub Name: u32,
    pub Signature: MethodDefSig,
}

impl Records {
    pub fn into_stream(self) -> Vec<u8> {
        if [
            self.Assembly.len(),
            self.AssemblyRef.len(),
            self.Attribute.len(),
            self.ClassLayout.len(),
            self.Constant.len(),
            self.Field.len(),
            self.GenericParam.len(),
            self.ImplMap.len(),
            self.InterfaceImpl.len(),
            self.MemberRef.len(),
            self.MethodDef.len(),
            self.Module.len(),
            self.ModuleRef.len(),
            self.NestedClass.len(),
            self.Param.len(),
            self.Property.len(),
            self.TypeDef.len(),
            self.TypeRef.len(),
            self.TypeSpec.len(),
        ]
        .iter()
        .any(|len| *len > u32::MAX as usize)
        {
            panic!("metadata table too large");
        }

        let resolution_scope = coded_index_size(&[
            self.Module.len(),
            self.ModuleRef.len(),
            self.AssemblyRef.len(),
            self.TypeRef.len(),
        ]);
        let type_def_or_ref =
            coded_index_size(&[self.TypeDef.len(), self.TypeRef.len(), self.TypeSpec.len()]);
        let has_constant =
            coded_index_size(&[self.Field.len(), self.Param.len(), self.Property.len()]);

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
        (1 << 0x17) | // Property
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

        buffer.write_u32(self.Module.len() as u32);
        buffer.write_u32(self.TypeRef.len() as u32);
        buffer.write_u32(self.TypeDef.len() as u32);
        buffer.write_u32(self.Field.len() as u32);
        buffer.write_u32(self.MethodDef.len() as u32);
        buffer.write_u32(self.Param.len() as u32);
        buffer.write_u32(self.InterfaceImpl.len() as u32);
        buffer.write_u32(self.MemberRef.len() as u32);
        buffer.write_u32(self.Constant.len() as u32);
        buffer.write_u32(self.Attribute.len() as u32);
        buffer.write_u32(self.ClassLayout.len() as u32);
        buffer.write_u32(self.Property.len() as u32);
        buffer.write_u32(self.ModuleRef.len() as u32);
        buffer.write_u32(self.TypeSpec.len() as u32);
        buffer.write_u32(self.ImplMap.len() as u32);
        buffer.write_u32(self.Assembly.len() as u32);
        buffer.write_u32(self.AssemblyRef.len() as u32);
        buffer.write_u32(self.NestedClass.len() as u32);
        buffer.write_u32(self.GenericParam.len() as u32);

        // Followed by each table's rows...

        for x in self.Module {
            buffer.write_u16(x.Generation);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Mvid);
            buffer.write_u32(x.EncId);
            buffer.write_u32(x.EncBaseId);
        }

        for x in self.TypeRef {
            buffer.write_code(x.ResolutionScope.encode(), resolution_scope);
            buffer.write_u32(x.TypeName);
            buffer.write_u32(x.TypeNamespace);
        }

        for x in &self.TypeDef {
            buffer.write_u32(x.Flags.0);
            buffer.write_u32(x.TypeName);
            buffer.write_u32(x.TypeNamespace);
            buffer.write_code(x.Extends.encode(), type_def_or_ref);
            buffer.write_index(x.FieldList, self.Field.len());
            buffer.write_index(x.MethodList, self.MethodDef.len());
        }

        for x in self.Field {
            buffer.write_u16(x.Flags.0);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Signature.0);
        }

        for x in self.MethodDef {
            buffer.write_u32(x.RVA);
            buffer.write_u16(x.ImplFlags.0);
            buffer.write_u16(x.Flags.0);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Signature.0);
            buffer.write_index(x.ParamList, self.Param.len());
        }

        for x in self.Param {
            buffer.write_u16(x.Flags.0);
            buffer.write_u16(x.Sequence);
            buffer.write_u32(x.Name);
        }

        for x in self.InterfaceImpl {
            buffer.write_index(x.Class, self.TypeDef.len());
            buffer.write_code(x.Interface.encode(), type_def_or_ref);
        }

        for x in self.MemberRef {
            buffer.write_code(x.Parent.encode(), member_ref_parent);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Signature.0);
        }

        for x in self.Constant {
            buffer.push(x.Type);
            buffer.push(0);
            buffer.write_code(x.Parent.encode(), has_constant);
            buffer.write_u32(x.Value.0);
        }

        for x in self.Attribute {
            buffer.write_code(x.Parent.encode(), has_custom_attribute);
            buffer.write_code(x.Type.encode(), custom_attribute_type);
            buffer.write_u32(x.Value.0);
        }

        for x in self.TypeSpec {
            buffer.write_u32(x.Signature);
        }

        for x in self.Assembly {
            buffer.write_u32(x.HashAlgId);
            buffer.write_u16(x.MajorVersion);
            buffer.write_u16(x.MinorVersion);
            buffer.write_u16(x.BuildNumber);
            buffer.write_u16(x.RevisionNumber);
            buffer.write_u32(x.Flags.0);
            buffer.write_u32(x.PublicKey);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Culture);
        }

        for x in self.AssemblyRef {
            buffer.write_u16(x.MajorVersion);
            buffer.write_u16(x.MinorVersion);
            buffer.write_u16(x.BuildNumber);
            buffer.write_u16(x.RevisionNumber);
            buffer.write_u32(x.Flags.0);
            buffer.write_u32(x.PublicKeyOrToken);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Culture);
            buffer.write_u32(x.HashValue);
        }

        for x in self.GenericParam {
            buffer.write_u16(x.Number);
            buffer.write_u16(x.Flags.0);
            buffer.write_code(x.Owner.encode(), type_or_method_def);
            buffer.write_u32(x.Name);
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
