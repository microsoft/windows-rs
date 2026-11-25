include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

fn filter_contract_output(case: &str, style: &str, filters: &[&str]) -> String {
    let scratch = std::path::Path::new(env!("OUT_DIR"))
        .join("filter-contracts")
        .join(case)
        .join(style);
    std::fs::create_dir_all(&scratch).unwrap();
    let metadata = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/filter_contracts.rdl")
        .output(&metadata)
        .write()
        .unwrap();
    let output = scratch.join("bindings.rs");
    let mut builder = windows_bindgen::builder();
    builder.input(metadata).filters(filters).output(&output);
    match style {
        "minimal" => {}
        "full" => {
            builder.full();
        }
        "sys" => {
            builder.sys();
        }
        _ => unreachable!(),
    }
    builder.write();
    std::fs::read_to_string(output).unwrap()
}

#[test]
fn selected_variant_across_architectures() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("selected-arch-variant");
    std::fs::create_dir_all(&scratch).unwrap();
    let metadata = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/arch_enum_sys.rdl")
        .output(&metadata)
        .write()
        .unwrap();
    let output = scratch.join("bindings.rs");
    windows_bindgen::builder()
        .input(metadata)
        .filter("Test.ArchEnum::Arm64Only")
        .sys()
        .output(&output)
        .write();
    let source = std::fs::read_to_string(output).unwrap();
    assert!(source.contains("pub const Arm64Only:"));
    assert!(!source.contains("pub const First:"));
    assert!(!source.contains("pub const X64Only:"));
}

#[test]
fn namespace_exclusions_match_explicit_seeds() {
    for style in ["minimal", "full", "sys"] {
        let namespace = filter_contract_output(
            "namespace-excluded-dependency",
            style,
            &["NamespaceSeeds", "!Other.Token"],
        );
        let explicit = filter_contract_output(
            "explicit-excluded-dependency",
            style,
            &["NamespaceSeeds.Record", "!Other.Token"],
        );
        assert!(explicit.contains("pub struct Record"));
        assert!(!explicit.contains("pub struct Token"));
        assert_eq!(namespace, explicit, "{style}");
    }
}

