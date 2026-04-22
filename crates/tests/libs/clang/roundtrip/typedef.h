// Simple typedefs.
typedef long A;
typedef unsigned int B;
typedef void* C;

// Callbacks: function-pointer typedefs (renamed to avoid collision with A/B above).
typedef void (*CallbackA)();
typedef int (*CallbackB)(bool a, float b);

// Old-fashioned C typedef-struct idiom: internal tag name (_TEST) must never
// appear in the generated RDL; only the public alias (TEST) should be used.
typedef struct _TEST
{
    int value;
} TEST, *PTEST;

struct TEST2
{
    TEST value;
    PTEST pointer;
};

// Typedef from an included header: MY_BYTE and MY_WORD are defined here
// (simulating a system header) and referenced by TypedefInStruct below.
typedef unsigned char MY_BYTE;
typedef unsigned short MY_WORD;

struct TypedefInStruct {
    MY_BYTE value;
    MY_WORD count;
};
