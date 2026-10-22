typedef void Volatile(volatile int* value);
typedef void Varargs(int first, ...);
typedef void Method() const;
struct RejectedFunction { Volatile* callback; };
struct Survivor { int value; };
