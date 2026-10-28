#pragma once

typedef union _Value {
    int integer;
    float real;
    void* pointer;
    unsigned char bytes[9];
} Value, *ValuePointer, ValueArray[2];

struct Box {
    char lead;
    Value value;
    char tail;
};

union __attribute__((packed)) PackedValue {
    int integer;
    unsigned char bytes[5];
};

#if defined(_M_IX86)
union PlatformValue { int value; };
#else
struct PlatformValue { int value; };
#endif

union Node;
typedef Node* NodePointer;
union Node {
    NodePointer next;
    unsigned long long bits;
};
