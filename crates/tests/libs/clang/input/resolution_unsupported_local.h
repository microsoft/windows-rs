struct IFoo { IFoo(); virtual void Method() = 0; };
extern "C" void Local(IFoo* value);
