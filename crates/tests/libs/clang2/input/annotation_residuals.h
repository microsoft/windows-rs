#define SAL(name, args) __attribute__((annotate("windows_clang:sal:2:" name "\n(" args ")\n(" args ")")))

struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IObject {
    virtual void __stdcall Run() = 0;
};
typedef void (*Conditional)(SAL("_When_", "ready,_In_") int* value, int ready);
typedef void (*Mixed)(SAL("_In_", "") SAL("_Out_", "") int* value);
typedef void (*NullableResult)(SAL("_Outptr_result_maybenull_", "") void** value);
typedef void (*ObjectInput)(SAL("_In_", "") IObject* value);
typedef void (*ObjectOutput)(SAL("_Out_", "") IObject* value);
typedef void (*ObjectUpdate)(SAL("_Inout_", "") IObject* value);
typedef void (*Terminated)(SAL("_In_z_", "") const char* value);
typedef void (*UnexpectedArguments)(SAL("_In_", "unexpected") int* value);
struct FieldOwner {
    SAL("_In_", "") int* value;
};
