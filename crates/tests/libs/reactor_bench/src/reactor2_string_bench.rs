mod allocator;

use std::hint::black_box;
use std::mem::size_of;
use std::rc::Rc;
use std::time::Instant;
use windows_core::HSTRING;

#[derive(Clone, Copy)]
struct Options {
    iterations: usize,
    text_size: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            iterations: 1_000,
            text_size: 100_000,
        }
    }
}

impl Options {
    fn parse() -> Result<Option<Self>, String> {
        let mut options = Self::default();
        let mut arguments = std::env::args().skip(1);
        while let Some(argument) = arguments.next() {
            let target = match argument.as_str() {
                "--iterations" => &mut options.iterations,
                "--text-size" => &mut options.text_size,
                "--help" | "-h" => {
                    println!(
                        "reactor2-string-bench\n\
                         \n\
                         Measures candidate Reactor2 string representations.\n\
                         \n\
                         Options:\n\
                           --iterations N  Operations per row (default: 1000)\n\
                           --text-size N   ASCII text length (default: 100000)\n\
                           -h, --help      Print this help"
                    );
                    return Ok(None);
                }
                _ => return Err(format!("unknown argument: {argument}")),
            };
            let value = arguments
                .next()
                .ok_or_else(|| format!("{argument} requires a value"))?;
            *target = value
                .parse()
                .map_err(|_| format!("invalid {argument} value: {value}"))?;
        }
        if options.iterations == 0 {
            return Err("--iterations must be greater than zero".to_string());
        }
        Ok(Some(options))
    }
}

struct ResultRow {
    name: &'static str,
    ns_per_op: f64,
    allocations_per_op: f64,
    bytes_per_op: f64,
}

fn measure<T>(
    name: &'static str,
    iterations: usize,
    mut operation: impl FnMut() -> T,
) -> ResultRow {
    for _ in 0..10 {
        black_box(operation());
    }
    let allocations = allocator::ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed);
    let bytes = allocator::allocated_bytes();
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(operation());
    }
    let elapsed = start.elapsed();
    ResultRow {
        name,
        ns_per_op: elapsed.as_nanos() as f64 / iterations as f64,
        allocations_per_op: (allocator::ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed)
            - allocations) as f64
            / iterations as f64,
        bytes_per_op: (allocator::allocated_bytes() - bytes) as f64 / iterations as f64,
    }
}

fn main() {
    let Some(options) = Options::parse().unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2);
    }) else {
        return;
    };

    let source = "x".repeat(options.text_size);
    let hstring = HSTRING::from(source.as_str());
    let string = source.clone();
    let rc_str = Rc::<str>::from(source.as_str());
    let rc_string = Rc::new(source.clone());

    let rows = [
        measure("str -> String", options.iterations, || source.clone()),
        measure("str -> Rc<str>", options.iterations, || {
            Rc::<str>::from(source.as_str())
        }),
        measure("str -> Rc<String>", options.iterations, || {
            Rc::new(source.clone())
        }),
        measure("str -> HSTRING", options.iterations, || {
            HSTRING::from(source.as_str())
        }),
        measure("String clone", options.iterations, || string.clone()),
        measure("Rc<str> clone", options.iterations, || Rc::clone(&rc_str)),
        measure("Rc<String> clone", options.iterations, || {
            Rc::clone(&rc_string)
        }),
        measure("HSTRING clone", options.iterations, || hstring.clone()),
        measure("HSTRING -> String", options.iterations, || {
            hstring.to_string_lossy()
        }),
        measure("HSTRING -> String -> Rc<str>", options.iterations, || {
            Rc::<str>::from(hstring.to_string_lossy())
        }),
        measure("HSTRING -> Rc<String>", options.iterations, || {
            Rc::new(hstring.to_string_lossy())
        }),
    ];

    println!(
        "text size: {} bytes, iterations: {}",
        options.text_size, options.iterations
    );
    println!(
        "inline sizes: String={} Rc<str>={} Rc<String>={} HSTRING={}",
        size_of::<String>(),
        size_of::<Rc<str>>(),
        size_of::<Rc<String>>(),
        size_of::<HSTRING>()
    );
    println!("operation                              ns/op   allocs/op     bytes/op");
    for row in rows {
        println!(
            "{:<34} {:>10.1} {:>11.2} {:>12.0}",
            row.name, row.ns_per_op, row.allocations_per_op, row.bytes_per_op
        );
    }
}
