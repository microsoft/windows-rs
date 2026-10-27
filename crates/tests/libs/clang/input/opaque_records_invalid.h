struct Opaque;
typedef Opaque Alias;
extern "C" Opaque Make();
extern "C" int Consume(Alias value);
typedef Opaque (*Callback)();
struct Empty {};
struct Unsupported { const int value; };
struct Consumer { Unsupported* value; };
union Unknown;
struct __declspec(align(16)) Aligned;
struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) Identified;
struct Parent {
    struct Child;
    Child* value;
};
struct Base { int value; };
struct Derived : Base { int extra; };
extern "C" void Inherit(Derived value);
