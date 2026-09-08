use std::collections::BTreeMap;
use windows_collections::*;
use windows_core::*;

fn main() -> Result<()> {
    let numbers = IVector::<i32>::from(vec![1, 2, 3]);
    numbers.Append(4)?;

    print!("vector:");
    for number in &numbers {
        print!(" {number}");
    }
    println!();

    let scores = IMap::<i32, i32>::from(BTreeMap::from([(1, 10), (2, 20)]));
    scores.Insert(3, 30)?;

    println!("map:");
    for pair in &scores {
        println!("  {} -> {}", pair.Key()?, pair.Value()?);
    }

    Ok(())
}
