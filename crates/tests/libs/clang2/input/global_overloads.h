#ifndef NATIVE_TYPE
#define NATIVE_TYPE int
#endif
inline int NativeSum(double value) {
    struct Local { int& value; };
    return static_cast<int>(value);
}
extern "C" int NativeSum(NATIVE_TYPE value);
struct Payload { int value; };
extern "C" int __stdcall ReadPayload(const Payload* value, unsigned int* written);
inline int ReadPayload(const Payload& value, unsigned int* written);
inline int ReadPayload(const Payload& value, unsigned int* written) {
    return ReadPayload(&value, written);
}