#[test]
fn filter_preserves_slot_identity() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("filtered-slots");
    std::fs::create_dir_all(&scratch).unwrap();
    let core = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libs")
        .join("core");
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!(
            "[package]\nname = \"filtered-slots\"\nversion = \"0.0.0\"\n\
             edition = \"2024\"\n[workspace]\n\
             [[bin]]\nname = \"filtered-slots\"\npath = \"main.rs\"\n\
             [dependencies]\nwindows-core = {{ path = '{}', features = ['proc-macros'] }}\n",
            core.display()
        ),
    )
    .unwrap();
    std::fs::write(
        scratch.join("complete.rs"),
        filter_contract_output("complete-slots", "full", &["Slots"]),
    )
    .unwrap();
    let mut failures = Vec::new();
    for style in ["minimal", "full"] {
        std::fs::write(
            scratch.join("selected.rs"),
            filter_contract_output(
                "selected-slots",
                style,
                &[
                    "Slots.IProbe::Value",
                    "!Slots.IProbe::get_Value",
                    "Slots.NativeProbe::Value",
                    "!Slots.NativeProbe::get_Value",
                    "Slots.Widget::Value",
                    "!Slots.Widget::get_Value",
                    "Slots.IRequired",
                    "Slots.FactoryWidget::{Create,Value}",
                    "!Slots.FactoryWidget::{get_Create,get_Value}",
                ],
            ),
        )
        .unwrap();
        let mut source = String::from(
            r#"#![allow(dead_code, nonstandard_style)]
use windows_core::{Interface, Result, implement};
mod complete;
mod selected;

#[implement(complete::IProbe, complete::IRequired)]
struct Probe;
impl complete::IProbe_Impl for Probe_Impl {
    fn Value(&self) -> Result<i32> { Ok(1) }
    fn Value2(&self) -> Result<i32> { Ok(2) }
}
impl complete::IRequired_Impl for Probe_Impl {}

struct Native;
impl complete::NativeProbe_Impl for Native {
    fn Value(&self) -> i32 { 1 }
    fn Value2(&self) -> i32 { 2 }
}

fn main() {
    let complete: complete::IProbe = Probe.into();
    assert_eq!(complete.Value().unwrap(), 1);
    assert_eq!(complete.Value2().unwrap(), 2);
    let probe: selected::IProbe = complete.cast().unwrap();
    let widget: selected::Widget = complete.cast().unwrap();
    let native = complete::NativeProbe::new(&Native);
    let raw = native.as_raw();
    // Both projections describe the same vtable; the scoped owner outlives the borrow.
    let native = unsafe { selected::NativeProbe::from_raw_borrowed(&raw).unwrap() };
    let values = [probe.Value().unwrap(), widget.Value().unwrap(), unsafe { native.Value() }];
    assert_eq!(values, [2; 3]);
"#,
        );
        if style == "full" {
            source.push_str(
                "let required: selected::IRequired = complete.cast().unwrap();\n\
                 assert_eq!(required.Value().unwrap(), 2);\n",
            );
        }
        source.push_str("}\n");
        std::fs::write(scratch.join("main.rs"), source).unwrap();
        let output = std::process::Command::new("cargo")
            .args(["run", "--quiet", "--offline", "--manifest-path"])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(scratch.join("target"))
            .output()
            .unwrap();
        if !output.status.success() {
            failures.push(format!(
                "{style}: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn class_instance_conversions() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("instance-conversions");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/narrow_class_static.rdl")
        .input("input/minimal_compose_target.rdl")
        .input("input/minimal_required_hierarchy.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let core = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libs")
        .join("core");
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!(
            "[package]\nname = \"instance-conversions\"\nversion = \"0.0.0\"\n\
             edition = \"2024\"\n[workspace]\n[lib]\npath = \"lib.rs\"\n\
             [dependencies]\nwindows-core = {{ path = '{}' }}\n",
            core.display()
        ),
    )
    .unwrap();
    let check = || {
        std::process::Command::new("cargo")
            .args(["check", "--quiet", "--offline", "--manifest-path"])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(scratch.join("target"))
            .output()
            .unwrap()
    };
    let valid = r#"#![allow(dead_code, nonstandard_style)]
mod bindings;
use bindings::*;
use windows_core::{Interface, Param, Result};

fn parameter<I: Interface, P: Param<I>>(_: P) {}

pub fn instances(widget: &Widget, derived: &Derived, explicit: &Explicit) {
    parameter::<IWidget, _>(widget);
    parameter::<IDerived, _>(derived);
    parameter::<IOverrides, _>(derived);
    parameter::<Base, _>(derived);
    parameter::<IClose, _>(explicit);
}

pub fn factories(factory: &IWidgetFactory, statics: &IWidgetStatics, composed: &IDerivedFactory) {
    parameter::<IWidgetFactory, _>(factory);
    parameter::<IWidgetStatics, _>(statics);
    parameter::<IDerivedFactory, _>(composed);
}

pub fn entry_points() {
    let _: fn(i32) -> Result<Widget> = Widget::Create;
    let _: fn() -> Result<Widget> = Widget::GetDefault;
    let _: fn() -> Result<Derived> = Derived::new;
}

pub fn explicit_query(widget: &Widget) -> Result<IWidgetFactory> {
    widget.cast()
}
"#;
    let mut failures = Vec::new();
    for full in [false, true] {
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Test")
            .output(scratch.join("bindings.rs"));
        if full {
            builder.full();
        }
        builder.write();
        std::fs::write(scratch.join("lib.rs"), valid).unwrap();
        let output = check();
        assert!(
            output.status.success(),
            "full={full}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        for (class, interface) in [
            ("Widget", "IWidgetFactory"),
            ("Widget", "IWidgetStatics"),
            ("Base", "IBaseFactory"),
            ("Derived", "IDerivedFactory"),
        ] {
            std::fs::write(
                scratch.join("lib.rs"),
                format!(
                    "{valid}\npub fn invalid(value: &{class}) {{ parameter::<{interface}, _>(value); }}\n"
                ),
            )
            .unwrap();
            let output = check();
            let error = String::from_utf8_lossy(&output.stderr);
            if output.status.success() {
                failures.push(format!("full={full}: {class} accepted as {interface}"));
            } else {
                assert!(
                    error.contains("E0277")
                        && error.contains("CanInto")
                        && error.contains(interface),
                    "full={full}: {error}"
                );
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn filter_mixed_namespaces_preserve_precise_methods() {
    for style in ["minimal", "full", "sys"] {
        let filters = [
            "Test.ISource::GetResult",
            "Native.INativeSource::GetNativeResult",
        ];
        let precise = filter_contract_output("mixed", style, &filters);
        let mixed = filter_contract_output("mixed", style, &[&filters[..], &["Other"]].concat());
        for output in [&precise, &mixed] {
            assert!(output.contains("pub GetResult:"), "{style}\n{output}");
            assert!(output.contains("pub GetNativeResult:"), "{style}\n{output}");
            assert!(!output.contains("pub Read:"), "{style}\n{output}");
            assert!(!output.contains("pub NativeRead:"), "{style}\n{output}");
        }
        assert!(mixed.contains("pub struct Token"));
        let excluded = filter_contract_output(
            "mixed",
            style,
            &[&filters[..], &["Other", "!Test.ISource"]].concat(),
        );
        assert!(!excluded.contains("pub GetResult:"), "{style}\n{excluded}");
    }
}

#[test]
fn filter_enum_exclusions() {
    for style in ["minimal", "full", "sys"] {
        for (name, red, green, blue) in [
            ("Test.Color", "Red", "Green", "Blue"),
            (
                "Native.NativeColor",
                "NativeRed",
                "NativeGreen",
                "NativeBlue",
            ),
        ] {
            for selection in [
                name.to_string(),
                name.rsplit_once('.').unwrap().0.to_string(),
            ] {
                let exclusion = format!("!{name}::{red}");
                let output = filter_contract_output("enums", style, &[&selection, &exclusion]);
                assert!(
                    !output.contains(&format!("pub const {red}:")),
                    "{style}\n{output}"
                );
                for variant in [green, blue] {
                    assert!(
                        output.contains(&format!("pub const {variant}:")),
                        "{style}\n{output}"
                    );
                }
            }
            let selected = format!("{name}::{{{red},{green}}}");
            let exclusion = format!("!{name}::{red}");
            let output = filter_contract_output("enums", style, &[&selected, &exclusion]);
            assert!(
                !output.contains(&format!("pub const {red}:")),
                "{style}\n{output}"
            );
            assert!(
                output.contains(&format!("pub const {green}:")),
                "{style}\n{output}"
            );
            assert!(
                !output.contains(&format!("pub const {blue}:")),
                "{style}\n{output}"
            );
        }
    }
}

#[test]
fn filter_enum_dependencies_respect_exclusions() {
    for style in ["minimal", "full", "sys"] {
        for exclusion in ["!Native.NativeColor::NativeRed", "!Native.NativeRed"] {
            for broad in [false, true] {
                let mut filters = vec![
                    "Native.ApplyColor",
                    "Native.INativeOptions::ApplyColor",
                    "Native.NativeColor",
                    exclusion,
                ];
                if broad {
                    filters.push("Other");
                }
                let output = filter_contract_output("enum-dependencies", style, &filters);
                assert!(
                    !output.contains("pub const NativeRed:"),
                    "{style}\n{output}"
                );
                assert!(
                    output.contains("pub const NativeGreen:"),
                    "{style}\n{output}"
                );
                assert!(output.contains("\"fixture.dll\""), "{style}\n{output}");
                assert!(output.contains("pub ApplyColor:"), "{style}\n{output}");
            }
        }
    }
}

#[test]
fn filter_class_property_exclusions() {
    for style in ["minimal", "full", "sys"] {
        for (owner, exclusion) in [
            ("Test.Widget", "!Test.Widget::Value"),
            ("Test.IWidget", "!Test.IWidget::Value"),
        ] {
            let output = filter_contract_output("properties", style, &[owner, exclusion]);
            assert!(!output.contains("pub get_Value:"), "{style}\n{output}");
            assert!(!output.contains("pub put_Value:"), "{style}\n{output}");
            assert!(output.contains("pub Other:"), "{style}\n{output}");
        }
    }
}

#[test]
fn filter_selection_order() {
    for style in ["minimal", "full", "sys"] {
        for (name, member, omitted) in [
            ("Test.ISource", "GetResult", "IUnused"),
            ("Test.Color", "Red", "pub const Green:"),
            ("Native.NativeColor", "NativeRed", "pub const NativeGreen:"),
        ] {
            let selected = format!("{name}::{member}");
            let whole_first = filter_contract_output("order", style, &[name, &selected]);
            let member_first = filter_contract_output("order", style, &[&selected, name]);
            assert_eq!(whole_first, member_first, "{style}: {name}");
            assert!(!whole_first.contains(omitted), "{style}\n{whole_first}");
        }
    }
}

#[test]
fn filter_bundled_metadata_contracts() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("filter-bundled");
    std::fs::create_dir_all(&scratch).unwrap();
    let output = scratch.join("bindings.rs");
    windows_bindgen::builder()
        .filters([
            "IDXGIDevice::GetAdapter",
            "IDataReader::ReadBuffer",
            "Windows.System.Power",
            "BitmapPixelFormat",
            "!BitmapPixelFormat::Rgba8",
            "Uri",
            "!Uri::AbsoluteUri",
        ])
        .output(&output)
        .write();
    let output = std::fs::read_to_string(output).unwrap();
    assert!(output.contains("pub GetAdapter:"));
    assert!(output.contains("pub ReadBuffer:"));
    assert!(!output.contains("pub const Rgba8:"));
    assert!(output.contains("pub const Bgra8:"));
    assert!(!output.contains(" fn AbsoluteUri("));
}

#[test]
fn class_selection_preserves_shells_and_members() {
    for style in ["minimal", "full"] {
        let whole = filter_contract_output("whole-class", style, &["Slots.FactoryWidget"]);
        assert!(whole.contains(" fn Create("));
        assert!(whole.contains(" fn Value("));
        let shell = filter_contract_output("class-shell", style, &["Slots.FactoryWidget::{}"]);
        assert!(!shell.contains(" fn Create("));
        assert!(!shell.contains(" fn Value("));
        let member =
            filter_contract_output("class-member", style, &["Slots.FactoryWidget::Create"]);
        assert!(member.contains(" fn Create("));
        assert!(!member.contains(" fn Value("));
        let combined = filter_contract_output(
            "class-combined",
            style,
            &["Slots.FactoryWidget", "Slots.FactoryWidget::Create"],
        );
        assert!(combined.contains(" fn Create("));
        assert!(!combined.contains(" fn Value("));
        let reversed = filter_contract_output(
            "class-combined-reversed",
            style,
            &["Slots.FactoryWidget::Create", "Slots.FactoryWidget"],
        );
        assert_eq!(combined, reversed);
        let combined = filter_contract_output(
            "class-interface-combined",
            style,
            &["Test.Widget::Other", "Test.IWidget::get_Value"],
        );
        assert!(combined.contains(" fn Other("));
        assert!(combined.contains(" fn get_Value("));
        let excluded = filter_contract_output(
            "class-excluded-member",
            style,
            &[
                "Slots.FactoryWidget",
                "!Slots.FactoryWidget::{get_Create,Create}",
            ],
        );
        assert!(!excluded.contains(" fn Create("));
        assert!(excluded.contains(" fn Value("));
        let default = filter_contract_output("default-class", style, &["Slots.DefaultWidget"]);
        assert!(default.contains(" fn new("));
        let member = filter_contract_output(
            "default-class-member",
            style,
            &["Slots.DefaultWidget", "Slots.DefaultWidget::Value"],
        );
        assert!(!member.contains(" fn new("));
        let shell =
            filter_contract_output("default-class-shell", style, &["Slots.DefaultWidget::{}"]);
        assert!(!shell.contains(" fn new("));
        let excluded = filter_contract_output(
            "default-class-excluded",
            style,
            &[
                "Slots.DefaultWidget",
                "!Slots.DefaultWidget::CreateInstance",
            ],
        );
        assert!(!excluded.contains(" fn new("));
    }
}

#[test]
fn class_methods_keep_declaring_identity() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("class-method-identity");
    std::fs::create_dir_all(&scratch).unwrap();
    let metadata = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/narrow_iterable_names.rdl")
        .output(&metadata)
        .write()
        .unwrap();
    let output = scratch.join("bindings.rs");
    windows_bindgen::builder()
        .input(metadata)
        .filter("Test.Items::First")
        .full()
        .output(&output)
        .write();
    let source = std::fs::read_to_string(output).unwrap();
    assert!(source.contains("pub fn First(&self) -> windows_core::Result<u32>"));
    assert!(!source.contains("impl IntoIterator for Items"));
}

#[test]
fn whole_class_with_named_implementation() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("whole-class-implementation");
    std::fs::create_dir_all(&scratch).unwrap();
    let metadata = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/filter_contracts.rdl")
        .output(&metadata)
        .write()
        .unwrap();
    windows_bindgen::builder()
        .input(metadata)
        .filter("Test.Widget")
        .implement("Test.IWidget")
        .output(scratch.join("bindings.rs"))
        .write();
}

#[test]
fn derived_class_does_not_select_base_factories() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("derived-class-factories");
    std::fs::create_dir_all(&scratch).unwrap();
    let metadata = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/minimal_compose_target.rdl")
        .output(&metadata)
        .write()
        .unwrap();
    for full in [false, true] {
        let output = scratch.join("bindings.rs");
        let mut builder = windows_bindgen::builder();
        builder
            .input(&metadata)
            .filters(["Test.Derived", "Test.Base::{}"])
            .output(&output);
        if full {
            builder.full();
        }
        builder.write();
        let source = std::fs::read_to_string(output).unwrap();
        assert!(source.contains("IDerivedFactory"));
        assert!(!source.contains("IBaseFactory"));
        assert_eq!(source.matches(" fn new(").count(), 1);
    }
}

#[test]
fn nested_generic_class_iteration() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("nested-generic-class-iteration.rs");
    windows_bindgen::builder()
        .filters([
            "Windows.Foundation.Collections.StringMap",
            "Windows.UI.Composition.ImplicitAnimationCollection::{}",
        ])
        .full()
        .output(&output)
        .write();
    let source = std::fs::read_to_string(output).unwrap();
    assert!(source.contains("impl IntoIterator for StringMap"));
    assert!(source.contains("impl IntoIterator for &StringMap"));
    assert!(!source.contains("impl IntoIterator for ImplicitAnimationCollection"));
}

#[test]
fn reference_class_selection() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("reference-class-selection.rs");
    for (filter, lookup, insert) in [
        ("StringMap", true, true),
        ("StringMap::{}", false, false),
        ("StringMap::Insert", false, true),
    ] {
        windows_bindgen::builder()
            .filter(filter)
            .full()
            .output(&output)
            .write();
        let source = std::fs::read_to_string(&output).unwrap();
        assert_eq!(source.contains(" fn Lookup("), lookup, "{filter}");
        assert_eq!(source.contains(" fn Insert("), insert, "{filter}");
    }
}

