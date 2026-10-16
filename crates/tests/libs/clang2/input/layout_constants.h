struct Record { char tag; void* value; };
extern int runtime();
extern int mutable_value;
#define END_OF(T, M) \
    ((int)((char*)&((T*)0)->M - (char*)((T*)0)) + sizeof(((T*)0)->M))
#define MEMBER_END END_OF(Record, value)
#define MEMBER_OFFSET ((unsigned long long)&((Record*)0)->value)
#define BUILTIN_OFFSET __builtin_offsetof(Record, value)
#define SIDE_EFFECT (runtime(), 3)
#define MUTABLE_READ mutable_value
#define DIV_ZERO (1 / 0)
#define SIGNED_OVERFLOW (2147483647 + 1)
#define NULL_READ (*(int*)0)
