use crate::*;
use winmd_macros::*;

table!(Constant);
table!(CustomAttribute);
table!(Field);
table!(GenericParam);
table!(InterfaceImpl);
table!(MemberRef);
table!(MethodDef);
table!(Param);
table!(TypeDef);
table!(TypeRef);
table!(TypeSpec);

pub enum TypeCategory {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
    Attribute,
    Contract,
}

pub enum ConstantValue {
    I32(i32),
    U32(u32),
}

impl CustomAttribute {
    pub fn parent(&self, r: &Reader) -> HasCustomAttribute {
        r.decode(&self.row, 0)
    }

    pub fn constructor(&self, r: &Reader) -> CustomAttributeType {
        r.decode(&self.row, 1)
    }

    pub fn name<'a>(&self, r: &'a Reader) -> (&'a str, &'a str) {
        match self.constructor(r) {
            CustomAttributeType::MethodDef(value) => {
                let value = value.parent(r);
                (value.namespace(r), value.name(r))
            }
            CustomAttributeType::MemberRef(value) => match value.parent(r) {
                MemberRefParent::TypeDef(value) => (value.namespace(r), value.name(r)),
                MemberRefParent::TypeRef(value) => (value.namespace(r), value.name(r)),
                _ => panic!(),
            },
        }
    }

    pub fn arguments(&self, r: &Reader) -> Vec<(String, ArgumentSig)> {
        match self.constructor(r) {
            CustomAttributeType::MethodDef(value) => ArgumentSig::new(r, self.row.file, r.blob(&value.row, 4), r.blob(&self.row, 2)),
            CustomAttributeType::MemberRef(value) => ArgumentSig::new(r, self.row.file, r.blob(&value.row, 2), r.blob(&self.row, 2)),
        }
    }
}

impl Field {
    pub fn name<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 1)
    }

    pub fn signature(&self, r: &Reader) -> TypeSig {
        field_sig(self, r)
    }

    pub fn constants(&self, r: &Reader) -> RowIterator<Constant> {
        r.equal_range(self.row.file, 1, HasConstant::Field(*self).encode())
    }
}

impl GenericParam {
    pub fn name<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 3)
    }
}

impl InterfaceImpl {
    pub fn interface(&self, r: &Reader) -> TypeDefOrRef {
        r.decode(&self.row, 1)
    }

    pub fn attributes(&self, r: &Reader) -> RowIterator<CustomAttribute> {
        r.equal_range(self.row.file, 0, HasCustomAttribute::InterfaceImpl(*self).encode())
    }

    pub fn has_attribute(&self, r: &Reader, namespace: &str, name: &str) -> bool {
        self.attributes(r).any(|attribute| attribute.name(r) == (namespace, name))
    }
}

impl MemberRef {
    pub fn parent(&self, r: &Reader) -> MemberRefParent {
        r.decode(&self.row, 0)
    }

    pub fn name<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 1)
    }
}

impl MethodDef {
    pub fn flags(&self, r: &Reader) -> MethodAttributes {
        MethodAttributes(r.u32(&self.row, 2))
    }

    pub fn parent(&self, r: &Reader) -> TypeDef {
        r.upper_bound(self.row.file, 6, self.row.row)
    }

    pub(crate) fn params(&self, r: &Reader) -> RowIterator<Param> {
        r.list(&self.row, 5)
    }

    pub fn name<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 3)
    }

    pub fn signature(&self, r: &Reader) -> MethodSig {
        MethodSig::new(r, self)
    }

    pub fn is_add_overload(&self, r: &Reader) -> bool {
        self.flags(r).special() && self.name(r).starts_with("add")
    }

    pub fn is_remove_overload(&self, r: &Reader) -> bool {
        self.flags(r).special() && self.name(r).starts_with("remove")
    }

    pub fn attributes(&self, r: &Reader) -> RowIterator<CustomAttribute> {
        r.equal_range(self.row.file, 0, HasCustomAttribute::MethodDef(*self).encode())
    }

    pub fn find_attribute(&self, r: &Reader, namespace: &str, name: &str) -> Option<CustomAttribute> {
        self.attributes(r).find(|attribute| attribute.name(r) == (namespace, name))
    }
}

