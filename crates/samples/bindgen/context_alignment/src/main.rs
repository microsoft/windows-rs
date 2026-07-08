// Generates `CONTEXT` from the in-house Win32 metadata with `windows-bindgen`
// and prints its size and alignment. This confirms the metadata scrapes the
// architecture-specific `CONTEXT` (and its `__declspec(align(16))` `M128A`
// members) with correct layout — including the 16-byte alignment on x64/arm64.

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
