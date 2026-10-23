use std::time::Instant;
use windows_clang::{Input, Snapshot, Target, capture, merge};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const ROOTS: &[&str] = &[
    "LIST_ENTRY",
    "SINGLE_LIST_ENTRY",
    "FILETIME",
    "SYSTEMTIME",
    "POINT",
    "RECT",
    "SIZE",
    "POINTS",
    "RGBQUAD",
    "BITMAPINFOHEADER",
    "BITMAPFILEHEADER",
    "SECURITY_ATTRIBUTES",
    "PROCESS_INFORMATION",
    "STARTUPINFOW",
    "WIN32_FIND_DATAW",
    "OSVERSIONINFOW",
    "OSVERSIONINFOEXW",
    "MEMORYSTATUSEX",
    "OVERLAPPED",
    "GUID",
    "CONTEXT",
    "SYSTEM_INFO",
    "LOGFONTW",
    "BY_HANDLE_FILE_INFORMATION",
    "WIN32_FILE_ATTRIBUTE_DATA",
    "TIME_ZONE_INFORMATION",
];

pub fn inventory() -> bool {
    let mut passed = 0;
    for root in ROOTS {
        let mut input = test_clang::sdk_fixture();
        input.roots = vec![(*root).into()];
        match capture(&[input], &TARGETS) {
            Err(error) => println!(
                "{root}: CAPTURE BLOCKED: {}",
                error.to_string().lines().last().unwrap()
            ),
            Ok(snapshots) => match merge(&snapshots) {
                Err(error) => println!("{root}: MERGE BLOCKED: {error}"),
                Ok(merged) => {
                    passed += 1;
                    println!(
                        "{root}: VERIFIED ({} records, {} enums, {} aliases)",
                        merged.records.len(),
                        merged.enums.len(),
                        merged.aliases.len()
                    );
                }
            },
        }
    }
    println!(
        "{passed}/{} roots passed capture and merge on all three targets",
        ROOTS.len()
    );
    passed == ROOTS.len()
}

pub fn scale(tus: usize) {
    assert!(
        matches!(tus, 2 | 8 | 32),
        "SDK scale cases are 2, 8, or 32 TUs"
    );
    let mut template = test_clang::sdk_fixture();
    template.roots = [
        "LIST_ENTRY",
        "SINGLE_LIST_ENTRY",
        "FILETIME",
        "SYSTEMTIME",
        "POINT",
        "RECT",
        "SIZE",
        "SECURITY_ATTRIBUTES",
        "PROCESS_INFORMATION",
        "STARTUPINFOW",
        "WIN32_FIND_DATAW",
        "OSVERSIONINFOW",
        "BY_HANDLE_FILE_INFORMATION",
        "WIN32_FILE_ATTRIBUTE_DATA",
        "TIME_ZONE_INFORMATION",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let inputs: Vec<_> = (0..tus)
        .map(|id| Input {
            name: format!("sdk_{id:03}.h"),
            source: template.source.clone(),
            arguments: template.arguments.clone(),
            roots: template.roots.clone(),
            namespace_headers: template.namespace_headers.clone(),
            root_paths: template.root_paths.clone(),
            function_symbols: template.function_symbols.clone(),
            external_interfaces: template.external_interfaces.clone(),
            macros: template.macros.clone(),
            macro_headers: template.macro_headers.clone(),
        })
        .collect();
    let started = Instant::now();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let capture_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let merged = merge(&snapshots).unwrap();
    let merge_ms = started.elapsed().as_secs_f64() * 1000.0;
    let stats: Vec<_> = snapshots.iter().map(Snapshot::statistics).collect();
    for resolution in merged.targets.values() {
        assert_eq!(
            resolution.work.definition_pairs,
            resolution.groups.len() * (tus - 1)
        );
        assert!(resolution.work.type_pairs <= resolution.snapshots[0].types.len() * (tus - 1));
    }
    println!(
        "TUs,targets,roots,records,enums,aliases,observations,owned_bytes,capture_ms,merge_ms,definition_pairs,type_pairs,enum_pairs"
    );
    println!(
        "{tus},3,{},{},{},{},{},{},{capture_ms:.3},{merge_ms:.3},{},{},{}",
        template.roots.len(),
        stats.iter().map(|s| s.records).sum::<usize>(),
        stats.iter().map(|s| s.enums).sum::<usize>(),
        stats.iter().map(|s| s.aliases).sum::<usize>(),
        stats.iter().map(|s| s.observations).sum::<usize>(),
        stats.iter().map(|s| s.owned_bytes).sum::<usize>(),
        merged
            .targets
            .values()
            .map(|r| r.work.definition_pairs)
            .sum::<usize>(),
        merged
            .targets
            .values()
            .map(|r| r.work.type_pairs)
            .sum::<usize>(),
        merged
            .targets
            .values()
            .map(|r| r.work.enum_pairs)
            .sum::<usize>()
    );
}
