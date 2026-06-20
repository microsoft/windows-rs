// Struct with various pointer and array fields.
struct Buffer {
    unsigned char* data;
    unsigned int size;
    unsigned int capacity;
};

struct FixedArray {
    int values[16];
    char name[64];
};
