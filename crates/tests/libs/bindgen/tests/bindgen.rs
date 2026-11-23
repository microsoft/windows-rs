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

fn bundled_filter_output(case: &str, style: &str, filters: &[&str]) -> String {
    let output = std::path::Path::new(env!("OUT_DIR")).join(format!("{case}-{style}.rs"));
    let mut builder = windows_bindgen::builder();
    builder.filters(filters).output(&output);
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
fn filter_negative_members_do_not_add_roots() {
    for style in ["minimal", "full", "sys"] {
        let baseline = filter_contract_output("negative-baseline", style, &["Other.Token"]);
        for exclusion in [
            "!Test.ISource::GetResult",
            "!Test.Widget::Value",
            "!Test.Color::Red",
            "!Native.NativeColor::NativeRed",
        ] {
            for filters in [["Other.Token", exclusion], [exclusion, "Other.Token"]] {
                let output = filter_contract_output("negative-roots", style, &filters);
                assert_eq!(baseline, output, "{style}: {exclusion}");
            }
        }
    }
}

#[test]
fn filter_exclusions_do_not_expand_shells() {
    for style in ["minimal", "full", "sys"] {
        for (owner, member) in [
            ("Test.ISource", "GetResult"),
            ("Test.Widget", "Value"),
            ("ClassDependency.Widget", "Write"),
            ("Native.INativeSource", "GetNativeResult"),
        ] {
            let shell = format!("{owner}::{{}}");
            let exclusion = format!("!{owner}::{member}");
            let baseline = filter_contract_output("shell-baseline", style, &[&shell]);
            for filters in [
                [shell.as_str(), exclusion.as_str()],
                [exclusion.as_str(), shell.as_str()],
            ] {
                let output = filter_contract_output("shell-exclusion", style, &filters);
                assert_eq!(baseline, output, "{style}: {owner}");
            }
        }
    }
}

#[test]
fn filter_class_exclusions_preserve_other_signatures() {
    for style in ["minimal", "full", "sys"] {
        for root in ["ClassDependency", "ClassDependency.Widget"] {
            for filters in [
                [root, "!ClassDependency.Widget::Write"],
                ["!ClassDependency.Widget::Write", root],
            ] {
                let output = filter_contract_output("class-excluded-signatures", style, &filters);
                assert!(output.contains("pub struct Token"), "{style}: {root}");
                assert!(output.contains("pub Read:"), "{style}: {root}");
                assert!(!output.contains("pub Write:"), "{style}: {root}");
            }
        }
    }
}

#[test]
fn filter_constructor_exclusions() {
    for style in ["minimal", "full"] {
        for filters in [
            &["PropertySet::{}", "!PropertySet::CreateInstance"][..],
            &[
                "PropertySet::CreateInstance",
                "!PropertySet::CreateInstance",
            ][..],
            &[
                "!PropertySet::CreateInstance",
                "PropertySet::CreateInstance",
            ][..],
        ] {
            let output = bundled_filter_output("constructor-exclusion", style, filters);
            assert!(!output.contains("pub fn new("), "{style}: {filters:?}");
        }
    }
}

#[test]
fn filter_factories_are_not_instance_interfaces() {
    for style in ["minimal", "full"] {
        let output = bundled_filter_output("factory-hierarchy", style, &["Windows.Devices.I2c"]);
        assert!(output.contains("pub fn FromIdAsync"), "{style}");
        for hierarchy in output.split("required_hierarchy!").skip(1) {
            assert!(
                !hierarchy
                    .split_once(");")
                    .unwrap()
                    .0
                    .contains("II2cDeviceStatics"),
                "{style}: {}",
                hierarchy.split_once(");").unwrap().0
            );
        }
    }
}

#[test]
fn filter_preserves_vtable_slot_names() {
    for style in ["minimal", "full"] {
        let output = filter_contract_output(
            "filtered-slots",
            style,
            &[
                "Slots.IProbe::Value",
                "!Slots.IProbe::get_Value",
                "Slots.NativeProbe::Value",
                "!Slots.NativeProbe::get_Value",
                "Slots.Widget::Value",
                "!Slots.Widget::get_Value",
            ],
        );
        assert!(output.contains("pub Value2:"), "{style}");
        assert!(!output.contains("vtable(self).Value)"), "{style}");
        assert!(!output.contains("vtable(this).Value)"), "{style}");
    }
}

#[test]
fn filter_native_override_selection() {
    for style in ["minimal", "full", "sys"] {
        let output = bundled_filter_output(
            "override-selection",
            style,
            &["IDirect3D9Ex::GetAdapterCount"],
        );
        assert!(output.contains("pub GetAdapterCount:"), "{style}");
        let output = bundled_filter_output(
            "override-exclusion",
            style,
            &[
                "IDirect3D9",
                "IDirect3D9Ex::{}",
                "!IDirect3D9Ex::GetAdapterCount",
            ],
        );
        assert!(!output.contains("pub GetAdapterCount:"), "{style}");
        assert!(output.contains("pub GetAdapterIdentifier:"), "{style}");
    }
}

#[test]
fn filter_namespace_dependency_exclusions() {
    for style in ["minimal", "full", "sys"] {
        for (namespace, dependency) in [
            ("Publication", "Publication"),
            ("NativePublication", "NativePublication"),
            ("PublicationSource", "Publication"),
            ("NativePublicationSource", "NativePublication"),
        ] {
            let output = filter_contract_output(
                "dependency-exclusions",
                style,
                &[namespace, &format!("!{dependency}.Hidden")],
            );
            for excluded in ["pub struct Hidden", "pub struct Holder", "pub GetHolder:"] {
                assert!(
                    !output.contains(excluded),
                    "{style}: {namespace}: {excluded}"
                );
            }
            assert!(output.contains("pub Ping:"), "{style}: {namespace}");
        }
    }
}

#[test]
fn filter_architecture_dependency_exclusions() {
    for style in ["minimal", "full", "sys"] {
        let output = filter_contract_output(
            "architecture-exclusions",
            style,
            &["NativeArchitecture", "!Publication.Hidden"],
        );
        for present in [
            "pub struct Record",
            "pub struct Holder",
            "pub type Callback",
            "fn UseRecord(",
        ] {
            assert!(output.contains(present), "{style}: {present}");
        }
        assert!(!output.contains("target_arch = \"aarch64\""), "{style}");
        assert!(!output.contains("pub GetRecord:"), "{style}");
        assert!(output.contains("pub Ping:"), "{style}");
        assert!(!output.contains("pub struct AllTargets"), "{style}");
        assert!(!output.contains("fn UseAllTargets("), "{style}");
    }
}

#[test]
fn filter_function_dependency_exclusions() {
    for style in ["minimal", "full", "sys"] {
        let output = filter_contract_output(
            "function-exclusions",
            style,
            &["NativePublication", "!NativePublication.Hidden"],
        );
        assert!(output.contains("fn Variant("), "{style}");
        assert!(!output.contains("target_arch = \"aarch64\""), "{style}");
    }
}

#[test]
fn filter_sys_erased_dependencies() {
    for selection in [
        "Raw",
        "Raw.Holder",
        "Raw.Callback",
        "Raw.Functions",
        "Raw.Take",
        "Raw.Return",
    ] {
        let output = filter_contract_output("sys-erasure", "sys", &[selection]);
        assert!(!output.contains("windows_core"), "{selection}\n{output}");
        assert!(!output.contains("pub struct Widget"), "{selection}");
    }
    let output = bundled_filter_output("sys-erased-alias", "sys", &["PDISPATCHERQUEUE"]);
    assert!(output.contains("pub type PDISPATCHERQUEUE = *mut core::ffi::c_void;"));
    assert!(!output.contains("windows_core"));
}

#[test]
fn filter_regressions_compile() {
    compile_filter_regressions(&[None]);
}

#[test]
#[ignore = "requires x86, x64, and ARM64 MSVC Rust targets"]
fn filter_regressions_cross_targets() {
    compile_filter_regressions(&[
        Some("i686-pc-windows-msvc"),
        Some("x86_64-pc-windows-msvc"),
        Some("aarch64-pc-windows-msvc"),
    ]);
}

fn compile_filter_regressions(targets: &[Option<&str>]) {
    let scratch = std::path::Path::new(env!("OUT_DIR"))
        .join("filter-regressions")
        .join(targets[0].unwrap_or("host"));
    std::fs::create_dir_all(&scratch).unwrap();
    let libs = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libs");
    let manifest = scratch.join("Cargo.toml");
    for style in ["minimal", "full", "sys"] {
        let core = if style == "sys" {
            String::new()
        } else {
            format!(
                "windows-core = {{ path = '{}' }}\n",
                libs.join("core").display()
            )
        };
        std::fs::write(
            &manifest,
            format!(
                "[package]\nname = \"filter-regressions\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
                 [workspace]\n[lib]\npath = \"lib.rs\"\n[dependencies]\n\
                 windows-link = {{ path = '{}' }}\n{core}",
                libs.join("link").display(),
            ),
        ).unwrap();
        let mut source =
            String::from("#![deny(warnings)]\n#![allow(dead_code, nonstandard_style)]\n");
        for (name, filters) in [
            (
                "architecture",
                &["NativeArchitecture", "!Publication.Hidden"][..],
            ),
            (
                "functions",
                &["NativePublication", "!NativePublication.Hidden"][..],
            ),
        ] {
            let output = filter_contract_output(&format!("compile-{name}"), style, filters);
            std::fs::write(scratch.join(format!("{name}.rs")), output).unwrap();
            source.push_str(&format!("mod {name};\n"));
        }
        let output = bundled_filter_output(
            "compile-override",
            style,
            &["IDirect3D9Ex::GetAdapterCount"],
        );
        std::fs::write(scratch.join("inherited.rs"), output).unwrap();
        source.push_str(
            "mod inherited;\npub fn inherited_slot(value: &inherited::IDirect3D9_Vtbl) { let _ = value.GetAdapterCount; }\n",
        );
        if style == "sys" {
            let output = filter_contract_output("compile-raw", style, &["Raw"]);
            std::fs::write(scratch.join("raw.rs"), output).unwrap();
            source.push_str(
                "mod raw;\npub fn raw_slot(value: &raw::IDerived_Vtbl) { let _ = value.base__.Get; let _ = value.Set; }\n",
            );
            source.push_str(r#"
pub fn raw_signatures(holder: raw::Holder, callback: raw::Callback, functions: raw::Functions) {
    let _: [*mut core::ffi::c_void; 2] = holder.widgets;
    let _: *mut *mut core::ffi::c_void = holder.interface;
    let _: *mut core::ffi::c_void = holder.generic;
    let _: *mut core::ffi::c_void = holder.handler;
    let _: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void> = callback;
    let _: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void> = functions.callback;
    let _: unsafe extern "C" fn() -> *mut core::ffi::c_void = raw::Return;
}
"#);
        } else {
            let output = filter_contract_output(
                "compile-slots",
                style,
                &[
                    "Slots.IProbe::Value",
                    "!Slots.IProbe::get_Value",
                    "Slots.NativeProbe::Value",
                    "!Slots.NativeProbe::get_Value",
                    "Slots.Widget::Value",
                    "!Slots.Widget::get_Value",
                    "Slots.IRequired::{}",
                ],
            );
            std::fs::write(scratch.join("slots.rs"), output).unwrap();
            source.push_str(
                r#"
mod slots;
pub unsafe fn callers(value: &slots::IProbe, native: &slots::NativeProbe, class: &slots::Widget) {
    let _ = value.Value();
    let _ = unsafe { native.Value() };
    let _ = class.Value();
}
"#,
            );
            if style == "full" {
                source.push_str(
                    "pub fn forwarder(value: &slots::IRequired) { let _ = value.Value(); }\n",
                );
            }
        }
        std::fs::write(scratch.join("lib.rs"), source).unwrap();
        for target in targets {
            let mut command = std::process::Command::new("cargo");
            command
                .args(["check", "--quiet", "--offline", "--manifest-path"])
                .arg(&manifest)
                .arg("--target-dir")
                .arg(scratch.join("target"));
            if let Some(target) = target {
                command.args(["--target", target]);
            }
            let result = command.output().unwrap();
            assert!(
                result.status.success(),
                "{style} {target:?}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
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
fn filter_mixed_class_dependencies_compile() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("filter-mixed-class");
    std::fs::create_dir_all(&scratch).unwrap();
    let libs = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libs");
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!(
            "[package]\nname = \"filter-mixed-class\"\nversion = \"0.0.0\"\n\
             edition = \"2024\"\n[workspace]\n[lib]\npath = \"lib.rs\"\n\
             [dependencies]\n\
             windows-core = {{ path = '{}' }}\n\
             windows-collections = {{ path = '{}' }}\n\
             windows-numerics = {{ path = '{}' }}\n",
            libs.join("core").display(),
            libs.join("collections").display(),
            libs.join("numerics").display(),
        ),
    )
    .unwrap();
    std::fs::write(
        scratch.join("lib.rs"),
        "#![allow(dead_code, nonstandard_style)]\nmod bindings;\n",
    )
    .unwrap();
    for full in [true, false] {
        let mut builder = windows_bindgen::builder();
        builder
            .filters(["Uri", "Windows.Foundation.Numerics"])
            .implement_all()
            .modules()
            .output(scratch.join("bindings.rs"));
        if full {
            builder.full();
        }
        builder.write();
        let result = std::process::Command::new("cargo")
            .args(["check", "--quiet", "--offline", "--manifest-path"])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(scratch.join("target"))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "full={full}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
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
    for all in [false, true] {
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filters([
                "Test.Native.NativeDerived::{}",
                "Test.Native.Scoped::{}",
                "Test.Runtime.RuntimeBase::{}",
                "Test.Runtime.RuntimeDerived::{}",
                "Windows.Win32.Api.I::{}",
            ])
            .dead_code()
            .output(scratch.join("bindings.rs"));
        if all {
            builder.implement_all();
        } else {
            builder.implement("Test").implement("Windows.Win32.Api.I");
        }
        builder.write();
        let result = compile();
        assert!(
            result.status.success(),
            "implement_all={all}: {}",
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
            assert_eq!(
                source.contains("pub const One:"),
                annotated,
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
