use super::*;
use rust::cfg::*;
use serde::Serialize;
use std::collections::BTreeMap;
use windows_metadata::Item;

#[derive(Default, Serialize)]
struct Index {
    namespace_map: Vec<String>,
    feature_map: Vec<String>,
    namespaces: BTreeMap<usize, Vec<IndexItem>>,
}

#[derive(Default, Serialize)]
struct IndexItem {
    name: String,
    features: Vec<usize>,
}

pub fn gen_index(writer: &Writer) -> String {
    let mut feature_index = Index { ..Default::default() };

    for namespace in writer.reader.namespaces() {
        let namespace_idx = match feature_index.namespace_map.iter().position(|ns| ns == namespace) {
            Some(idx) => idx,
            None => {
                feature_index.namespace_map.push(namespace.to_string());
                feature_index.namespace_map.len() - 1
            }
        };

        for item in writer.reader.namespace_items(namespace) {
            let mut index_item = IndexItem { ..Default::default() };
            let mut cfg = Cfg::default();
            cfg.add_feature(namespace);

            cfg = match item {
                Item::Type(def) => {
                    index_item.name = def.name().to_string();
                    cfg.union(type_def_cfg(writer, def, &[]))
                }
                Item::Const(field) => {
                    index_item.name = field.name().to_string();
                    cfg.union(field_cfg(writer, field))
                }
                Item::Fn(method, _) => {
                    index_item.name = method.name().to_string();
                    cfg.union(signature_cfg(writer, method))
                }
            };

            let cfg_features = cfg_features(&cfg);
            index_item.features = cfg_features
                .iter()
                .map(|feature| match feature_index.feature_map.iter().position(|f| f == feature) {
                    Some(idx) => idx,
                    None => {
                        feature_index.feature_map.push(feature.to_string());
                        feature_index.feature_map.len() - 1
                    }
                })
                .collect();

            feature_index.namespaces.entry(namespace_idx).or_default().push(index_item);
        }
    }

    serde_json::to_string(&feature_index).unwrap()
}
