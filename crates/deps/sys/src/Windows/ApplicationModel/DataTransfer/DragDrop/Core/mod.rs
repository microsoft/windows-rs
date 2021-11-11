#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CoreDragDropManager();
    fn CoreDragInfo();
    fn CoreDragOperation();
    fn CoreDragUIContentMode();
    fn CoreDragUIOverride();
    fn CoreDropOperationTargetRequestedEventArgs();
    fn ICoreDragDropManager();
    fn ICoreDragDropManagerStatics();
    fn ICoreDragInfo();
    fn ICoreDragInfo2();
    fn ICoreDragOperation();
    fn ICoreDragOperation2();
    fn ICoreDragUIOverride();
    fn ICoreDropOperationTarget();
    fn ICoreDropOperationTargetRequestedEventArgs();
}