#[test]
fn interface_forwarders_respect_selected_hierarchy() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("interface-forwarders.rs");
    for (filter, own_methods, inherited_methods) in [
        ("Windows.Foundation.IAsyncAction::{}", false, false),
        ("Windows.Foundation.IAsyncAction::GetResults", true, false),
        ("Windows.Foundation.IAsyncAction", true, true),
    ] {
        windows_bindgen::builder()
            .filter(filter)
            .full()
            .output(&output)
            .write();
        let source = std::fs::read_to_string(&output).unwrap();
        assert_eq!(source.contains(" fn GetResults("), own_methods, "{filter}");
        for method in ["Status", "Cancel"] {
            assert_eq!(
                source.contains(&format!(" fn {method}(")),
                inherited_methods,
                "{filter}: {method}"
            );
        }
    }
}

#[test]
fn interface_shell_does_not_forward_selected_base_methods() {
    let source = filter_contract_output(
        "interface-shell-forwarders",
        "full",
        &["Slots.IRequired::{}", "Slots.IProbe::Value"],
    );
    assert!(source.contains("impl IProbe {"));
    assert!(source.contains(" fn Value("));
    assert!(!source.contains("impl IRequired {"));
}

#[test]
fn filter_mixed_class_dependencies_compile() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("filter-mixed-class");
    std::fs::create_dir_all(&scratch).unwrap();
    let libs = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libs");
    for package in [false, true] {
        let output = scratch.join(if package { "package" } else { "modules" });
        std::fs::create_dir_all(&output).unwrap();
        let manifest = output.join("Cargo.toml");
        for mixed in [false, true] {
            for full in [true, false] {
                std::fs::write(
                    &manifest,
                    format!(
                        "[package]\nname = \"filter-mixed-class\"\nversion = \"0.0.0\"\n\
                         edition = \"2024\"\n[workspace]\n[lib]\npath = \"lib.rs\"\n\
                         [dependencies]\n\
                         windows-core = {{ path = '{}' }}\n\
                         windows-collections = {{ path = '{}' }}\n\
                         windows-numerics = {{ path = '{}' }}\n\
                         [features]\n# generated features\n",
                        libs.join("core").display(),
                        libs.join("collections").display(),
                        libs.join("numerics").display(),
                    ),
                )
                .unwrap();
                let mut builder = windows_bindgen::builder();
                builder.filter("Uri");
                if mixed {
                    builder
                        .filter("Windows.Foundation.Numerics")
                        .implement_all();
                }
                if package {
                    builder.package().output(&output);
                } else {
                    builder.modules().output(output.join("bindings.rs"));
                }
                if full {
                    builder.full();
                }
                builder.write();
                let modules = if package {
                    "#[path = \"src/Windows/mod.rs\"] mod Windows;"
                } else {
                    "mod bindings; use bindings::Windows;"
                };
                let uri = if full {
                    "&windows_core::HSTRING::from(\"https://example.com\")"
                } else {
                    "\"https://example.com\""
                };
                std::fs::write(
                    output.join("lib.rs"),
                    format!(
                        "#![allow(dead_code, nonstandard_style)]\n{modules}\n\
                         fn create_uri() -> windows_core::Result<Windows::Foundation::Uri> {{\n\
                             Windows::Foundation::Uri::CreateUri({uri})\n\
                         }}\n"
                    ),
                )
                .unwrap();
                let result = std::process::Command::new("cargo")
                    .args([
                        "check",
                        "--quiet",
                        "--offline",
                        "--all-features",
                        "--manifest-path",
                    ])
                    .arg(&manifest)
                    .arg("--target-dir")
                    .arg(scratch.join("target"))
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "package={package} mixed={mixed} full={full}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
    }
}

#[test]
fn narrow_event_delegates_respect_dead_code() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("narrow-event-dead-code");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/narrow_event_minimal.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Test.ISource::Changed")
        .dead_code()
        .output(scratch.join("bindings.rs"))
        .write();
    let core = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libs")
        .join("core");
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!(
            "[package]\nname = \"narrow-event-dead-code\"\nversion = \"0.0.0\"\n\
             edition = \"2024\"\n[workspace]\n[lib]\npath = \"lib.rs\"\n\
             [dependencies]\nwindows-core = {{ path = '{}', default-features = false }}\n",
            core.display()
        ),
    )
    .unwrap();
    std::fs::write(
        scratch.join("lib.rs"),
        r#"#![deny(warnings)]
#![allow(nonstandard_style)]
mod bindings;
pub fn subscribe(source: &bindings::ISource) -> windows_core::Result<windows_core::EventRevoker> {
    source.Changed(|_| {})
}
"#,
    )
    .unwrap();
    let output = std::process::Command::new("cargo")
        .args(["check", "--quiet", "--offline", "--manifest-path"])
        .arg(&manifest)
        .arg("--target-dir")
        .arg(scratch.join("target"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn delegate_thread_safety() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("delegate_thread_safety");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/event.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let core = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libs")
        .join("core");
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!(
            "[package]\nname = \"delegate-thread-safety\"\nversion = \"0.0.0\"\n\
             edition = \"2024\"\n[workspace]\n[lib]\npath = \"lib.rs\"\n\
             [dependencies]\nwindows-core = {{ path = '{}', default-features = false }}\n",
            core.display()
        ),
    )
    .unwrap();
    for minimal in [false, true] {
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Test")
            .output(scratch.join("bindings.rs"));
        if !minimal {
            builder.full();
        }
        builder.write();
        for event in [false, true] {
            for (capture, rejection) in [
                ("core::sync::atomic::AtomicUsize::new(0)", None),
                (
                    "core::cell::Cell::new(0)",
                    Some("cannot be shared between threads safely"),
                ),
                (
                    "std::rc::Rc::new(0)",
                    Some("cannot be shared between threads safely"),
                ),
                (
                    "NotSend(core::marker::PhantomData)",
                    Some("cannot be sent between threads safely"),
                ),
            ] {
                let result = if minimal || event { "" } else { "Ok(())" };
                let parameters = if event {
                    "source: &bindings::Interface"
                } else {
                    ""
                };
                let call = if event {
                    "source.Changed"
                } else {
                    "bindings::Delegate::new"
                };
                std::fs::write(
                    scratch.join("lib.rs"),
                    format!(
                        "#![allow(dead_code, nonstandard_style)]\nmod bindings;\n\
                     struct NotSend(core::marker::PhantomData<*const ()>);\n\
                     unsafe impl Sync for NotSend {{}}\n\
                     pub fn construct({parameters}) {{\n\
                         let capture = {capture};\n\
                         let _ = {call}(move |_| {{\n\
                             let _ = &capture;\n\
                             {result}\n\
                         }});\n\
                     }}\n"
                    ),
                )
                .unwrap();
                let output = std::process::Command::new("cargo")
                    .args(["check", "--quiet", "--offline", "--manifest-path"])
                    .arg(&manifest)
                    .arg("--target-dir")
                    .arg(scratch.join("target"))
                    .output()
                    .unwrap();
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert_eq!(
                    output.status.success(),
                    rejection.is_none(),
                    "minimal={minimal}, event={event}, capture={capture}: {stderr}"
                );
                if let Some(rejection) = rejection {
                    assert!(stderr.contains(rejection), "{stderr}");
                }
            }
        }
    }
}

