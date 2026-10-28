#ifndef NO_INIT
#define NO_INIT __declspec(no_init_all)
#endif
#ifndef RECORD_NAME
#define RECORD_NAME Uninitialized
#endif

struct NO_INIT RECORD_NAME {
    unsigned int count;
    void* data;
};

typedef RECORD_NAME PublicInitialization;

struct Initialized {
    unsigned int count;
    void* data;
};

static_assert(sizeof(RECORD_NAME) == sizeof(Initialized));
static_assert(alignof(RECORD_NAME) == alignof(Initialized));
static_assert(__builtin_offsetof(RECORD_NAME, data) == __builtin_offsetof(Initialized, data));
