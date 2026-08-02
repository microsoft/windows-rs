pub use windows_animation::*;

pub fn bar(value: f64, max: f64) -> String {
    let width = 40.0;
    let filled = ((value / max) * width).round().clamp(0.0, width) as usize;
    "#".repeat(filled)
}
