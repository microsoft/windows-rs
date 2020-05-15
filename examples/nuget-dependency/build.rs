fn main() {
    let current = std::env::current_dir().unwrap();
    let src = current.join("nuget");
    let target = current.join("target").join("nuget");
    if !target.exists() {
        std::os::windows::fs::symlink_dir(src, target).unwrap();
    }
}