#[test]
fn minimal_interface_implementation_selection() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("minimal_interface_implementations");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/minimal_interfaces.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let output = scratch.join("bindings.rs");
    for selection in ["none", "selected", "all"] {
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Test")
            .dead_code()
            .output(&output);
        match selection {
            "selected" => {
                builder
                    .implement("Test.Native")
                    .implement("Test.Runtime.RuntimeBase");
            }
            "all" => {
                builder.implement_all();
            }
            _ => {}
        }
        builder.write();
        let source = std::fs::read_to_string(&output).unwrap();
        for name in [
            "NativeBase",
            "NativeDerived",
            "Scoped",
            "RuntimeBase",
            "RuntimeDerived",
        ] {
            let implemented =
                selection == "all" || (selection == "selected" && name != "RuntimeDerived");
            assert_eq!(
                source.contains(&format!("pub trait {name}_Impl")),
                implemented,
                "{selection}: {name}"
            );
            assert!(source.contains(&format!("pub struct {name}_Vtbl")));
        }
    }
}

#[test]
fn interface_helpers_respect_dead_code() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("interface_helpers_dead_code");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/minimal_interfaces.rdl")
        .input("input/associated_enum_methods.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let core = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\libs\\core");
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!(
            "[package]\nname = \"interface-helpers-dead-code\"\nversion = \"0.0.0\"\n\
             edition = \"2024\"\n[workspace]\n[lib]\npath = \"lib.rs\"\n\
             [dependencies]\nwindows-core = {{ path = '{}', features = ['proc-macros'] }}\n",
            core.display()
        ),
    )
    .unwrap();
    std::fs::write(
        scratch.join("lib.rs"),
        r#"#![deny(warnings)]
#![allow(nonstandard_style)]
mod bindings;
pub fn native(base: &bindings::NativeBase, derived: &bindings::NativeDerived) -> i32 {
    unsafe { base.BaseValue().value + derived.DerivedValue() }
}
pub fn scoped(value: &bindings::Scoped) -> i32 {
    unsafe { value.Value() }
}
pub fn runtime(
    base: &bindings::RuntimeBase,
    derived: &bindings::RuntimeDerived,
) -> windows_core::Result<i32> {
    Ok(base.BaseValue()? + derived.DerivedValue()?)
}
"#,
    )
    .unwrap();
    let compile = || {
        std::process::Command::new("cargo")
            .args(["check", "--quiet", "--offline", "--manifest-path"])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(scratch.join("target"))
            .output()
            .unwrap()
    };
    for (minimal, dead_code) in [(false, false), (false, true), (true, true)] {
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Test")
            .output(scratch.join("bindings.rs"));
        if !minimal {
            builder.full();
        }
        if dead_code {
            builder.dead_code();
        }
        builder.write();
        let result = compile();
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert_eq!(
            result.status.success(),
            minimal || !dead_code,
            "minimal={minimal}, dead_code={dead_code}: {stderr}"
        );
        if !minimal && dead_code {
            assert!(
                stderr.contains("associated functions `new` and `matches` are never used"),
                "{stderr}"
            );
        }
    }
    std::fs::write(
        scratch.join("lib.rs"),
        r#"#![deny(warnings)]
#![allow(nonstandard_style)]
mod bindings;
#[windows_core::implement(bindings::NativeDerived, bindings::RuntimeBase, bindings::RuntimeDerived)]
struct Object;
impl bindings::NativeBase_Impl for Object_Impl {
    fn BaseValue(&self) -> bindings::Payload { bindings::Payload { value: 1 } }
}
impl bindings::NativeDerived_Impl for Object_Impl {
    fn DerivedValue(&self) -> i32 { 2 }
}
impl bindings::RuntimeBase_Impl for Object_Impl {
    fn BaseValue(&self) -> windows_core::Result<i32> { Ok(3) }
}
impl bindings::RuntimeDerived_Impl for Object_Impl {
    fn DerivedValue(&self) -> windows_core::Result<i32> { Ok(4) }
}
struct Scoped;
impl bindings::Scoped_Impl for Scoped {
    fn Value(&self) -> i32 { 5 }
}
impl bindings::I_Impl for Scoped {
    fn Plain(&self) {}
    fn Annotated(&self, _: u32) {}
}
pub fn create() -> bindings::NativeDerived { Object.into() }
pub fn scoped() -> windows_core::ScopedInterface<'static, bindings::Scoped> {
    bindings::Scoped::new(&Scoped)
}
pub fn annotated() -> windows_core::ScopedInterface<'static, bindings::I> {
    bindings::I_Impl::Annotated(&Scoped, bindings::One);
    bindings::I::new(&Scoped)
}
"#,
    )
    .unwrap();
    for (full, selection, excluded) in [
        (false, "named", None),
        (true, "named", None),
        (false, "prefix", None),
        (false, "all", None),
        (true, "named", Some("Test.Native.NativeBase")),
        (false, "named", Some("Test.Native.Payload")),
        (false, "named", Some("Test.Runtime.RuntimeBase")),
    ] {
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filters([
                "Test.Native.NativeDerived::{}",
                "Test.Native.Scoped::{}",
                "Test.Runtime.RuntimeDerived::{}",
                "Windows.Win32.Api.I::{}",
            ])
            .output(scratch.join("bindings.rs"));
        if full {
            builder.full();
        } else {
            builder.dead_code();
        }
        if let Some(excluded) = excluded {
            builder.filter(&format!("!{excluded}"));
        }
        match selection {
            "named" => {
                builder.implements([
                    "Test.Native.NativeDerived",
                    "Test.Native.Scoped",
                    "Test.Runtime.RuntimeDerived",
                    "Windows.Win32.Api.I",
                ]);
            }
            "prefix" => {
                builder.implement("Test").implement("Windows.Win32.Api.I");
            }
            "all" => {
                builder.implement_all();
            }
            _ => unreachable!(),
        }
        builder.write();
        if let Some(excluded) = excluded {
            let name = excluded.rsplit('.').next().unwrap();
            let source = std::fs::read_to_string(scratch.join("bindings.rs")).unwrap();
            assert!(
                !source.contains(&format!("pub struct {name}")),
                "{excluded}"
            );
        }
        let result = compile();
        assert_eq!(
            result.status.success(),
            excluded.is_none(),
            "full={full}, implementation={selection}, excluded={excluded:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
fn associated_enum_method_filtering() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("associated_enum_method_filtering");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/associated_enum_methods.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["full", "minimal", "minimal_implement_all", "sys"] {
        for root in [
            "Windows.Win32",
            "Windows.Win32.Api.I",
            "Windows.Win32.Api.I::Plain",
        ] {
            let output = scratch.join(format!("{style}-{}.rs", root.replace("::", "-")));
            let mut builder = windows_bindgen::builder();
            builder.input(&winmd).filter(root).output(&output);
            if root == "Windows.Win32" {
                builder.filter("!Windows.Win32.Values.Options");
            }
            match style {
                "full" => {
                    builder.full();
                }
                "minimal_implement_all" => {
                    builder.implement_all();
                }
                "sys" => {
                    builder.sys();
                }
                _ => {}
            }
            builder.write();
            let source = std::fs::read_to_string(output).unwrap();
            assert!(
                source.contains("pub struct I_Vtbl"),
                "{style} {root}: {source}"
            );
            assert!(source.contains("pub Plain:"), "{style} {root}: {source}");
            let annotated = root == "Windows.Win32.Api.I"
                || (style == "minimal_implement_all" && root.ends_with("::Plain"));
            assert_eq!(
                source.contains("pub Annotated:"),
                annotated,
                "{style} {root}: {source}"
            );
            assert_eq!(
                source.contains("pub type Options"),
                annotated,
                "{style} {root}: {source}"
            );
            // Namespace expansion selects this standalone constant even when its type is excluded.
            assert_eq!(
                source.contains("pub const One:"),
                annotated || root == "Windows.Win32",
                "{style} {root}: {source}"
            );
        }
    }
}

#[test]
fn associated_enum_dependencies() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("associated_enum_dependencies");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/associated_enum.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["full", "minimal", "sys"] {
        for (root, dependency) in [
            ("Duplicate", "Options"),
            ("Socket", "Family"),
            ("GetFamily", "Family"),
            ("Callback", "Options"),
            ("Arguments", "Family"),
            ("IOptions::Apply", "Options"),
        ] {
            let output = scratch.join(format!("{style}-{}.rs", root.replace("::", "-")));
            let mut builder = windows_bindgen::builder();
            builder
                .input(&winmd)
                .filter(&format!("Test.{root}"))
                .output(&output);
            match style {
                "full" => {
                    builder.full();
                }
                "sys" => {
                    builder.sys();
                }
                _ => {}
            }
            builder.write();
            let source = std::fs::read_to_string(output).unwrap();
            assert!(
                source.contains(&format!("pub type {dependency} =")),
                "{style} {root}: {source}"
            );
            assert!(
                !source.contains("pub type Signed ="),
                "{style} {root}: {source}"
            );
        }
    }
}

