#ifdef SECOND
#define FIRST zebra
#define NEXT alpha
#define LENGTH size
#else
#define FIRST alpha
#define NEXT zebra
#define LENGTH count
#endif
#define TEXT_(n) #n
#define TEXT(n) TEXT_(n)
#define BUFFER(n) __attribute__((annotate("_In_reads_(" TEXT(n) ")")))
extern "C" void Names(int FIRST, int NEXT, int p1, int, int p3, int type, int self,
    int gen, int box, int priv, int yield);
extern "C" void Buffer(unsigned LENGTH, BUFFER(LENGTH) const char* data);
typedef void (__stdcall *Callback)(int FIRST, int NEXT);
struct __declspec(uuid("6f3577c1-2047-4713-a2c9-9239f4ded029")) IExample {
    virtual void __stdcall Method(int FIRST, int NEXT, int) = 0;
};
