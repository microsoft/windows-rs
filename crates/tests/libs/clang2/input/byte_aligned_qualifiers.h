struct Bytes { unsigned char value[3]; };
#pragma pack(push, 1)
struct Packed { unsigned short size; unsigned char value; };
#pragma pack(pop)

#ifdef PLAIN
#define QUALIFIER
#else
#define QUALIFIER __unaligned
#endif

typedef QUALIFIER unsigned char Byte;
typedef QUALIFIER Bytes QualifiedBytes;
typedef QUALIFIER Packed* PackedPointer;
typedef PackedPointer InheritedPointer;
typedef const QUALIFIER Packed* ConstPackedPointer;
typedef QUALIFIER Packed PackedArray[2];
typedef void (*Callback)(PackedPointer value);

struct Uses {
    QualifiedBytes* bytes;
    Byte* scalar;
    PackedPointer writable;
    ConstPackedPointer readonly;
    InheritedPointer inherited;
    PackedArray array;
    Callback callback;
};
extern "C" void BufferFunction(ConstPackedPointer value, Byte* bytes);