#[test]
fn minimal_abi_dependency_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("minimal_abi_dependency_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/minimal_abi_dependencies.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["full", "minimal", "sys"] {
        for selection in [
            None,
            Some("ValueChain"),
            Some("Holder"),
            Some("IConsumer"),
            Some("Get"),
        ] {
            let output = scratch.join(format!("{style}-{}.rs", selection.unwrap_or("only")));
            let mut builder = windows_bindgen::builder();
            builder
                .input(&winmd)
                .filter("Test.CreateValue")
                .output(&output);
            if let Some(selection) = selection {
                builder.filter(&format!("Test.{selection}"));
            }
            match style {
                "full" => {
                    builder.full();
                }
                "sys" => {
                    builder.sys();
                }
                _ => {}
            }
            builder.write();
            let source = std::fs::read_to_string(output).unwrap();
            let retains_alias = style != "minimal" || selection.is_some();
            for alias in ["Value", "ValueChain"] {
                assert_eq!(
                    source.contains(&format!("pub type {alias} =")),
                    retains_alias,
                    "{style} {selection:?}: {source}"
                );
            }
            if style == "minimal" {
                assert!(source.contains("*mut *mut core::ffi::c_void"), "{source}");
                assert!(source.contains("pub struct IValue_Vtbl"), "{source}");
            }
        }
    }
}

