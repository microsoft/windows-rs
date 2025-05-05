use windows_threading::*;

fn send<T: Send>(_: T) {}
fn sync<T: Sync>(_: T) {}

#[test]
fn test() {
    send(Pool::new());
    sync(Pool::new());
}
