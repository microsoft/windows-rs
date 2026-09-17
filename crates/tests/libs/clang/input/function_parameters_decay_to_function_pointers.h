//! namespace FunctionParameter
//! library api.dll

#define IN __attribute__((annotate("_In_")))
             extern "C" void UseCallback(void Callback(int value));
             extern "C" void UseInputCallback(IN void InputCallback(int value));
