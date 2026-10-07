use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
use windows_clang2::{Input, ReferenceKind};

#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [backend, pairs, roots] = args.as_slice() else {
        return Err("usage: webview <new|old> <TU pairs> <1|3|interop|consumer>".into());
    };
    let pairs: usize = pairs.parse()?;
    if pairs == 0 {
        return Err("TU pairs must be positive".into());
    }
    let root_set = roots;
    let consumer_roots = sdk::webview_roots();
    let roots: &[&str] = match root_set.as_str() {
        "1" => &["ICoreWebView2Deferral"],
        "3" => &[
            "ICoreWebView2Deferral",
            "ICoreWebView2StringCollection",
            "ICoreWebView2HttpHeadersCollectionIterator",
        ],
        "interop" => &["ICoreWebView2Interop2"],
        "consumer" => &consumer_roots,
        _ => return Err("roots must be 1, 3, interop, or consumer".into()),
    };
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let headers = sdk::webview_headers();
    let include = headers[0].parent().unwrap();
    let interop = headers[1].parent().unwrap();
    let mut arguments = sdk::arguments("--target=x86_64-pc-windows-msvc");
    arguments.extend([
        format!("-I{}", include.display()),
        format!("-I{}", interop.display()),
    ]);
    let arguments: Vec<_> = arguments.iter().map(String::as_str).collect();
    let inputs: Vec<_> = (0..pairs)
        .flat_map(|pair| {
            [
                Input::new(format!("main{pair}.hpp"), "#include <WebView2.h>"),
                Input::new(format!("interop{pair}.hpp"), "#include <WebView2Interop.h>"),
            ]
        })
        .collect();
    let options = sdk::webview_options("--target=x86_64-pc-windows-msvc");
    let output =
        std::path::Path::new(env!("OUT_DIR")).join(format!("webview-{backend}-{pairs}-{root_set}"));
    std::fs::create_dir_all(&output)?;
    println!("roots={} output={}", roots.len(), output.display());
    let start = Instant::now();
    let rdl = match backend.as_str() {
        "new" => {
            let snapshot = windows_clang2::capture(inputs, &arguments, roots)?;
            let capture = start.elapsed();
            let start = Instant::now();
            let resolved = snapshot.resolve()?;
            let resolution = start.elapsed();
            println!(
                "capture_ms={} resolve_us={} groups={} observations={} declaration_pairs={} type_pairs={} incomplete={:?}",
                capture.as_millis(),
                resolution.as_micros(),
                resolved.group_count(),
                resolved.report().observations,
                resolved.report().declaration_pairs,
                resolved.report().type_pairs,
                resolved.report().incomplete
            );
            let start = Instant::now();
            let plan = resolved.project(&options)?;
            println!("project_us={}", start.elapsed().as_micros());
            plan.rdl()
        }
        "old" => {
            let inputs = inputs.into_iter().map(|input| {
                windows_clang::Input::new(input.name, input.source).with_roots([
                    include.join("WebView2.h").to_string_lossy().into_owned(),
                    interop
                        .join("WebView2Interop.h")
                        .to_string_lossy()
                        .into_owned(),
                ])
            });
            let snapshot = windows_clang::extract(inputs, &arguments)?;
            let capture = start.elapsed();
            let excluded: BTreeSet<_> = snapshot
                .facts()
                .iter()
                .filter(|fact| !roots.contains(&fact.name.as_str()))
                .map(|fact| fact.name.clone())
                .collect();
            let references: BTreeMap<_, _> = options
                .references
                .iter()
                .map(|(native, reference)| {
                    (
                        native.clone(),
                        windows_clang::TypeReference::new(
                            &reference.namespace,
                            &reference.name,
                            match reference.kind {
                                ReferenceKind::Value => windows_clang::TypeReferenceKind::Type,
                                ReferenceKind::Interface => {
                                    windows_clang::TypeReferenceKind::Interface
                                }
                            },
                        ),
                    )
                })
                .collect();
            let mut options = windows_clang::EmitOptions::new("WebView2", &references);
            options.library = Some("WebView2Loader.dll");
            options.excluded = Some(&excluded);
            let start = Instant::now();
            let rdl = snapshot.emit_with_options(&options)?;
            println!(
                "capture_ms={} emit_ms={} facts={}",
                capture.as_millis(),
                start.elapsed().as_millis(),
                snapshot.facts().len()
            );
            rdl
        }
        _ => return Err("backend must be new or old".into()),
    };
    std::fs::write(output.join("test.rdl"), &rdl)?;
    let reference = output.join("reference.winmd");
    windows_rdl::reader()
        .input_text(include_str!("../input/webview_reference.rdl"))
        .reference_default()
        .output(&reference)
        .write()?;
    let winmd = output.join("test.winmd");
    windows_rdl::reader()
        .input_text(&rdl)
        .reference_default()
        .reference(&reference)
        .output(&winmd)
        .write()?;
    let index =
        windows_metadata::reader::Index::read(&winmd).ok_or("invalid generated metadata")?;
    use windows_metadata::reader::{HasAttributes, Item};
    println!(
        "types={} methods={}",
        index.types().count(),
        index.types().map(|ty| ty.methods().count()).sum::<usize>()
    );
    for root in roots {
        if root_set == "consumer" {
            index.expect_item("WebView2", root);
            continue;
        }
        let Item::Type(ty) = index.expect_item("WebView2", root) else {
            println!("{root} function");
            continue;
        };
        let Some(guid) = ty.find_attribute("GuidAttribute") else {
            println!("{root} value");
            continue;
        };
        println!("{root} guid={:?}", guid.value());
        println!(
            "  bases={:?}",
            ty.interface_impls()
                .map(|base| base.interface(&[]))
                .collect::<Vec<_>>()
        );
        for method in ty.methods() {
            let signature = method.signature(&[]);
            let parameters = method.params_by_sequence(signature.types.len()).unwrap();
            println!(
                "  {} {:?} {:?}",
                method.name(),
                signature,
                parameters
                    .params()
                    .iter()
                    .map(|param| param.map(|param| (param.direction(), param.is_optional())))
                    .collect::<Vec<_>>()
            );
        }
    }
    sdk::webview_bindings(&winmd, &reference, &output.join("bindings.rs"), roots);
    println!("output={}", output.display());
    Ok(())
}
