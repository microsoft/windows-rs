#ifndef VALUE
#define VALUE int
#endif

struct Packet {
    char tag;
    union {
        VALUE number;
        struct {
            short low;
            short high;
        };

        double real;
    };
    int tail;
    union {
        int final_number;
        float final_real;
    };
};

struct NamedMember {
    struct { int value; } named;
};

extern "C" void Use(Packet* packet);
