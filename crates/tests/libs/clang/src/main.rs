use std::time::Instant;
use test_clang::{fixture, shared_graph};
use windows_clang::{Snapshot, Target, capture, merge};

mod gate;
mod sweep;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    helpers::ensure_libclang();
    match args.first().map(String::as_str) {
        Some("scale") if args.len() == 1 => scale(),
        Some("sdk-gate") if args.len() == 1 => {
            if !gate::inventory() {
                std::process::exit(1);
            }
        }
        Some("sdk-scale") if args.len() == 2 => gate::scale(args[1].parse().unwrap()),
        Some("sdk-sweep") if args.len() == 3 => sweep::run(&args[1], args[2].parse().unwrap()),
        Some("inspect") | None => inspect(&args),
        _ => {
            eprintln!(
                "usage: test_clang [inspect [fixture ...] | scale | sdk-gate | sdk-scale 2|8|32 | sdk-sweep breadth 1|2|4|8 | sdk-sweep tus 2|8|32]"
            );
            std::process::exit(2);
        }
    }
}

fn inspect(args: &[String]) {
    let names: Vec<_> = if args.len() <= 1 {
        vec!["shared_a", "shared_b"]
    } else {
        args[1..].iter().map(String::as_str).collect()
    };
    let inputs: Vec<_> = names.into_iter().map(fixture).collect();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    for (target, resolved) in &merged.targets {
        println!(
            "{:?}: {} TUs, {} records, {} enums -> {} record groups, {} enum groups",
            target,
            resolved.snapshots.len(),
            resolved
                .snapshots
                .iter()
                .map(|snapshot| snapshot.records.len())
                .sum::<usize>(),
            resolved
                .snapshots
                .iter()
                .map(|snapshot| snapshot.enums.len())
                .sum::<usize>(),
            resolved.groups.len(),
            resolved.enums.len()
        );
        for group in &resolved.groups {
            print!("  {}", resolved.name(group));
            if let Some(definition) = resolved.definition(group) {
                println!(
                    " size={} align={} field_offsets_bits={:?}",
                    definition.layout.size,
                    definition.layout.alignment,
                    definition
                        .fields
                        .iter()
                        .map(|field| (&field.name, field.offset_bits))
                        .collect::<Vec<_>>()
                );
            } else {
                println!(" incomplete");
            }
            for group in &resolved.enums {
                let reference = group.enums[0];
                let enumeration =
                    &resolved.snapshots[reference.snapshot].enums[reference.enumeration.0];
                print!("  enum {}", enumeration.name);
                if let Some(reference) = group.definition {
                    let snapshot = resolved.snapshots[reference.snapshot];
                    let observation = &snapshot.enums[reference.enumeration.0].observations
                        [reference.observation];
                    println!(
                        " backing={:?} scoped={} members={:?}",
                        snapshot.types[observation.canonical.0].kind,
                        observation.scoped,
                        observation
                            .members
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|member| (&member.name, &member.value))
                            .collect::<Vec<_>>()
                    );
                } else {
                    println!(" incomplete");
                }
            }
        }
    }
}

fn scale() {
    println!(
        "records_per_TU,TUs,targets,source_bytes,capture_ms,merge_ms,owned_bytes,entities,observations,types,fields,definition_pairs,type_pairs"
    );
    for count in [128, 512, 2048, 8192] {
        let inputs = shared_graph(count);
        let source_bytes: usize = inputs.iter().map(|input| input.source.len()).sum();
        let started = Instant::now();
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let capture_time = started.elapsed();
        let started = Instant::now();
        let merged = merge(&snapshots).unwrap();
        let resolve_time = started.elapsed();
        let stats: Vec<_> = snapshots.iter().map(Snapshot::statistics).collect();
        println!(
            "{count},2,3,{source_bytes},{:.3},{:.3},{},{},{},{},{},{},{}",
            capture_time.as_secs_f64() * 1000.0,
            resolve_time.as_secs_f64() * 1000.0,
            stats.iter().map(|s| s.owned_bytes).sum::<usize>(),
            stats.iter().map(|s| s.records).sum::<usize>(),
            stats.iter().map(|s| s.observations).sum::<usize>(),
            stats.iter().map(|s| s.types).sum::<usize>(),
            stats.iter().map(|s| s.fields).sum::<usize>(),
            merged
                .targets
                .values()
                .map(|r| r.work.definition_pairs)
                .sum::<usize>(),
            merged
                .targets
                .values()
                .map(|r| r.work.type_pairs)
                .sum::<usize>()
        );
    }
}
