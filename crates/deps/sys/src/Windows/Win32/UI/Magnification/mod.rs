#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn MagGetColorEffect();
    fn MagGetFullscreenColorEffect();
    fn MagGetFullscreenTransform();
    fn MagGetImageScalingCallback();
    fn MagGetInputTransform();
    fn MagGetWindowFilterList();
    fn MagGetWindowSource();
    fn MagGetWindowTransform();
    fn MagInitialize();
    fn MagSetColorEffect();
    fn MagSetFullscreenColorEffect();
    fn MagSetFullscreenTransform();
    fn MagSetImageScalingCallback();
    fn MagSetInputTransform();
    fn MagSetWindowFilterList();
    fn MagSetWindowSource();
    fn MagSetWindowTransform();
    fn MagShowSystemCursor();
    fn MagUninitialize();
}
