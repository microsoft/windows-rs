#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(push)
#pragma warning(disable: 4121 4201 4324)
#endif

typedef unsigned long BitWord;

struct BitUnits {
    unsigned char flag : 1;
    unsigned char : 2;
    unsigned char mode : 5;
    unsigned short low : 4;
    unsigned short high : 12;
    BitWord first : 3;
    unsigned long second : 29;
    unsigned long next : 2;
    unsigned long : 0;
    unsigned long after : 7;
    unsigned long long wide : 40;
    unsigned long long tail : 24;
    unsigned long __bitfield0;
    unsigned char end;
};

#pragma pack(push, 1)
struct BitPacked {
    unsigned char tag;
    unsigned long enabled : 1;
    unsigned long level : 7;
    unsigned long : 24;
    unsigned long value;
};
#pragma pack(pop)

struct BitNested {
    BitPacked entries[2];
    union {
        struct {
            unsigned long count : 11;
            unsigned long remaining : 21;
        } bits;
        unsigned long raw;
    } choice;
};

struct BitFull {
    unsigned long long whole : 64;
};

typedef void (*BitCallback)(BitUnits* value, BitPacked* packed);
extern "C" unsigned long BitLayoutEvidence(unsigned long index);
extern "C" unsigned long long BitRead(const BitUnits* value, unsigned long index);
extern "C" void BitWrite(BitUnits* value, unsigned long index, unsigned long long input);
extern "C" unsigned long BitPackedRead(const BitPacked* value);
extern "C" void BitPackedWrite(BitPacked* value, unsigned long input);
extern "C" void BitInvoke(BitCallback callback, BitUnits* value, BitPacked* packed);

#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(pop)
#endif
