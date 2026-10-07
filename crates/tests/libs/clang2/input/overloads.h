void Use(int value);
void Use(float value);

struct __declspec(uuid("981a8ac7-36ca-4b94-a186-a201ed0ed053")) IOverloads {
    virtual void __stdcall Use(int value) = 0;
    virtual void __stdcall Use(float value) = 0;
};
