pub fn panic(message: &str) -> ! {
    panic!("windows-bindgen: {message}");
}

pub fn with_path(message: &str, path: &str) -> ! {
    panic!("windows-bindgen: {message} `{path}`");
}
