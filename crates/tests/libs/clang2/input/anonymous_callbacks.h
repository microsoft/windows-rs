#define IN __attribute__((annotate("_In_")))
#define READS(n) __attribute__((annotate("_In_reads_(" #n ")")))
#define OUT __attribute__((annotate("_Out_")))
typedef unsigned long COUNT;
struct CallbackFields {
    int (__stdcall *fill)(IN CallbackFields* owner);
    void (*read)(unsigned count, READS(count) const int* data, OUT int* result);
    void (**indirect)(int value);
    COUNT (*named)(COUNT value);
    void (*other)(int value);
    void (*unnamed)(int, int p0);
};

struct CallbackFields_read_Callback { int value; };
struct Collision {
    CallbackFields fields;
    CallbackFields_read_Callback conflict;
};
struct VariadicCallback { void (*invoke)(int value, ...); };
struct FastCallback { void (__fastcall *invoke)(int value); };
struct NestedCallback { void (*invoke)(void (*nested)(int value)); };
