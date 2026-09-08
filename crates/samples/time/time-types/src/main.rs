use windows_time::*;

fn main() {
    let span = TimeSpan::from_minutes(90);
    println!("90 minutes = {} seconds", span.whole_seconds());

    let start = DateTime::from_unix_secs(1_000_000_000);
    let later = start.checked_add(span).unwrap();

    println!("start: {} (unix secs)", start.unix_secs());
    println!("later: {} (unix secs)", later.unix_secs());
}
