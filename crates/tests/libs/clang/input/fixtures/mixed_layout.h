struct __declspec(align(16)) Aligned { int value; };
struct Inherited { Aligned value; };
struct alignas(16) StandardAligned { int value; };
#pragma pack(push, 4)
struct PackedNatural { char tag; long long value; };
struct PackedChild { char tag; Aligned value; };
struct PackedInherited { char tag; Inherited value; };
struct PackedStandard { char tag; StandardAligned value; };
struct PackedMember { char tag; __declspec(align(16)) int value; char tail; };
struct __declspec(align(16)) PackedWhole { char tag; long long value; };
struct PackedMixed { char tag; void* pointer; Aligned value; char tail; };
struct PackedArray { char tag; Aligned values[2]; };
struct PackedNames { char __padding_1; __declspec(align(16)) int value; };
union PackedUnion { Aligned aligned; long long number; char bytes[19]; };
struct PackedBitfields { char tag; unsigned long long low : 3; unsigned long long high : 5; Aligned value; };
#pragma pack(pop)
struct Container { PackedMixed values[2]; char tail; };
