use windows_ecma335::*;

fn main() {
    let time = std::time::Instant::now();

    let file = reader::File::read("all.winmd").unwrap();

    //let r = file.equal_range_str::<reader::TypeDef>(2, "Windows.Foundation");
    let r = file.namespace("Windows.Foundation");

    for td in r.clone() {
        dbg!(td);
    }

    println!("len: {}", r.len());

    let mut r = file.get("Windows.Foundation", "IAsyncOperation`1");
    println!("len: {}", r.len());
    dbg!(r.next());

    let all = file.TypeDef();
    println!("all: {}", all.len());

    for td in all.clone() {
        let find = file
            .get(td.namespace(), td.name())
            .next()
            .unwrap_or_else(|| panic!("{td:?}"));
        assert_eq!(td, find);
    }

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