#[test]
#[cfg(windows)]
fn generated_bindings_support_runtime_msrv() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("runtime-msrv");
    std::fs::create_dir_all(&scratch).unwrap();
    let libraries = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../libs");
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/struct_pointer_default.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let mut dependencies = String::new();
    for name in [
        "core",
        "link",
        "future",
        "collections",
        "reference",
        "time",
        "numerics",
    ] {
        dependencies.push_str(&format!(
            "windows-{name} = {{ path = '{}' }}\n",
            libraries.join(name).display()
        ));
    }
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!(
            "[package]\nname = \"runtime-msrv\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
             [workspace]\n[lib]\npath = \"lib.rs\"\n[dependencies]\n{dependencies}\
             [lints.rust]\nwarnings = \"deny\"\nnonstandard_style = \"allow\"\n"
        ),
    )
    .unwrap();
    for style in ["minimal", "full", "sys"] {
        let mut builder = windows_bindgen::builder();
        builder
            .filter("Windows.Win32::{SECURITY_ATTRIBUTES,HANDLE,WNDCLASSA}")
            .derive("SECURITY_ATTRIBUTES=Default")
            .output(scratch.join(format!("{style}.rs")));
        if style != "sys" {
            builder
                .filter("Windows.System.Threading.ThreadPool::RunAsync")
                .filter("Windows.Storage.Pickers.FileOpenPicker::FileTypeFilter")
                .filter("Windows.Devices.Geolocation.Geocoordinate::Speed")
                .filter("Windows.Foundation.IPropertyValue::{GetDateTime,GetTimeSpan}")
                .filter("Windows.UI.Composition.IVisual::Offset");
        }
        match style {
            "full" => {
                builder.full();
            }
            "sys" => {
                builder.sys();
            }
            _ => {}
        }
        builder.write();
        let mut pointers = windows_bindgen::builder();
        pointers
            .input(&winmd)
            .filter("Test")
            .derive("Test.PointerDefault=Default")
            .derive("Test.HandleDefault=Default")
            .output(scratch.join(format!("pointer_{style}.rs")));
        if style == "full" {
            pointers.full();
        } else if style == "sys" {
            pointers.sys();
        }
        pointers.write();
        if style != "sys" {
            let generated = std::fs::read_to_string(scratch.join(format!("{style}.rs"))).unwrap();
            for name in [
                "core",
                "future",
                "collections",
                "reference",
                "time",
                "numerics",
            ] {
                assert!(
                    generated.contains(&format!("windows_{name}::")),
                    "{style}: {name}"
                );
            }
        }
    }
    std::fs::write(
        scratch.join("lib.rs"),
        r#"
pub mod minimal;
pub mod full;
pub mod sys;
pub mod pointer_minimal;
pub mod pointer_full;
pub mod pointer_sys;

#[cfg(test)]
mod tests {
    use super::*;
    use windows_core::Interface;

    #[test]
    fn pointer_defaults() {
        assert!(minimal::SECURITY_ATTRIBUTES::default().lpSecurityDescriptor.is_null());
        assert!(full::SECURITY_ATTRIBUTES::default().lpSecurityDescriptor.is_null());
        assert!(sys::SECURITY_ATTRIBUTES::default().lpSecurityDescriptor.is_null());
        assert!(sys::WNDCLASSA::default().lpszClassName.is_null());
        assert!(pointer_full::MutableHandle::default().0.is_null());
        assert!(pointer_full::ConstHandle::default().0.is_null());
        assert!(pointer_minimal::PointerDefault::default().alias.is_null());
        assert!(pointer_full::PointerDefault::default().constant.is_null());
        assert!(pointer_sys::PointerDefault::default().raw.is_null());
        assert!(pointer_minimal::HandleDefault::default().mutable.is_null());
        assert!(pointer_full::HandleDefault::default().constant.0.is_null());
        assert!(pointer_sys::HandleDefault::default().constant.is_null());
        assert!(pointer_sys::InterfaceDefault::default().object.is_null());
    }

    #[windows_core::interface("bc192da2-54c5-4923-8ef8-f5be2e1cd8a3")]
    unsafe trait IProbe: windows_core::IUnknown {
        unsafe fn Value(&self) -> windows_core::Result<()>;
    }

    #[windows_core::implement(IProbe)]
    struct Probe;

    impl IProbe_Impl for Probe_Impl {
        unsafe fn Value(&self) -> windows_core::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn interface_macros_and_weak_references() {
        let probe: IProbe = Probe.into();
        unsafe { probe.Value().unwrap() };
        let weak = probe.downgrade().unwrap();
        assert!(weak.upgrade().is_some());
        drop(probe);
        assert!(weak.upgrade().is_none());
    }
}
"#,
    )
    .unwrap();
    let mut command = std::process::Command::new("cargo");
    command
        .args(["test", "--quiet", "--offline", "--manifest-path"])
        .arg(&manifest)
        .arg("--target-dir")
        .arg(scratch.join("target"));
    if let Some(toolchain) = std::env::var_os("WINDOWS_BINDGEN_TEST_TOOLCHAIN") {
        command.env("RUSTUP_TOOLCHAIN", toolchain);
    }
    let result = command.output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

fn run(name: &str) {
    let input_path = format!("input/{name}.rdl");
    let expected_path = format!("expected/{name}.rs");
    let scratch = format!("{}/{name}", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    // Extract args from `//!` comment lines at the top.
    let contents = std::fs::read_to_string(&input_path).unwrap();
    let mut args = Vec::new();

    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("//!") {
            for arg in rest.split_whitespace() {
                args.push(arg.to_string());
            }
        }
    }

    let winmd = format!("{scratch}/out.winmd");

    windows_rdl::reader()
        .input(&input_path)
        .output(&winmd)
        .write()
        .unwrap();

    let out_rs = format!("{scratch}/out.rs");
    let mut cli: Vec<String> = vec!["--in".into(), winmd];
    cli.extend(args);
    cli.push("--out".into());
    cli.push(out_rs.clone());

    windows_bindgen::bindgen(cli);

    let actual = std::fs::read_to_string(&out_rs).unwrap();
    std::fs::write(&expected_path, &actual).unwrap();
}

#[test]
fn layout_prelude_names() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("layout_prelude_names");
    std::fs::create_dir_all(&scratch).unwrap();
    for fixture in ["struct_packed", "struct_nested_member_alignment"] {
        let winmd = scratch.join(format!("{fixture}.winmd"));
        windows_rdl::reader()
            .input(format!("input/{fixture}.rdl"))
            .output(&winmd)
            .write()
            .unwrap();
        for style in ["full", "minimal", "sys"] {
            for (modules, collision) in [
                (false, "none"),
                (false, "constant"),
                (false, "nested"),
                (true, "none"),
                (true, "constant"),
                (true, "nested"),
            ] {
                let output = scratch.join(format!("{fixture}-{style}-{modules}-{collision}.rs"));
                let mut builder = windows_bindgen::builder();
                builder.input(&winmd).filter("Test").output(&output);
                if collision != "constant" {
                    builder.filter("!Test.size_of");
                    if fixture == "struct_nested_member_alignment" {
                        builder.filter("!Test.align_of");
                    }
                }
                if collision != "nested" {
                    builder.filter("!Test.size");
                    if fixture == "struct_nested_member_alignment" {
                        builder.filter("!Test.align");
                    }
                }
                match style {
                    "full" => {
                        builder.full();
                    }
                    "sys" => {
                        builder.sys();
                    }
                    _ => {}
                }
                if modules {
                    builder.modules();
                }
                builder.write();
                let source = std::fs::read_to_string(&output).unwrap();
                let result = std::process::Command::new("rustc")
                    .args([
                        "--crate-name=layout_imports",
                        "--crate-type=lib",
                        "--edition=2024",
                        "--emit=metadata",
                        "-Dwarnings",
                        "-Wunused-qualifications",
                        "-Anonstandard_style",
                    ])
                    .arg(&output)
                    .arg("-o")
                    .arg(scratch.join("output.rmeta"))
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "{fixture} {style} {modules} {collision}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
                assert!(!source.contains("use core::mem"));
                assert_eq!(source.contains("::core::mem::size_of"), collision != "none");
            }
        }
    }
}

