// #![allow(non_snake_case)]

// use test_winrt_signatures::*;
// use windows::*;
// use Component::Signatures::*;

// #[implement(Component::Signatures::ITest)]
// struct RustTest();

// impl RustTest {
//     fn SignatureBoolean(bool a, out bool b) -> Result<bool> {
//         panic!()
//     }
//     bool[] ArraySignatureBoolean(bool[] a, ref bool[] b, out bool[] c);
//     void CallSignatureBoolean(SignatureBoolean handler);
//     void CallArraySignatureBoolean(ArraySignatureBoolean handler);

//     u8 SignatureUInt8(u8 a, out u8 b);
//     u8[] ArraySignatureUInt8(u8[] a, ref u8[] b, out u8[] c);
//     void CallSignatureUInt8(SignatureUInt8 handler);
//     void CallArraySignatureUInt8(ArraySignatureUInt8 handler);

//     u16 SignatureUInt16(u16 a, out u16 b);
//     u16[] ArraySignatureUInt16(u16[] a, ref u16[] b, out u16[] c);
//     void CallSignatureUInt16(SignatureUInt16 handler);
//     void CallArraySignatureUInt16(ArraySignatureUInt16 handler);

//     u32 SignatureUInt32(u32 a, out u32 b);
//     u32[] ArraySignatureUInt32(u32[] a, ref u32[] b, out u32[] c);
//     void CallSignatureUInt32(SignatureUInt32 handler);
//     void CallArraySignatureUInt32(ArraySignatureUInt32 handler);

//     u64 SignatureUInt64(u64 a, out u64 b);
//     u64[] ArraySignatureUInt64(u64[] a, ref u64[] b, out u64[] c);
//     void CallSignatureUInt64(SignatureUInt64 handler);
//     void CallArraySignatureUInt64(ArraySignatureUInt64 handler);

//     i16 SignatureInt16(i16 a, out i16 b);
//     i16[] ArraySignatureInt16(i16[] a, ref i16[] b, out i16[] c);
//     void CallSignatureInt16(SignatureInt16 handler);
//     void CallArraySignatureInt16(ArraySignatureInt16 handler);

//     i32 SignatureInt32(i32 a, out i32 b);
//     i32[] ArraySignatureInt32(i32[] a, ref i32[] b, out i32[] c);
//     void CallSignatureInt32(SignatureInt32 handler);
//     void CallArraySignatureInt32(ArraySignatureInt32 handler);

//     i64 SignatureInt64(i64 a, out i64 b);
//     i64[] ArraySignatureInt64(i64[] a, ref i64[] b, out i64[] c);
//     void CallSignatureInt64(SignatureInt64 handler);
//     void CallArraySignatureInt64(ArraySignatureInt64 handler);

//     f32 SignatureSingle(f32 a, out f32 b);
//     f32[] ArraySignatureSingle(f32[] a, ref f32[] b, out f32[] c);
//     void CallSignatureSingle(SignatureSingle handler);
//     void CallArraySignatureSingle(ArraySignatureSingle handler);

//     f64 SignatureDouble(f64 a, out f64 b);
//     f64[] ArraySignatureDouble(f64[] a, ref f64[] b, out f64[] c);
//     void CallSignatureDouble(SignatureDouble handler);
//     void CallArraySignatureDouble(ArraySignatureDouble handler);

//     u16 SignatureChar(u16 a, out u16 b);
//     u16[] ArraySignatureChar(u16[] a, ref u16[] b, out u16[] c);
//     void CallSignatureChar(SignatureChar handler);
//     void CallArraySignatureChar(ArraySignatureChar handler);

//     HSTRING SignatureString(HSTRING a, out HSTRING b);
//     HSTRING[] ArraySignatureString(HSTRING[] a, ref HSTRING[] b, out HSTRING[] c);
//     void CallSignatureString(SignatureString handler);
//     void CallArraySignatureString(ArraySignatureString handler);

//     Guid SignatureGuid(Guid a, out Guid b);
//     Guid[] ArraySignatureGuid(Guid[] a, ref Guid[] b, out Guid[] c);
//     void CallSignatureGuid(SignatureGuid handler);
//     void CallArraySignatureGuid(ArraySignatureGuid handler);

//     HRESULT SignatureHResult(HRESULT a, out HRESULT b);
//     HRESULT[] ArraySignatureHResult(HRESULT[] a, ref HRESULT[] b, out HRESULT[] c);
//     void CallSignatureHResult(SignatureHResult handler);
//     void CallArraySignatureHResult(ArraySignatureHResult handler);

//     IInspectable SignatureObject(IInspectable a, out IInspectable b);
//     IInspectable[] ArraySignatureObject(IInspectable[] a, ref IInspectable[] b, out IInspectable[] c);
//     void CallSignatureObject(SignatureObject handler);
//     void CallArraySignatureObject(ArraySignatureObject handler);

//     Component::Simple::Class SignatureClass(Component::Simple::Class a, out Component::Simple::Class b);
//     Component::Simple::Class[] ArraySignatureClass(Component::Simple::Class[] a, ref Component::Simple::Class[] b, out Component::Simple::Class[] c);
//     void CallSignatureClass(SignatureClass handler);
//     void CallArraySignatureClass(ArraySignatureClass handler);

//     Component::Structs::Blittable SignatureBlittable(Component::Structs::Blittable a, ref const Component::Structs::Blittable b, out Component::Structs::Blittable c);
//     Component::Structs::Blittable[] ArraySignatureBlittable(Component::Structs::Blittable[] a, ref Component::Structs::Blittable[] b, out Component::Structs::Blittable[] c);
//     void CallSignatureBlittable(SignatureBlittable handler);
//     void CallArraySignatureBlittable(ArraySignatureBlittable handler);

//     Component::Structs::NonBlittable SignatureNonBlittable(Component::Structs::NonBlittable a, ref const Component::Structs::NonBlittable b, out Component::Structs::NonBlittable c);
//     Component::Structs::NonBlittable[] ArraySignatureNonBlittable(Component::Structs::NonBlittable[] a, ref Component::Structs::NonBlittable[] b, out Component::Structs::NonBlittable[] c);
//     void CallSignatureNonBlittable(SignatureNonBlittable handler);
//     void CallArraySignatureNonBlittable(ArraySignatureNonBlittable handler);

//     Component::Structs::Nested SignatureNested(Component::Structs::Nested a, ref const Component::Structs::Nested b, out Component::Structs::Nested c);
//     Component::Structs::Nested[] ArraySignatureNested(Component::Structs::Nested[] a, ref Component::Structs::Nested[] b, out Component::Structs::Nested[] c);
//     void CallSignatureNested(SignatureNested handler);
//     void CallArraySignatureNested(ArraySignatureNested handler);
// }
