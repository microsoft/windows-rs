use super::*;

#[derive(Default, Clone)]
pub struct Cfg {
    pub types: BTreeMap<&'static str, BTreeSet<TypeDef>>,
    pub core_types: BTreeSet<Type>,
    pub arches: BTreeSet<&'static str>,
    pub implement: bool,
}

impl Cfg {
    pub fn add_feature(&mut self, feature: &'static str) {
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

pub fn field_cfg(row: Field) -> Cfg {
    let mut cfg = Cfg::default();
    field_cfg_combine(row, None, &mut cfg);
    cfg
}
fn field_cfg_combine(row: Field, enclosing: Option<TypeDef>, cfg: &mut Cfg) {
    type_cfg_combine(&row.ty(enclosing), cfg)
}

pub fn type_def_cfg(row: TypeDef, generics: &[Type]) -> Cfg {
    let mut cfg = Cfg::default();
    type_def_cfg_combine(row, generics, &mut cfg);
    cfg_add_attributes(&mut cfg, row);
    cfg
}
pub fn type_def_cfg_impl(def: TypeDef, generics: &[Type]) -> Cfg {
    let mut cfg = Cfg { implement: true, ..Default::default() };

    fn combine(def: TypeDef, generics: &[Type], cfg: &mut Cfg) {
        type_def_cfg_combine(def, generics, cfg);

        for method in def.methods() {
            signature_cfg_combine(&method.signature(generics), cfg);
        }
    }

    combine(def, generics, &mut cfg);

    for interface in type_interfaces(&Type::TypeDef(def, generics.to_vec())) {
        if let Type::TypeDef(def, generics) = interface.ty {
            combine(def, &generics, &mut cfg);
        }
    }

    cfg_add_attributes(&mut cfg, def);
    cfg
}
pub fn type_def_cfg_combine(row: TypeDef, generics: &[Type], cfg: &mut Cfg) {
    let type_name = row.type_name();

    for generic in generics {
        type_cfg_combine(generic, cfg);
    }

    if cfg.types.entry(type_name.namespace).or_default().insert(row) {
        match row.kind() {
            TypeKind::Class => {
                if let Some(default_interface) = type_def_default_interface(row) {
                    type_cfg_combine(&default_interface, cfg);
                }
            }
            TypeKind::Interface => {
                if !row.flags().contains(TypeAttributes::WindowsRuntime) {
                    for def in type_def_vtables(row) {
                        if let Type::TypeDef(def, _) = def {
                            cfg.add_feature(def.namespace());
                        }
                    }
                }
            }
            TypeKind::Struct => {
                row.fields().for_each(|field| field_cfg_combine(field, Some(row), cfg));
                if !type_name.namespace.is_empty() {
                    for def in row.reader().get_type_def(type_name.namespace, type_name.name) {
                        if def != row {
                            type_def_cfg_combine(def, &[], cfg);
                        }
                    }
                }
            }
            TypeKind::Delegate => signature_cfg_combine(&type_def_invoke_method(row).signature(generics), cfg),
            _ => {}
        }
    }
}

pub fn signature_cfg(method: MethodDef) -> Cfg {
    let mut cfg = Cfg::default();
    signature_cfg_combine(&method.signature(&[]), &mut cfg);
    cfg_add_attributes(&mut cfg, method);
    cfg
}
fn signature_cfg_combine(signature: &MethodDefSig, cfg: &mut Cfg) {
    type_cfg_combine(&signature.return_type, cfg);
    signature.params.iter().for_each(|param| type_cfg_combine(param, cfg));
}

fn cfg_add_attributes<R: AsRow + Into<HasAttribute>>(cfg: &mut Cfg, row: R) {
    for attribute in row.attributes() {
        match attribute.name() {
            "SupportedArchitectureAttribute" => {
                if let Some((_, Value::EnumDef(_, value))) = attribute.args().first() {
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

pub fn type_cfg(ty: &Type) -> Cfg {
    let mut cfg = Cfg::default();
    type_cfg_combine(ty, &mut cfg);
    cfg
}

fn type_cfg_combine(ty: &Type, cfg: &mut Cfg) {
    match ty {
        Type::TypeDef(row, generics) => type_def_cfg_combine(*row, generics, cfg),
        Type::Win32Array(ty, _) => type_cfg_combine(ty, cfg),
        Type::ConstPtr(ty, _) => type_cfg_combine(ty, cfg),
        Type::MutPtr(ty, _) => type_cfg_combine(ty, cfg),
        Type::WinrtArray(ty) => type_cfg_combine(ty, cfg),
        Type::WinrtArrayRef(ty) => type_cfg_combine(ty, cfg),
        ty => _ = cfg.core_types.insert(ty.clone()),
    }
}
