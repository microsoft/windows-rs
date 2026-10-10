extern "C" unsigned long long Product(
    unsigned long long left,
    unsigned long long right,
    unsigned long long* __attribute__((annotate("_Out_")))
        __attribute__((annotate("_Deref_out_range_(==,left * right)"))) high);

#define Alias Product
extern "C" unsigned long long Alias(
    unsigned long long multiplier,
    unsigned long long multiplicand,
    unsigned long long* __attribute__((annotate("_Out_"))) high);