impl Param {
    pub fn flags(&self, r: &Reader) -> ParamAttributes {
        ParamAttributes(r.u32(&self.row, 0))
    }

    pub fn sequence(&self, r: &Reader) -> u32 {
        r.u32(&self.row, 1)
    }

    pub fn name<'a>(&self, r: &'a Reader) -> String {
        to_snake(r.str(&self.row, 2))
    }
}

impl TypeDef {
    pub fn invalid() -> TypeDef {
        Self { row: RowData::invalid() }
    }

    pub fn flags(&self, r: &Reader) -> TypeAttributes {
        TypeAttributes(r.u32(&self.row, 0))
    }

    pub fn name<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 1)
    }

    pub fn namespace<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 2)
    }

    pub fn extends(&self, r: &Reader) -> TypeDefOrRef {
        r.decode(&self.row, 3)
    }

    pub fn fields(&self, r: &Reader) -> RowIterator<Field> {
        r.list(&self.row, 4)
    }

    pub fn methods(&self, r: &Reader) -> RowIterator<MethodDef> {
        r.list(&self.row, 5)
    }

    pub fn generics(&self, r: &Reader) -> RowIterator<GenericParam> {
        r.equal_range(self.row.file, 2, TypeOrMethodDef::TypeDef(*self).encode())
    }

    pub fn interfaces(&self, r: &Reader) -> RowIterator<InterfaceImpl> {
        r.equal_range(self.row.file, 0, self.row.row + 1)
    }

    pub fn attributes(&self, r: &Reader) -> RowIterator<CustomAttribute> {
        r.equal_range(self.row.file, 0, HasCustomAttribute::TypeDef(*self).encode())
    }

    pub fn has_attribute(&self, r: &Reader, namespace: &str, name: &str) -> bool {
        self.attributes(r).any(|attribute| attribute.name(r) == (namespace, name))
    }

    pub fn find_attribute(&self, r: &Reader, namespace: &str, name: &str) -> Option<CustomAttribute> {
        self.attributes(r).find(|attribute| attribute.name(r) == (namespace, name))
    }

    pub fn category(&self, r: &Reader) -> TypeCategory {
        if self.flags(r).interface() {
            TypeCategory::Interface
        } else {
            match self.extends(r).name(r) {
                "Enum" => TypeCategory::Enum,
                "MulticastDelegate" => TypeCategory::Delegate,
                "ValueType" => {
                    if self.has_attribute(r, "Windows.Foundation.Metadata", "ApiContractAttribute") {
                        TypeCategory::Contract
                    } else {
                        TypeCategory::Struct
                    }
                }
                "Attribute" => TypeCategory::Attribute,
                _ => TypeCategory::Class,
            }
        }
    }

    // TODO: should return BaseIterator
    pub fn bases(&self, r: &Reader) -> Vec<TypeDef> {
        let mut bases = Vec::new();
        let mut current = *self;

        loop {
            let extends = current.extends(r);
            let namespace = extends.namespace(r);
            let name = extends.name(r);

            if namespace == "System" && name == "Object" {
                break;
            }

            current = extends.resolve(r);
            bases.push(current);
        }

        bases
    }
}

impl TypeRef {
    pub fn name<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 1)
    }

    pub fn namespace<'a>(&self, r: &'a Reader) -> &'a str {
        r.str(&self.row, 2)
    }

    // TODO: panic with "'full name' not found"
    pub fn resolve(&self, r: &Reader) -> TypeDef {
        *r.namespaces().get(self.namespace(r)).unwrap().get(self.name(r)).unwrap()
    }
}

impl TypeSpec {
    pub fn signature(&self, r: &Reader) -> GenericSig {
        let mut bytes = r.blob(&self.row, 0);
        read_unsigned(&mut bytes);
        GenericSig::new(self.row.file, &mut bytes)
    }
}
