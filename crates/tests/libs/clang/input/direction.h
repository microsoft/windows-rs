#define SAL(name) __attribute__((annotate("windows_clang:sal:2:" #name "\n()\n()")))
#define WHEN(condition, annotation) __attribute__((annotate("windows_clang:sal:2:_When_\n(" #condition "," #annotation ")\n(" #condition "," #annotation ")")))
#define QUERY WHEN(data != 0, _Inout_opt_) WHEN(data == 0, _Out_opt_)
typedef unsigned long* Count;
typedef void* Data;
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IObject {
    virtual void __stdcall Apply() = 0;
};
#define UNKNOWN int scalar, int* pointer, const int* constant, Count alias, int& reference, IObject* object
extern "C" void Unknown(UNKNOWN);
typedef void (__stdcall *UnknownCallback)(UNKNOWN);
struct __declspec(uuid("22222222-2222-3333-4444-555555555555")) IDirections {
    virtual void __stdcall Unknown(UNKNOWN) = 0;
    virtual void __stdcall Query(void* data, QUERY Count size) = 0;
};
extern "C" void Known(SAL(_In_) int* input, SAL(_Out_) int* output,
    SAL(_Inout_) Count both, SAL(_In_opt_) IObject* object);
extern "C" void Query(Data data, QUERY Count size);
typedef void (__stdcall *QueryCallback)(void* data, QUERY Count size);
extern "C" void Reversed(void* data,
    WHEN(data==0, _Out_opt_) WHEN(data!=0, _Inout_opt_) Count size);
extern "C" void MixedOptional(void* data,
    WHEN(data != 0, _Inout_) WHEN(data == 0, _Out_opt_) Count size);
extern "C" void Required(void* data, SAL(_Out_) QUERY Count size);
extern "C" void Disjoint(void* data,
    WHEN(data != 0, _In_opt_) WHEN(data == 0, _Out_opt_) Count size);
extern "C" void Partial(void* data, WHEN(data != 0, _Inout_opt_) Count size);
extern "C" void NonComplementary(void* data,
    WHEN(data != 0, _Inout_opt_) WHEN(data != 0, _Out_opt_) Count size);
extern "C" void DifferentSubjects(void* data, void* other,
    WHEN(data != 0, _Inout_opt_) WHEN(other == 0, _Out_opt_) Count size);
extern "C" void IntegerSubject(int data, QUERY Count size);
extern "C" void Complex(void* data,
    WHEN(data != 0 && size != 0, _Inout_opt_) WHEN(data == 0, _Out_opt_) Count size);
extern "C" void UnknownBranch(void* data,
    WHEN(data != 0, _Custom_) WHEN(data == 0, _Out_opt_) Count size);
extern "C" void Scoped(void* data, SAL(_Post_) QUERY Count size);
