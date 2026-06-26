fn main() {
    let counter = std::sync::RwLock::<usize>::new(0);

    windows_threading::for_each(0..10, |value| {
        *counter.write().unwrap() += value;
    });

    println!("sum of 0..10 = {}", counter.read().unwrap());
}