#[test]
fn nested_dependency_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("nested_dependency_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/struct_nested_dependency.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["full", "minimal", "sys"] {
        for (flat, filter) in [
            (false, "Consumer.Value"),
            (true, "Consumer.Value"),
            (false, "Consumer"),
        ] {
            let output = scratch.join(format!("{style}-{flat}-{filter}.rs"));
            let mut builder = windows_bindgen::builder();
            builder.input(&winmd).filter(filter).output(&output);
            match style {
                "full" => {
                    builder.full();
                }
                "sys" => {
                    builder.sys();
                }
                _ => {}
            }
            if !flat {
                builder.modules();
            }
            builder.write();
            let result = std::process::Command::new("rustc")
                .args([
                    "--crate-name",
                    "nested_dependency",
                    "--crate-type=lib",
                    "--edition=2024",
                    "--emit=metadata",
                    "-Dwarnings",
                    "-Anonstandard_style",
                ])
                .arg(&output)
                .arg("-o")
                .arg(scratch.join("output.rmeta"))
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{style} {flat} {filter}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
}

#[test]
fn fixed_pointer_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("fixed_pointer_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/struct_fixed_pointer.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["full", "minimal", "sys"] {
        for package in [false, true] {
            for selection in ["fixture", "fixture.DeepOnly"] {
                let output = scratch.join(format!("{style}-{package}-{selection}"));
                std::fs::create_dir_all(output.join("src")).unwrap();
                let manifest = output.join("Cargo.toml");
                std::fs::write(
                    &manifest,
                    "[package]\nname = \"fixed-pointer-styles\"\nversion = \"0.0.0\"\n\
                 edition = \"2024\"\n[workspace]\n\
                 [lints.rust]\nwarnings = \"deny\"\nnonstandard_style = \"allow\"\n\
                 [features]\n# generated features\n",
                )
                .unwrap();
                let library = output.join("src\\lib.rs");
                let mut builder = windows_bindgen::builder();
                builder
                    .input(&winmd)
                    .filter(&format!("Windows.Win32.{selection}"));
                match style {
                    "full" => {
                        builder.full();
                    }
                    "sys" => {
                        builder.sys();
                    }
                    _ => {}
                }
                if package {
                    builder.package().output(&output);
                    std::fs::write(&library, "pub mod Windows;\n").unwrap();
                } else {
                    builder.output(&library);
                }
                builder.write();
                let mut command = std::process::Command::new("cargo");
                command
                    .args(["check", "--quiet", "--manifest-path"])
                    .arg(&manifest);
                if package {
                    command.args(["--no-default-features", "--features", "fixture"]);
                }
                let result = command.output().unwrap();
                assert!(
                    result.status.success(),
                    "{style} {package} {selection}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
    }
}

#[test]
fn named_bitfield_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("named_bitfield_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/struct_named_bitfield.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let core = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\libs\\core");
    let manifest = scratch.join("Cargo.toml");
    for style in ["minimal", "sys"] {
        let output = scratch.join(format!("{style}.rs"));
        let mut builder = windows_bindgen::builder();
        builder.input(&winmd).filter("Test").output(&output);
        if style == "sys" {
            builder.sys();
        }
        builder.write();
        std::fs::write(
            &manifest,
            format!(
                "[package]\nname = \"named-bitfield-styles\"\nversion = \"0.0.0\"\n\
                 edition = \"2024\"\n[workspace]\n[lib]\npath = \"{style}.rs\"\n\
                 [dependencies]\nwindows-core = {{ path = '{}' }}\n\
                 [lints.rust]\nwarnings = \"deny\"\nnonstandard_style = \"allow\"\n",
                core.display()
            ),
        )
        .unwrap();
        let result = std::process::Command::new("cargo")
            .args(["check", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(scratch.join("target"))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{style}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
#[ignore = "requires the x86, x64, and ARM64 MSVC Rust targets"]
fn enum_constant_shadow_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("enum_constant_shadow_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/enum_constant_shadow.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["full", "minimal", "sys"] {
        for filter in [
            "Test",
            "Test.Pool",
            "Test.Pool::{Default,Aligned,Unshadowed,Replaced}",
            "Test.Default",
        ] {
            let output = scratch.join("bindings.rs");
            let mut builder = windows_bindgen::builder();
            builder.input(&winmd).filter(filter).output(&output);
            match style {
                "full" => {
                    builder.full();
                }
                "sys" => {
                    builder.sys();
                }
                _ => {}
            }
            builder.write();
            let mut rust = std::fs::read_to_string(&output).unwrap();
            rust.push_str("\nconst _: () = { assert!(Default == if cfg!(target_arch = \"aarch64\") { 512 } else { 0 }); };\n");
            if filter != "Test.Default" {
                rust.push_str(
                    "\nconst _: () = {\n\
                    assert!(Aligned == if cfg!(target_arch = \"aarch64\") { 516 } else { 4 });\n\
                    assert!(Unshadowed == 8 && Replaced == 10);\n\
                };\n",
                );
            }
            if filter == "Test" {
                rust.push_str(
                    "\n#[cfg(target_arch = \"x86\")] const PartialValue: i32 = -1;\n\
                    const _: () = {\n\
                    #[cfg(target_arch = \"x86_64\")] assert!(PartialValue == 20);\n\
                    #[cfg(target_arch = \"aarch64\")] assert!(PartialValue == 21);\n\
                    #[cfg(target_arch = \"x86\")] assert!(PartialValue == -1);\n\
                    assert!(ScopedValue == 41 && Scoped::ScopedValue.0 == 40);\n\
                };\n",
                );
            }
            std::fs::write(&output, rust).unwrap();
            for target in [
                "i686-pc-windows-msvc",
                "x86_64-pc-windows-msvc",
                "aarch64-pc-windows-msvc",
            ] {
                let result = std::process::Command::new("rustc")
                    .args([
                        "--crate-name",
                        "enum_constant_shadow",
                        "--crate-type=lib",
                        "--emit=metadata",
                        "--edition=2024",
                        "--target",
                        target,
                    ])
                    .arg(&output)
                    .arg("-o")
                    .arg(scratch.join("output.rmeta"))
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "{style} {filter} {target}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
    }
}

#[test]
fn real_variadic_exports_are_raw_sys_only() {
    let rich =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/authz/mod.rs").unwrap();
    assert!(!rich.contains("pub unsafe fn AuthzReportSecurityEvent("));

    let sys = std::fs::read_to_string("../../../libs/sys/src/Windows/Win32/authz/mod.rs").unwrap();
    assert!(sys.contains(
        "windows_link::link!(\"authz.dll\" \"C\" fn AuthzReportSecurityEvent(dwflags : super::DWORD, \
         heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : super::DWORD, pusersid : \
         super::PSID, dwcount : super::DWORD, ...) -> windows_sys::core::BOOL);"
    ));
}

#[test]
fn real_inout_parameters_remain_output_capable() {
    let wingdi =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/wingdi/mod.rs").unwrap();
    assert!(wingdi.contains(
        "pub unsafe fn DPtoLP(hdc: super::HDC, lppt: super::LPPOINT, c: i32) -> \
         windows_core::BOOL"
    ));
    let windef =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/windef/mod.rs").unwrap();
    assert!(windef.contains("pub type LPPOINT = *mut POINT;"));

    let dpapi =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/dpapi/mod.rs").unwrap();
    assert!(dpapi.contains(
        "pub unsafe fn CryptProtectMemory(pdatain: super::LPVOID, cbdatain: super::DWORD, \
         dwflags: super::DWORD) -> windows_core::BOOL"
    ));
    let minwindef =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/minwindef/mod.rs")
            .unwrap();
    assert!(minwindef.contains("pub type LPVOID = *mut core::ffi::c_void;"));
}
