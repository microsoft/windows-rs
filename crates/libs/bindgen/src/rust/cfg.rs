use super::*;
use metadata::{AsRow, HasAttributes};

#[derive(Default, Clone)]
pub struct Cfg {
    pub types: std::collections::BTreeMap<&'static str, std::collections::BTreeSet<metadata::TypeDef>>,
    pub core_types: std::collections::BTreeSet<metadata::Type>,
    pub arches: std::collections::BTreeSet<&'static str>,
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

pub fn field_cfg(row: metadata::Field) -> Cfg {
    let mut cfg = Cfg::default();
    field_cfg_combine(row, None, &mut cfg);
    cfg
}
fn field_cfg_combine(row: metadata::Field, enclosing: Option<metadata::TypeDef>, cfg: &mut Cfg) {
    type_cfg_combine(&row.ty(enclosing), cfg)
}

pub fn type_def_cfg(row: metadata::TypeDef, generics: &[metadata::Type]) -> Cfg {
    let mut cfg = Cfg::default();
    type_def_cfg_combine(row, generics, &mut cfg);
    cfg_add_attributes(&mut cfg, row);
    cfg
}
pub fn type_def_cfg_impl(def: metadata::TypeDef, generics: &[metadata::Type]) -> Cfg {
    let mut cfg = Cfg { implement: true, ..Default::default() };

    fn combine(def: metadata::TypeDef, generics: &[metadata::Type], cfg: &mut Cfg) {
        type_def_cfg_combine(def, generics, cfg);

        for method in def.methods() {
            signature_cfg_combine(&method.signature(generics), cfg);
        }
    }

    combine(def, generics, &mut cfg);

    for interface in metadata::type_interfaces(&metadata::Type::TypeDef(def, generics.to_vec())) {
        if let metadata::Type::TypeDef(def, generics) = interface.ty {
            combine(def, &generics, &mut cfg);
        }
    }

    cfg_add_attributes(&mut cfg, def);
    cfg
}
pub fn type_def_cfg_combine(row: metadata::TypeDef, generics: &[metadata::Type], cfg: &mut Cfg) {
    let type_name = row.type_name();

    for generic in generics {
        type_cfg_combine(generic, cfg);
    }

    if cfg.types.entry(type_name.namespace).or_default().insert(row) {
        match row.kind() {
            metadata::TypeKind::Class => {
                if let Some(default_interface) = metadata::type_def_default_interface(row) {
                    type_cfg_combine(&default_interface, cfg);
                }
            }
            metadata::TypeKind::Interface => {
                if !row.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
                    for def in metadata::type_def_vtables(row) {
                        if let metadata::Type::TypeDef(def, _) = def {
                            cfg.add_feature(def.namespace());
                        }
                    }
                }
            }
            metadata::TypeKind::Struct => {
                row.fields().for_each(|field| field_cfg_combine(field, Some(row), cfg));
                if !type_name.namespace.is_empty() {
                    for def in row.reader().get_type_def(type_name.namespace, type_name.name) {
                        if def != row {
                            type_def_cfg_combine(def, &[], cfg);
                        }
                    }
                }
            }
            metadata::TypeKind::Delegate => signature_cfg_combine(&metadata::type_def_invoke_method(row).signature(generics), cfg),
            _ => {}
        }
    }
}

pub fn signature_cfg(method: metadata::MethodDef) -> Cfg {
    let mut cfg = Cfg::default();
    signature_cfg_combine(&method.signature(&[]), &mut cfg);
    cfg_add_attributes(&mut cfg, method);
    cfg
}
fn signature_cfg_combine(signature: &metadata::MethodDefSig, cfg: &mut Cfg) {
    type_cfg_combine(&signature.return_type, cfg);
    signature.params.iter().for_each(|param| type_cfg_combine(param, cfg));
}

fn cfg_add_attributes<R: AsRow + Into<metadata::HasAttribute>>(cfg: &mut Cfg, row: R) {
    for attribute in row.attributes() {
        match attribute.name() {
            "SupportedArchitectureAttribute" => {
                if let Some((_, metadata::Value::EnumDef(_, value))) = attribute.args().first() {
                    if let metadata::Value::I32(value) = **value {
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

pub fn type_cfg(ty: &metadata::Type) -> Cfg {
    let mut cfg = Cfg::default();
    type_cfg_combine(ty, &mut cfg);
    cfg
}

fn type_cfg_combine(ty: &metadata::Type, cfg: &mut Cfg) {
    match ty {
        metadata::Type::TypeDef(row, generics) => type_def_cfg_combine(*row, generics, cfg),
        metadata::Type::Win32Array(ty, _) => type_cfg_combine(ty, cfg),
        metadata::Type::ConstPtr(ty, _) => type_cfg_combine(ty, cfg),
        metadata::Type::MutPtr(ty, _) => type_cfg_combine(ty, cfg),
        metadata::Type::WinrtArray(ty) => type_cfg_combine(ty, cfg),
        metadata::Type::WinrtArrayRef(ty) => type_cfg_combine(ty, cfg),
        ty => _ = cfg.core_types.insert(ty.clone()),
    }
}
