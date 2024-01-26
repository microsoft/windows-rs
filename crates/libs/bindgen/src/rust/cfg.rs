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

pub fn field_cfg(writer: &Writer, row: metadata::Field) -> Cfg {
    let mut cfg = Cfg::default();
    field_cfg_combine(writer, row, None, &mut cfg);
    cfg
}
fn field_cfg_combine(writer: &Writer, row: metadata::Field, enclosing: Option<metadata::TypeDef>, cfg: &mut Cfg) {
    type_cfg_combine(writer, &row.ty(enclosing), cfg)
}

pub fn type_def_cfg(writer: &Writer, row: metadata::TypeDef, generics: &[metadata::Type]) -> Cfg {
    let mut cfg = Cfg::default();
    type_def_cfg_combine(writer, row, generics, &mut cfg);
    cfg_add_attributes(&mut cfg, row);
    cfg
}
pub fn type_def_cfg_impl(writer: &Writer, def: metadata::TypeDef, generics: &[metadata::Type]) -> Cfg {
    let mut cfg = Cfg { implement: true, ..Default::default() };

    fn combine(writer: &Writer, def: metadata::TypeDef, generics: &[metadata::Type], cfg: &mut Cfg) {
        type_def_cfg_combine(writer, def, generics, cfg);

        for method in def.methods() {
            signature_cfg_combine(writer, &method.signature(generics), cfg);
        }
    }

    combine(writer, def, generics, &mut cfg);

    for interface in metadata::type_interfaces(&metadata::Type::TypeDef(def, generics.to_vec())) {
        if let metadata::Type::TypeDef(def, generics) = interface.ty {
            combine(writer, def, &generics, &mut cfg);
        }
    }

    cfg_add_attributes(&mut cfg, def);
    cfg
}
pub fn type_def_cfg_combine(writer: &Writer, row: metadata::TypeDef, generics: &[metadata::Type], cfg: &mut Cfg) {
    let type_kind = row.kind();

    if writer.sys && type_kind == metadata::TypeKind::Interface {
        return;
    }

    let type_name = row.type_name();

    for generic in generics {
        type_cfg_combine(writer, generic, cfg);
    }

    if cfg.types.entry(type_name.namespace).or_default().insert(row) {
        match type_kind {
            metadata::TypeKind::Class => {
                if let Some(default_interface) = metadata::type_def_default_interface(row) {
                    type_cfg_combine(writer, &default_interface, cfg);
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
                row.fields().for_each(|field| field_cfg_combine(writer, field, Some(row), cfg));
                if !type_name.namespace.is_empty() {
                    for def in row.reader().get_type_def(type_name.namespace, type_name.name) {
                        if def != row {
                            type_def_cfg_combine(writer, def, &[], cfg);
                        }
                    }
                }
            }
            metadata::TypeKind::Delegate => signature_cfg_combine(writer, &metadata::type_def_invoke_method(row).signature(generics), cfg),
            _ => {}
        }
    }
}

pub fn signature_cfg(writer: &Writer, method: metadata::MethodDef) -> Cfg {
    let mut cfg = Cfg::default();
    signature_cfg_combine(writer, &method.signature(&[]), &mut cfg);
    cfg_add_attributes(&mut cfg, method);
    cfg
}
fn signature_cfg_combine(writer: &Writer, signature: &metadata::MethodDefSig, cfg: &mut Cfg) {
    type_cfg_combine(writer, &signature.return_type, cfg);
    signature.params.iter().for_each(|param| type_cfg_combine(writer, param, cfg));
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

pub fn type_cfg(writer: &Writer, ty: &metadata::Type) -> Cfg {
    let mut cfg = Cfg::default();
    type_cfg_combine(writer, ty, &mut cfg);
    cfg
}

fn type_cfg_combine(writer: &Writer, ty: &metadata::Type, cfg: &mut Cfg) {
    match ty {
        metadata::Type::TypeDef(row, generics) => type_def_cfg_combine(writer, *row, generics, cfg),
        metadata::Type::Win32Array(ty, _) => type_cfg_combine(writer, ty, cfg),
        metadata::Type::ConstPtr(ty, _) => type_cfg_combine(writer, ty, cfg),
        metadata::Type::MutPtr(ty, _) => type_cfg_combine(writer, ty, cfg),
        metadata::Type::WinrtArray(ty) => type_cfg_combine(writer, ty, cfg),
        metadata::Type::WinrtArrayRef(ty) => type_cfg_combine(writer, ty, cfg),
        ty => _ = cfg.core_types.insert(ty.clone()),
    }
}
