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
#ifndef PARAMETER_TYPE
#define PARAMETER_TYPE int*
#endif
#ifndef EXTRA_PARAMETER
#define EXTRA_PARAMETER
#endif
typedef CALLBACK_RESULT (CALLBACK_ABI* SharedCallback)(INPUT PARAMETER_TYPE PARAMETER EXTRA_PARAMETER);
struct CallbackConsumer {
    SharedCallback callback;
};
struct CallbackIndependent {
    int value;
};
