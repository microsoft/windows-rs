#define _Out_ __attribute__((annotate("_Out_")))
#define _Out_writes_bytes_to_(n, c) __attribute__((annotate("_Out_writes_bytes_to_(" #n "," #c ")")))

#ifdef _WIN64
typedef long long SignedSize;
typedef unsigned long long UnsignedSize;
#else
typedef int SignedSize;
typedef unsigned int UnsignedSize;
#endif
typedef unsigned long long FixedSize;
typedef UnsignedSize SizeChain;
typedef UnsignedSize AnnotatedSize __attribute__((annotate("_In_range_(0, 1024)")));

struct Sizes {
    SizeChain count;
    SignedSize offset;
    SizeChain values[2];
    SizeChain* pointer;
    FixedSize fixed;
};

extern "C" SignedSize Measure(SizeChain count, SignedSize offset, SizeChain* result, FixedSize fixed);
extern "C" AnnotatedSize Annotated(AnnotatedSize value);
extern "C" void Write(_Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
    SizeChain capacity, _Out_ SizeChain* written);
const SignedSize MinOffset = -1;
const UnsignedSize MaxCount = (UnsignedSize)-1;
