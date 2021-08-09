use std::vec;
use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;

#[implement(
    Windows::Foundation::Collections::IVectorView<T>,
)]
struct TestView<T>(Vec<T>)
where 
    T: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<T: ::windows::RuntimeType + 'static> TestView<T> {
    /* GetAt returns the value at the given index, and panic if given an invalid index */
    fn GetAt(&self, index : u32) -> Result<T> {
        let vec_size = self.0.len() as u32;
        assert!(index < vec_size, "Given index ({}) was out of bounds for the IVectorView (length {})", index, vec_size);
        self.0.get(index as usize).cloned().ok_or_else(|| panic!())
    }
   
    /* Size returns the length of the underlying vector */
    fn Size(&self) -> Result<u32> {
        let size : u32 = self.0.len() as u32;
        Ok(size)
    }

    /* IndexOf returns true if the given value is in the IVectorView, and false otherwise.
        If the given value is found, then its index is written to the `index` parameter */
    fn IndexOf(&self, value : &T::DefaultType, index : &mut u32) -> Result<bool> {
        // note: DefaultType covers nullable types; expand impl block to have constraints on when T is an Option and not 
        let vec_size = self.0.len();
        for i in 0..vec_size {
            let vec_element =  self.0.get(i).unwrap();
         
            //  Equality check here is tricky, cannot depend on `==` when T doesnt derive Eq 
            // if *vec_element == *value {
            if false {
                *index = i as u32;
                return Ok(true);
            }
        }
        return Ok(false);
    }

    /* GetMany retrieves multiple items, storing them in `items`, and returns the amount of items retrieved  */
    fn GetMany(&self, start_index : u32, items : &mut [T]) -> Result<u32> {
        let vec_size : u32 = self.0.len() as u32;
        assert!(start_index < vec_size, "Start index ({}) was out of bounds for the IVectorView (length {})", start_index, vec_size);
        
        // (?) assert that the incoming `items` array is sufficiently long enough (?)

        let mut get_many_size : u32 = 0;
        for n in start_index..vec_size {
            let idx = n as usize;
            items[idx] = self.0.get(idx).unwrap().clone();
            get_many_size += 1;
        }
        Ok(get_many_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* 
        Tests using an int vector (size 3) 
    */

    #[test]
    fn test_get_at_three_vec() -> Result<()> {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();
        assert_eq!(5, three_vec.GetAt(0)?);
        assert_eq!(120, three_vec.GetAt(1)?);
        assert_eq!(625, three_vec.GetAt(2)?);
        Ok(())
    }

    #[test]
    fn test_size_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();
        assert_eq!(3, three_vec.Size().unwrap());
    }

    // test_index_of_three_vec ALWAYS FAILS until we fix the `==` issue 
    #[test]
    fn test_index_of_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();
 
        // ? correct way to get a pointer to a u32 ?
        let mut idx = &mut 1;
        
        let index_of_five = three_vec.IndexOf(5, idx).unwrap();
        assert!(index_of_five);
        assert_eq!(0, *idx);
    }

    // test_get_many_three_vec ALWAYS FAILS until we have proper array param support
    #[test]
    fn test_get_many_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();

        // add a test where the items array (here, `arr`) has size 0
        let mut arr = &mut [0;3];
        three_vec.GetMany(1, arr).unwrap(); 
        assert_eq!(120, arr[0]);
        assert_eq!(625, arr[1]);
    }

    /* 
        Tests using an empty vector 
    */

    #[test]
    #[should_panic(expected = "Given index (0) was out of bounds for the IVectorView (length 0)")]
    #[allow(unused_must_use)]
    fn test_get_at_empty_vec() {
        let empty_vec : IVectorView<bool> = TestView(vec![]).into();
        empty_vec.GetAt(0);
    }

    #[test]
    fn test_size_empty_vec() {
        let empty_vec : IVectorView<bool> = TestView(vec![]).into();
        assert_eq!(0, empty_vec.Size().unwrap());
    }

    #[test]
    fn test_index_of_empty_vec() {
        let empty_vec : IVectorView<bool> = TestView(vec![]).into();
        
        // ? correct way to get a pointer to a u32 ?
        let idx = &mut 1;
        
        let index_of_anything = empty_vec.IndexOf(true, idx).unwrap();
        assert!(!index_of_anything);
    }
    
    /* todo: test using Vectors of different types (e.g. HSTRING, Uri, ...) */
}