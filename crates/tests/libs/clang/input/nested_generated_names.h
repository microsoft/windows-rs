//! namespace NestedNames
typedef struct ROOT {
    union { int value; } choice;
    struct { int value; } entries[1];
} ROOT;
