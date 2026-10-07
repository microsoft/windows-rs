use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
use windows_clang2::{Input, ProjectionOptions};

#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [backend, count] = args.as_slice() else {
        return Err("usage: wdk <new|old> <TU count>".into());
    };
    let count: usize = count.parse()?;
    if count == 0 || !matches!(backend.as_str(), "new" | "old") {
        return Err("backend must be new or old and TU count must be positive".into());
    }
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let arguments = sdk::wdk_arguments("--target=x86_64-pc-windows-msvc");
    let arguments: Vec<_> = arguments.iter().map(String::as_str).collect();
    let roots = ["DeviceIoControl", "QuerySecurity"];
    let inputs: Vec<_> = (0..count)
        .map(|index| {
            Input::new(
                format!("wdk{index}.hpp"),
                include_str!("../input/wdk_layout.h"),
            )
        })
        .collect();
    let start = Instant::now();
    let rdl = if backend == "new" {
        let snapshot = windows_clang2::capture(inputs, &arguments, &roots)?;
        println!("capture_ms={}", start.elapsed().as_millis());
        let start = Instant::now();
        let resolved = snapshot.resolve()?;
        println!(
            "resolve_us={} groups={} observations={} incomplete={:?}",
            start.elapsed().as_micros(),
            resolved.group_count(),
            resolved.report().observations,
            resolved.report().incomplete
        );
        let start = Instant::now();
        let plan = resolved.project(&ProjectionOptions::new("Wdk"))?;
        println!("project_us={}", start.elapsed().as_micros());
        plan.rdl()
    } else {
        let inputs = inputs.into_iter().map(|input| {
            let name = input.name.clone();
            windows_clang::Input::new(input.name, input.source).with_roots([name])
        });
        let snapshot = windows_clang::extract(inputs, &arguments)?;
        println!(
            "capture_ms={} facts={}",
            start.elapsed().as_millis(),
            snapshot.facts().len()
        );
        let excluded: BTreeSet<_> = snapshot
            .facts()
            .iter()
            .filter(|fact| !roots.contains(&fact.name.as_str()))
            .map(|fact| fact.name.clone())
            .collect();
        let references = BTreeMap::new();
        let mut options = windows_clang::EmitOptions::new("Wdk", &references);
        options.excluded = Some(&excluded);
        let start = Instant::now();
        let result = snapshot.emit_with_options(&options);
        println!("emit_us={}", start.elapsed().as_micros());
        result?
    };
    println!("{rdl}");
    let winmd = std::path::Path::new(env!("OUT_DIR")).join(format!("wdk-{backend}-{count}.winmd"));
    windows_rdl::reader()
        .input_text(&rdl)
        .reference_default()
        .output(&winmd)
        .write()?;
    let index = windows_metadata::reader::Index::read(&winmd).ok_or("invalid emitted metadata")?;
    for root in roots {
        if !index
            .types()
            .any(|ty| ty.namespace() == "Wdk" && ty.name() == root && ty.fields().count() > 0)
        {
            return Err(format!(
                "selected record `{root}` was not emitted with fields; no equal-output comparison"
            )
            .into());
        }
    }
    Ok(())
}
