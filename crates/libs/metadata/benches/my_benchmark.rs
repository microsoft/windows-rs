use criterion::{black_box, criterion_group, criterion_main, Criterion};
use windows_metadata::reader::File;
use windows_metadata::reader::Reader;

const EXCLUDE_NAMESPACES: [&str; 1] = ["Windows.Win32.Interop"];

pub fn criterion_benchmark(c: &mut Criterion) {
    let files = vec![File::new("default/Windows.winmd").unwrap(), File::new("default/Windows.Win32.winmd").unwrap(), File::new("default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &Reader::new(&files);
    let root = reader.tree("Windows", &EXCLUDE_NAMESPACES).expect("`Windows` namespace not found");

    let mut group = c.benchmark_group("flatten");

    group.bench_function("flatten_master", |b| {
        b.iter(|| {
            let root = black_box(&root);
            root.flatten()
        })
    });

    group.bench_function("flatten_iterative", |b| {
        b.iter(|| {
            let root = black_box(&root);
            root.flatten_iter()
        })
    });

    group.bench_function("flatten_recursive", |b| {
        b.iter(|| {
            let root = black_box(&root);
            root.flatten_rec()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
