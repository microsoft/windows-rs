pub trait IObjectArrayImpl: Sized {
    fn GetCount();
    fn GetAt();
}
pub trait IObjectCollectionImpl: Sized + IObjectArrayImpl {
    fn AddObject();
    fn AddFromArray();
    fn RemoveObjectAt();
    fn Clear();
}
