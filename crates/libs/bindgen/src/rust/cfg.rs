use super::*;

#[derive(Default, Clone)]
pub struct Cfg<'a> {
    pub types: BTreeMap<&'a str, BTreeSet<TypeDef>>,
    pub core_types: BTreeSet<Type>,
    pub arches: BTreeSet<&'static str>,
    pub implement: bool,
}

impl<'a> Cfg<'a> {
    pub fn add_feature(&mut self, feature: &'a str) {
        self.types.entry(feature).or_default();
    }
    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::default();
        self.types.keys().for_each(|feature| {
            union.types.entry(feature).or_default();
        });
        other.types.keys().for_each(|feature| {
            union.types.entry(feature).or_default();
        });
        self.arches.iter().for_each(|arch| {
            union.arches.insert(arch);
        });
        other.arches.iter().for_each(|arch| {
            union.arches.insert(arch);
        });
        union
    }
}

pub fn field_cfg<'a>(reader: &'a Reader<'a>, row: Field) -> Cfg<'a> {
    let mut cfg = Cfg::default();
    field_cfg_combine(reader, row, None, &mut cfg);
    cfg
}
fn field_cfg_combine<'a>(reader: &'a Reader, row: Field, enclosing: Option<TypeDef>, cfg: &mut Cfg<'a>) {
    type_cfg_combine(reader, &reader.field_type(row, enclosing), cfg)
}

pub fn type_def_cfg<'a>(reader: &'a Reader, row: TypeDef, generics: &[Type]) -> Cfg<'a> {
    let mut cfg = Cfg::default();
    type_def_cfg_combine(reader, row, generics, &mut cfg);
    cfg_add_attributes(reader, &mut cfg, row);
    cfg
}
pub fn type_def_cfg_impl<'a>(reader: &'a Reader, def: TypeDef, generics: &[Type]) -> Cfg<'a> {
    let mut cfg = Cfg { implement: true, ..Default::default() };

    fn combine<'a>(reader: &'a Reader, def: TypeDef, generics: &[Type], cfg: &mut Cfg<'a>) {
        type_def_cfg_combine(reader, def, generics, cfg);

        for method in reader.type_def_methods(def) {
            signature_cfg_combine(reader, &reader.method_def_signature(method, generics), cfg);
        }
    }

    combine(reader, def, generics, &mut cfg);

    for def in type_def_vtables(reader, def) {
        if let Type::TypeDef(def, generics) = def {
            combine(reader, def, &generics, &mut cfg);
        }
    }

    if reader.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
        for interface in type_def_interfaces(reader, def, generics) {
            if let Type::TypeDef(def, generics) = interface {
                combine(reader, def, &generics, &mut cfg);
            }
        }
    }

    cfg_add_attributes(reader, &mut cfg, def);
    cfg
}
pub fn type_def_cfg_combine<'a>(reader: &'a Reader, row: TypeDef, generics: &[Type], cfg: &mut Cfg<'a>) {
    let type_name = reader.type_def_type_name(row);

    for generic in generics {
        type_cfg_combine(reader, generic, cfg);
    }

    if cfg.types.entry(type_name.namespace).or_default().insert(row) {
        match reader.type_def_kind(row) {
            TypeKind::Class => {
                if let Some(default_interface) = type_def_default_interface(reader, row) {
                    type_cfg_combine(reader, &default_interface, cfg);
                }
            }
            TypeKind::Interface => {
                if !reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) {
                    for def in type_def_vtables(reader, row) {
                        if let Type::TypeDef(def, _) = def {
                            cfg.add_feature(reader.type_def_namespace(def));
                        }
                    }
                }
            }
            TypeKind::Struct => {
                reader.type_def_fields(row).for_each(|field| field_cfg_combine(reader, field, Some(row), cfg));
                if !type_name.namespace.is_empty() {
                    for def in reader.get_type_def(type_name) {
                        if def != row {
                            type_def_cfg_combine(reader, def, &[], cfg);
                        }
                    }
                }
            }
            TypeKind::Delegate => signature_cfg_combine(reader, &reader.method_def_signature(type_def_invoke_method(reader, row), generics), cfg),
            _ => {}
        }
    }
}

pub fn signature_cfg<'a>(reader: &'a Reader, method: MethodDef) -> Cfg<'a> {
    let mut cfg = Cfg::default();
    signature_cfg_combine(reader, &reader.method_def_signature(method, &[]), &mut cfg);
    cfg_add_attributes(reader, &mut cfg, method);
    cfg
}
fn signature_cfg_combine<'a>(reader: &'a Reader, signature: &MethodDefSig, cfg: &mut Cfg<'a>) {
    type_cfg_combine(reader, &signature.return_type, cfg);
    signature.params.iter().for_each(|param| type_cfg_combine(reader, param, cfg));
}

fn cfg_add_attributes<R: AsRow + Into<HasAttribute>>(reader: &Reader, cfg: &mut Cfg, row: R) {
    for attribute in reader.attributes(row) {
        match reader.attribute_name(attribute) {
            "SupportedArchitectureAttribute" => {
                if let Some((_, Value::EnumDef(_, value))) = reader.attribute_args(attribute).get(0) {
                    if let Value::I32(value) = **value {
                        if value & 1 == 1 {
                            cfg.arches.insert("x86");
                        }
                        if value & 2 == 2 {
                            cfg.arches.insert("x86_64");
                        }
                        if value & 4 == 4 {
                            cfg.arches.insert("aarch64");
                        }
                    }
                }
            }
            "DeprecatedAttribute" => {
                cfg.add_feature("deprecated");
            }
            _ => {}
        }
    }
}

pub fn type_cfg<'a>(reader: &'a Reader, ty: &Type) -> Cfg<'a> {
    let mut cfg = Cfg::default();
    type_cfg_combine(reader, ty, &mut cfg);
    cfg
}
fn type_cfg_combine<'a>(reader: &'a Reader, ty: &Type, cfg: &mut Cfg<'a>) {
    match ty {
        Type::TypeDef(row, generics) => type_def_cfg_combine(reader, *row, generics, cfg),
        Type::Win32Array(ty, _) => type_cfg_combine(reader, ty, cfg),
        Type::ConstPtr(ty, _) => type_cfg_combine(reader, ty, cfg),
        Type::MutPtr(ty, _) => type_cfg_combine(reader, ty, cfg),
        Type::WinrtArray(ty) => type_cfg_combine(reader, ty, cfg),
        Type::WinrtArrayRef(ty) => type_cfg_combine(reader, ty, cfg),
        ty => _ = cfg.core_types.insert(ty.clone()),
    }
}
