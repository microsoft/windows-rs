use std::cell::Cell;
use std::rc::Rc;

use windows::core::Owned;
use windows::core::Free;

struct FreeCounter {
    pub count: Rc<Cell<u32>>,
}

impl Free for FreeCounter {
    unsafe fn free(&mut self) {
        self.count.update(|c| c + 1);
    }
}

#[test]
fn into_raw() {
    let free_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter = FreeCounter { count: free_count.clone() };

    {
        let owned = unsafe { Owned::new(counter) };
        let _counter = unsafe { Owned::into_raw(owned) };
        assert!(free_count.get() == 0);
    }

    assert!(free_count.get() == 0);
}

#[test]
fn drop() {
    let free_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter = FreeCounter { count: free_count.clone() };

    {
        let _owned = unsafe { Owned::new(counter) };
        assert!(free_count.get() == 0);
    }

    assert!(free_count.get() == 1);
}
