#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CLSID_MILBitmapEffectBevel();
    fn CLSID_MILBitmapEffectBlur();
    fn CLSID_MILBitmapEffectDropShadow();
    fn CLSID_MILBitmapEffectEmboss();
    fn CLSID_MILBitmapEffectGroup();
    fn CLSID_MILBitmapEffectOuterGlow();
    fn IMILBitmapEffect();
    fn IMILBitmapEffectConnections();
    fn IMILBitmapEffectConnectionsInfo();
    fn IMILBitmapEffectConnector();
    fn IMILBitmapEffectConnectorInfo();
    fn IMILBitmapEffectEvents();
    fn IMILBitmapEffectFactory();
    fn IMILBitmapEffectGroup();
    fn IMILBitmapEffectGroupImpl();
    fn IMILBitmapEffectImpl();
    fn IMILBitmapEffectInputConnector();
    fn IMILBitmapEffectInteriorInputConnector();
    fn IMILBitmapEffectInteriorOutputConnector();
    fn IMILBitmapEffectOutputConnector();
    fn IMILBitmapEffectOutputConnectorImpl();
    fn IMILBitmapEffectPrimitive();
    fn IMILBitmapEffectPrimitiveImpl();
    fn IMILBitmapEffectRenderContext();
    fn IMILBitmapEffectRenderContextImpl();
    fn IMILBitmapEffects();
    fn MILBITMAPEFFECT_SDK_VERSION();
    fn MILMatrixF();
    fn MilPoint2D();
    fn MilRectD();
}
