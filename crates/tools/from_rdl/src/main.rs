//use windows_ecma335::*;

fn main() {
    let time = std::time::Instant::now();

    let file = std::fs::read_to_string("rdl.rs").unwrap();

    let rx = regex::Regex::new(r"(?m)^(\s*)(class|interface)\s").unwrap();

    let result = rx.replace_all(&file, |caps: &regex::Captures| {
        format!("{}#[{}]\n{}trait ", &caps[1], &caps[2], &caps[1])
    });

    std::fs::write("rdl.rs", &*result).unwrap();

    let _syntax = syn::parse_file(&*result).unwrap_or_else(|e| panic!("{:?}", e.span().start()));

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
