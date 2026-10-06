struct Native {
    int narrow;
    long wide;
    const volatile int* const* __restrict pointers;
    int& left;
    int&& right;
};
struct Forward;
typedef Forward* Raw;
typedef Raw Alias;
typedef const struct Forward* ConstForward;
extern int incomplete_array[];
extern int zero_array[0];
extern "C" void Use(
    Native* record,
    Alias value,
    ConstForward other,
    int* __attribute__((annotate("_Out_writes_(count)"))) output,
    int count);
