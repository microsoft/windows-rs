/// Generates `reactor_generated.txt` — additional control-specific binding
/// requirements derived from the TOML + metadata that are NOT already in
/// `reactor_base.txt`.
///
/// This file is consumed alongside `reactor_base.txt` by `tool_bindings`
/// (via tool_reactor_codegen). It only contains entries that the base file
/// doesn't already provide.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::metadata::MetadataResolver;
use crate::schema::*;

/// Generate reactor_generated.txt content — only entries not in base.
pub fn generate(controls: &[Control], resolver: &MetadataResolver, base_path: &Path) -> String {
    // Parse base file to know what's already covered
    let base_content = std::fs::read_to_string(base_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", base_path.display()));

    let mut base_iface_methods: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for line in base_content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((iface, methods_part)) = line.split_once("::{") {
            let methods_part = methods_part.trim_end_matches('}');
            let methods: BTreeSet<String> = methods_part
                .split(',')
                .map(|m| m.trim().to_string())
                .filter(|m| !m.is_empty())
                .collect();
            base_iface_methods
                .entry(iface.to_string())
                .or_default()
                .extend(methods);
        }
    }

    // Collect what the TOML needs
    let mut needed: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for ctrl in controls {
        let ns = ctrl
            .namespace
            .as_deref()
            .unwrap_or("Microsoft.UI.Xaml.Controls");

        // CreateInstance for each control class
        let class_path = format!("{ns}.{}", ctrl.handle());
        needed
            .entry(class_path)
            .or_default()
            .insert("CreateInstance".to_string());

        // Interface methods from metadata resolution
        for p in &ctrl.prop {
            let method_name = match p.setter() {
                SetterKind::Method { method } => Some(method),
                SetterKind::MethodOptional { method } => Some(method),
                SetterKind::MethodIReference { method } => Some(method),
                SetterKind::MethodTextblock { method } => Some(method),
                SetterKind::MethodEnumMap { setter } => Some(setter.method()),
                SetterKind::Custom => None,
            };
            if let Some(method) = method_name
                && let Some(iface_ref) = resolver.resolve(ctrl.handle(), method)
            {
                needed
                    .entry(iface_ref.full_path())
                    .or_default()
                    .insert(method.to_string());
            }
        }
    }

    // Subtract what base already provides
    let mut output: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for (iface, methods) in &needed {
        let base_methods = base_iface_methods.get(iface.as_str());
        let new_methods: BTreeSet<String> = methods
            .iter()
            .filter(|m| base_methods.is_none_or(|bm| !bm.contains(m.as_str())))
            .cloned()
            .collect();
        if !new_methods.is_empty() {
            output.insert(iface.clone(), new_methods);
        }
    }

    // Build sorted output
    let mut lines: Vec<String> = Vec::new();
    for (iface, methods) in &output {
        let methods_str: Vec<&str> = methods.iter().map(|s| s.as_str()).collect();
        lines.push(format!("{iface}::{{{}}}", methods_str.join(", ")));
    }

    if lines.is_empty() {
        String::new()
    } else {
        lines.join("\n") + "\n"
    }
}
