#ifndef BUFFER_CONTRACT
#define BUFFER_CONTRACT
#endif
#ifndef BUFFER_TYPE
#define BUFFER_TYPE Plain
#endif
typedef char* Plain;
#ifndef TERMINATION
#define TERMINATION __attribute__((annotate("windows_clang:sal:2:_Null_terminated_\n()\n()")))
#endif
typedef TERMINATION char* Terminated;
struct Counted {
    unsigned int capacity;
    unsigned int length;
    BUFFER_CONTRACT BUFFER_TYPE buffer;
};
struct ContractConsumer {
    Counted value;
};
struct ContractIndependent {
    int value;
};
