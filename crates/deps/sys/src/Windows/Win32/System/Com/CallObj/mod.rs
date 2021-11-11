#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CALLFRAMEINFO();
    fn CALLFRAMEPARAMINFO();
    fn CALLFRAME_COPY();
    fn CALLFRAME_FREE();
    fn CALLFRAME_MARSHALCONTEXT();
    fn CALLFRAME_NULL();
    fn CALLFRAME_WALK();
    fn CoGetInterceptor();
    fn CoGetInterceptorFromTypeInfo();
    fn ICallFrame();
    fn ICallFrameEvents();
    fn ICallFrameWalker();
    fn ICallIndirect();
    fn ICallInterceptor();
    fn ICallUnmarshal();
    fn IInterfaceRelated();
}
