#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BindableVectorChangedEventHandler();
    fn IBindableIterable();
    fn IBindableIterator();
    fn IBindableObservableVector();
    fn IBindableVector();
    fn IBindableVectorView();
    fn INotifyCollectionChanged();
    fn INotifyCollectionChangedEventArgs();
    fn INotifyCollectionChangedEventArgsFactory();
    fn NotifyCollectionChangedAction();
    fn NotifyCollectionChangedEventArgs();
    fn NotifyCollectionChangedEventHandler();
    fn TypeKind();
    fn TypeName();
}
