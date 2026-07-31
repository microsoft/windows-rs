#![expect(nonstandard_style)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("target arch: {}\n", std::env::consts::ARCH);
    println!("        size   align");
    println!(
        "M128A   {:>4}      {:>2}",
        size_of::<M128A>(),
        align_of::<M128A>()
    );
    println!(
        "CONTEXT {:>4}      {:>2}",
        size_of::<CONTEXT>(),
        align_of::<CONTEXT>()
    );
}
