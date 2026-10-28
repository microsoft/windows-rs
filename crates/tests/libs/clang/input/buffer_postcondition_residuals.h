#define SAL(name, args) __attribute__((annotate("windows_clang:sal:2:" name "\n(" args ")\n(" args ")")))

typedef void (*Expression)(
    SAL("_Out_writes_to_", "capacity, written + 1") unsigned short* buffer,
    unsigned int capacity, unsigned int written);
typedef void (*ReturnLength)(
    SAL("_Out_writes_to_", "capacity, return") unsigned short* buffer,
    unsigned int capacity);
typedef void (*IndirectCapacity)(
    SAL("_Out_writes_to_", "*capacity, written") unsigned short* buffer,
    unsigned int* capacity, unsigned int written);
typedef void (*ConstantCapacity)(
    SAL("_Out_writes_z_", "16") unsigned short* buffer);
typedef void (*Conditional)(
    SAL("_When_", "capacity > 0, _Out_writes_z_(capacity)") unsigned short* buffer,
    unsigned int capacity);
typedef void (*PointerDifference)(
    SAL("_Out_writes_to_ptr_z_", "end") unsigned short* buffer, unsigned short* end);
typedef void (*ExtraArguments)(
    SAL("_Out_writes_to_", "capacity, written, other") unsigned short* buffer,
    unsigned int capacity, unsigned int written, unsigned int other);
typedef void (*UnexpectedArguments)(SAL("_In_z_", "unexpected") unsigned short* buffer);
typedef void (*EntryOnly)(
    SAL("_In_z_", "") SAL("_Out_", "") unsigned short* buffer);
typedef void (*EntryAndSuccess)(
    SAL("_In_z_", "") SAL("_Out_writes_z_", "capacity") unsigned short* buffer,
    unsigned int capacity);
typedef void (*OptionalCount)(
    SAL("_Out_writes_to_opt_", "capacity, *written") unsigned short* buffer,
    unsigned int capacity, SAL("_Out_opt_", "") unsigned int* written);
typedef SAL("_Success_", "return != 0") int (*Successful)(
    SAL("_Out_writes_to_", "capacity, *written") unsigned short* buffer,
    unsigned int capacity, unsigned int* written);
