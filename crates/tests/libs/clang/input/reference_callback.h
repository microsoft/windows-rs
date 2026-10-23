#ifndef INPUT
#define INPUT
#endif
#ifndef CALLBACK_ABI
#define CALLBACK_ABI __stdcall
#endif
#ifndef CALLBACK_RESULT
#define CALLBACK_RESULT int
#endif
#ifndef PARAMETER
#define PARAMETER value
#endif
typedef CALLBACK_RESULT (CALLBACK_ABI* SharedCallback)(INPUT int* PARAMETER);
struct CallbackConsumer {
    SharedCallback callback;
};
struct CallbackIndependent {
    int value;
};
