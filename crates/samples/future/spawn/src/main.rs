use windows_future::*;

fn main() {
    let ready = IAsyncOperation::ready(Ok(123));
    println!("ready:   {}", ready.join().unwrap());

    let spawned = IAsyncOperation::spawn(|| Ok(456));
    println!("spawned: {}", spawned.join().unwrap());
}
