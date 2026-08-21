#![windows_subsystem = "windows"]

fn main() {
    let active = !std::env::args().any(|argument| argument == "--baseline");
    let samples = std::env::args()
        .skip_while(|argument| argument != "--samples")
        .nth(1)
        .map_or(300, |value| value.parse::<usize>().unwrap());
    sample_reactor_next_virtual::performance::run_live(samples, active);
}
