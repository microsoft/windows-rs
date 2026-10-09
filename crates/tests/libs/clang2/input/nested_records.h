#define ANNOTATION(value) __attribute__((annotate(value)))
#ifndef MEMBER_TYPE
#define MEMBER_TYPE int
#endif

struct First {
    struct ANNOTATION("_Vendor_(nested)") Shared {
        MEMBER_TYPE value;
        Shared* next;
        void (*visit)(Shared* value);
    };
    union Choice {
        Shared* shared;
        double scalar;
    };
    enum Kind : unsigned int { Empty = 0, Full = 1 };
    typedef Shared Alias;
    Shared owned;
    Shared* borrowed;
    Alias* aliased;
    Choice choice;
    Kind kind;
};

struct Second {
    struct Shared {
        MEMBER_TYPE value;
        Shared* next;
        void (*visit)(Shared* value);
    };
    Shared owned;
    Shared* borrowed;
};

struct Packet {
    First first;
    Second second;
    First::Shared* first_shared;
    Second::Shared* second_shared;
};

struct First_Shared { int collision; };
struct Collision {
    Packet packet;
    First_Shared conflict;
};

extern "C" void Consume(First::Shared* first, Second::Shared* second, First::Alias* alias);
extern "C" void ConsumeUnion(First::Choice choice);

struct mod {
    struct type { int value; };
};
