#pragma pack(push, 1)
struct One { char value; };
#pragma pack(pop)
#pragma pack(push, 2)
struct Two { char value; };
union PackedUnion { char tag; long long value; };
struct Nested { char tag; PackedUnion value; };
#pragma pack(pop)
#pragma pack(push, 4)
struct Four { char tag; long long value; };
#pragma pack(pop)
#pragma pack(push, 8)
struct Eight { char tag; long long value; };
#pragma pack(pop)
#pragma pack(push, 16)
struct Sixteen { char tag; long long value; };
#pragma pack(pop)
