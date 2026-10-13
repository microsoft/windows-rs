use std::time::Instant;
use windows_clang2::{Input, Snapshot, Target, capture_partial, merge, projection};

const GROUPS: &[&[&str]] = &[
    &[],
    &["ws2tcpip.h", "iphlpapi.h"],
    &["d3d12.h", "dxgi1_6.h"],
    &["d2d1_3.h", "dwrite_3.h"],
    &["winhttp.h", "http.h"],
    &["setupapi.h", "cfgmgr32.h", "devpropdef.h"],
    &["bcrypt.h", "ncrypt.h", "wincrypt.h"],
    &["shlobj.h", "shellapi.h", "propsys.h"],
];

pub fn run(mode: &str, count: usize) {
    let groups: Vec<Vec<&str>> = match mode {
        "breadth" => {
            assert!(matches!(count, 1 | 2 | 4 | 8));
            vec![
                GROUPS[..count]
                    .iter()
                    .flat_map(|group| group.iter().copied())
                    .collect(),
            ]
        }
        "tus" => {
            assert!(matches!(count, 2 | 8 | 32));
            (0..count)
                .map(|id| GROUPS[id % GROUPS.len()].to_vec())
                .collect()
        }
        _ => panic!("expected breadth or tus"),
    };
    let template = test_clang2::sdk_fixture();
    let inputs: Vec<_> = groups
        .iter()
        .enumerate()
        .map(|(id, headers)| {
            let mut source = "#include <winsock2.h>\n#include <windows.h>\n".to_string();
            for header in headers {
                source.push_str(&format!("#include <{header}>\n"));
            }
            Input {
                name: format!("sweep_{id:03}.h"),
                source,
                arguments: template.arguments.clone(),
                ..Default::default()
            }
        })
        .collect();
    let started = Instant::now();
    let captured = capture_partial(&inputs, &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let capture_ms = started.elapsed().as_secs_f64() * 1000.0;
    let stats: Vec<_> = captured
        .snapshots
        .iter()
        .map(Snapshot::statistics)
        .collect();
    let capture_omissions = captured.omissions.len();
    let started = Instant::now();
    let merged = merge(&captured.snapshots).unwrap();
    let merge_ms = started.elapsed().as_secs_f64() * 1000.0;
    let definition_pairs: usize = merged
        .targets
        .values()
        .map(|value| value.work.definition_pairs)
        .sum();
    let alias_pairs: usize = merged
        .targets
        .values()
        .map(|value| value.work.alias_pairs)
        .sum();
    let type_pairs: usize = merged
        .targets
        .values()
        .map(|value| value.work.type_pairs)
        .sum();
    drop(merged);
    let started = Instant::now();
    let result = projection::project_partial(captured, "Windows.Win32").unwrap();
    let project_ms = started.elapsed().as_secs_f64() * 1000.0;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\clang2-review");
    std::fs::create_dir_all(&scratch).unwrap();
    let name = format!("{mode}-{count}");
    std::fs::write(
        scratch.join(format!("{name}.omissions.txt")),
        result.omissions.join("\n") + "\n",
    )
    .unwrap();
    let output_directory = scratch.join(&name);
    std::fs::create_dir_all(&output_directory).unwrap();
    let winmd = output_directory.join("Windows.Win32.winmd");
    let started = Instant::now();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(
            result
                .output
                .partitions
                .values()
                .map(|partition| &partition.rdl),
        )
        .output(&winmd)
        .write()
        .unwrap();
    let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
    let rdl: String = result
        .output
        .partitions
        .iter()
        .map(|(stem, partition)| format!("// {stem}\n{}", partition.rdl))
        .collect();
    std::fs::write(scratch.join(format!("{name}.rdl")), &rdl).unwrap();
    println!(
        "mode,count,TUs,capture_prune_ms,merge_probe_ms,project_including_merge_prune_ms,compile_ms,native_records,native_aliases,native_observations,native_types,native_fields,native_owned_bytes,definition_pairs,alias_pairs,type_pairs,capture_omissions,output_records,output_aliases,partitions,omissions,rdl_bytes,winmd_bytes"
    );
    println!(
        "{mode},{count},{},{capture_ms:.3},{merge_ms:.3},{project_ms:.3},{compile_ms:.3},{},{},{},{},{},{},{definition_pairs},{alias_pairs},{type_pairs},{capture_omissions},{},{},{},{},{},{}",
        inputs.len(),
        stats.iter().map(|value| value.records).sum::<usize>(),
        stats.iter().map(|value| value.aliases).sum::<usize>(),
        stats.iter().map(|value| value.observations).sum::<usize>(),
        stats.iter().map(|value| value.types).sum::<usize>(),
        stats.iter().map(|value| value.fields).sum::<usize>(),
        stats.iter().map(|value| value.owned_bytes).sum::<usize>(),
        result.output.records,
        result.output.aliases,
        result.output.partitions.len(),
        result.omissions.len(),
        rdl.len(),
        std::fs::metadata(winmd).unwrap().len()
    );
}
