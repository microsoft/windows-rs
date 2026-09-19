use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const SCHEMA: &str = "crates/tools/reactor2/src/schema.toml";
const OUTPUT: &str = "crates/libs/reactor2/src/generated.rs";
const WINMD: &str = "crates/tools/reactor/winmd";
const BINDINGS_FILTER: &str = "crates/tools/reactor2/src/bindings.txt";
const BINDINGS_OUTPUT: &str = "crates/libs/reactor2/src/native/bindings.rs";

#[derive(Deserialize)]
struct Schema {
    objects: Vec<Object>,
}

#[derive(Deserialize)]
struct Object {
    name: String,
    category: String,
    #[serde(default)]
    properties: Vec<Property>,
    #[serde(default)]
    relations: Vec<Relation>,
    #[serde(default)]
    events: Vec<Event>,
}

#[derive(Deserialize)]
struct Property {
    name: String,
    value: String,
}

#[derive(Deserialize)]
struct Relation {
    name: String,
    child: String,
    cardinality: String,
    identity: String,
    realization: String,
}

#[derive(Deserialize)]
struct Event {
    name: String,
    value: String,
}

fn main() {
    let source = fs::read_to_string(workspace_path(SCHEMA)).unwrap();
    let schema: Schema = toml::from_str(&source).unwrap();
    validate(&schema);
    let generated = rustfmt(&generate(&schema));
    let output = workspace_path(OUTPUT);
    if fs::read_to_string(&output).ok().as_deref() != Some(&generated) {
        fs::write(output, generated).unwrap();
    }
    windows_bindgen::builder()
        .input(workspace_path(WINMD))
        .input_default()
        .output(workspace_path(BINDINGS_OUTPUT))
        .minimal()
        .dead_code()
        .flat()
        .filter_file(workspace_path(BINDINGS_FILTER))
        .write();
}

fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join(path)
}

fn validate(schema: &Schema) {
    let mut objects = BTreeSet::new();
    let mut property_types = BTreeMap::new();
    let categories = ["Visual", "Structural", "Data"];
    let cardinalities = ["One", "Many"];
    let identities = ["Positional", "Keyed"];
    let realizations = ["Owned", "Structural", "Container"];

    for object in &schema.objects {
        assert_identifier(&object.name);
        assert!(objects.insert(object.name.as_str()), "duplicate object");
        assert!(categories.contains(&object.category.as_str()));
        let mut properties = BTreeSet::new();
        for property in &object.properties {
            assert_identifier(&property.name);
            assert!(
                properties.insert(property.name.as_str()),
                "duplicate property"
            );
            assert!(matches!(property.value.as_str(), "String" | "Bool"));
            if let Some(previous) = property_types.insert(&property.name, &property.value) {
                assert_eq!(
                    previous, &property.value,
                    "conflicting property value types"
                );
            }
        }
        let mut relations = BTreeSet::new();
        for relation in &object.relations {
            assert_identifier(&relation.name);
            assert!(
                relations.insert(relation.name.as_str()),
                "duplicate relation"
            );
            assert!(categories.contains(&relation.child.as_str()));
            assert!(cardinalities.contains(&relation.cardinality.as_str()));
            assert!(identities.contains(&relation.identity.as_str()));
            assert!(realizations.contains(&relation.realization.as_str()));
            assert!(
                relation.cardinality != "One" || relation.identity == "Positional",
                "single relations must be positional"
            );
            match relation.realization.as_str() {
                "Owned" => assert_eq!(relation.child, "Visual"),
                "Structural" => assert_eq!(relation.child, "Structural"),
                "Container" => assert_eq!(relation.child, "Data"),
                _ => unreachable!(),
            }
        }
        let mut events = BTreeSet::new();
        for event in &object.events {
            assert_identifier(&event.name);
            assert!(events.insert(event.name.as_str()), "duplicate event");
            assert_eq!(event.value, "String");
        }
    }
}

fn assert_identifier(value: &str) {
    let mut characters = value.chars();
    assert!(
        characters
            .next()
            .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
            && characters.all(|character| character == '_' || character.is_ascii_alphanumeric()),
        "invalid Rust identifier: {value}"
    );
    assert!(
        !matches!(
            value,
            "as" | "break"
                | "const"
                | "continue"
                | "crate"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
        ),
        "Rust keyword cannot be used as an identifier: {value}"
    );
}

