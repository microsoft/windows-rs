#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(push)
#pragma warning(disable: 4201)
#endif

#ifdef __clang__
#define LAYOUT_ANNOTATION(value) __attribute__((annotate(value)))
#else
#define LAYOUT_ANNOTATION(value)
#endif

union LayoutChoice {
    unsigned long number;
    double real;
    unsigned char bytes[8];
};

union __declspec(align(16)) LayoutAligned {
    unsigned char bytes[16];
    unsigned long number;
};

struct LayoutPacket {
    unsigned char tag;
    union LAYOUT_ANNOTATION("_Vendor_(nested)") {
        unsigned long number;
        struct {
            short low;
            short high;
        };
        double real;
    };
    unsigned long Anonymous1;
    LayoutChoice choices[2];
    struct {
        long value;
        unsigned short marker;
    } named;
    LayoutAligned aligned;
};

typedef void (*LayoutCallback)(LayoutPacket* packet);
extern "C" unsigned long LayoutEvidence(unsigned long index);
extern "C" void LayoutMutate(LayoutPacket* packet);
extern "C" void LayoutInvoke(LayoutCallback callback, LayoutPacket* packet);

#undef LAYOUT_ANNOTATION

#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(pop)
#endif
