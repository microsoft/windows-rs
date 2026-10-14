#pragma once

struct Nested {
    char lead;
    union {
        int integer;
        struct { short low; short high; };
    };
    struct { int value; } named;
};
struct Shared {
    struct { int value; } first, second;
};
struct Indirect {
    struct Node { int value; Node* next; };
    Node* head;
    struct { int value; } values[2];
    int (*visitor)(Node*);
};
struct Scoped {
    struct Item;
    Item* value;
};
struct Scoped::Item { int value; };
typedef Scoped::Item ScopedAlias;
struct Other {
    struct Item { unsigned short value; };
    Item value;
};
#pragma pack(push, 1)
struct Packed {
    char lead;
    union { int integer; short halves[2]; };
};
#pragma pack(pop)
struct BitfieldParent {
    unsigned first : 1;
    unsigned second : 1;
    struct { int value; } named;
};