fn generate(schema: &Schema) -> String {
    let mut properties = BTreeMap::new();
    let mut relations = BTreeMap::new();
    let mut events = BTreeMap::new();
    for object in &schema.objects {
        for property in &object.properties {
            properties
                .entry(property.name.as_str())
                .or_insert(property.value.as_str());
        }
        for relation in &object.relations {
            relations.insert(relation.name.as_str(), ());
        }
        for event in &object.events {
            events
                .entry(event.name.as_str())
                .or_insert(event.value.as_str());
        }
    }

    let mut output = String::from("// This file is generated by tool-reactor2.\n");
    emit_enum(
        &mut output,
        "ObjectType",
        schema.objects.iter().map(|object| object.name.as_str()),
    );
    emit_enum(&mut output, "PropertyId", properties.keys().copied());
    emit_enum(&mut output, "RelationId", relations.keys().copied());
    emit_enum(&mut output, "EventId", events.keys().copied());
    output.push_str(
        "#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum ObjectCategory { Visual, Structural, Data }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum Cardinality { One, Many }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum Identity { Positional, Keyed }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum Realization { Owned, Structural, Container }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct PropertyContract { pub id: PropertyId, pub value: ValueType }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct EventContract { pub id: EventId, pub value: ValueType }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum ValueType { String, Bool }\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub struct RelationContract {\n\
             pub id: RelationId,\n\
             pub child: ObjectCategory,\n\
             pub cardinality: Cardinality,\n\
             pub identity: Identity,\n\
             pub realization: Realization,\n\
         }\n",
    );

    output.push_str("pub fn object_category(kind: ObjectType) -> ObjectCategory { match kind {\n");
    for object in &schema.objects {
        output.push_str(&format!(
            "ObjectType::{} => ObjectCategory::{},\n",
            object.name, object.category
        ));
    }
    output.push_str("} }\n");

    output.push_str(
        "pub fn event_contracts(kind: ObjectType) -> &'static [EventContract] { match kind {\n",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{} => &[", object.name));
        for event in &object.events {
            output.push_str(&format!(
                "EventContract {{ id: EventId::{}, value: ValueType::{} }},",
                event.name, event.value
            ));
        }
        output.push_str("],\n");
    }
    output.push_str("} }\n");

    output.push_str(
        "pub fn property_contracts(kind: ObjectType) -> &'static [PropertyContract] { match kind {\n",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{} => &[", object.name));
        for property in &object.properties {
            output.push_str(&format!(
                "PropertyContract {{ id: PropertyId::{}, value: ValueType::{} }},",
                property.name, property.value
            ));
        }
        output.push_str("],\n");
    }
    output.push_str("} }\n");

    output.push_str(
        "pub fn relation_contracts(kind: ObjectType) -> &'static [RelationContract] { match kind {\n",
    );
    for object in &schema.objects {
        output.push_str(&format!("ObjectType::{} => &[", object.name));
        for relation in &object.relations {
            output.push_str(&format!(
                "RelationContract {{ id: RelationId::{}, child: ObjectCategory::{}, \
                 cardinality: Cardinality::{}, identity: Identity::{}, \
                 realization: Realization::{} }},",
                relation.name,
                relation.child,
                relation.cardinality,
                relation.identity,
                relation.realization
            ));
        }
        output.push_str("],\n");
    }
    output.push_str("} }\n");
    output
}

fn emit_enum<'a>(output: &mut String, name: &str, values: impl Iterator<Item = &'a str>) {
    output.push_str(&format!(
        "#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]\n\
         pub enum {name} {{\n"
    ));
    for value in values {
        output.push_str(value);
        output.push_str(",\n");
    }
    output.push_str("}\n");
}

fn rustfmt(source: &str) -> String {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    std::io::Write::write_all(child.stdin.as_mut().unwrap(), source.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn checked_output_is_current() {
    let source = fs::read_to_string(workspace_path(SCHEMA)).unwrap();
    let schema: Schema = toml::from_str(&source).unwrap();
    validate(&schema);
    assert_eq!(
        fs::read_to_string(workspace_path(OUTPUT)).unwrap(),
        rustfmt(&generate(&schema))
    );
}
