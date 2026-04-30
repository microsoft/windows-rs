unsigned int A();
void B(unsigned a, signed b);

extern "C"
{
    unsigned int C();
    void D(unsigned a, signed b);
}

void* GetBuffer(int size);
int* GetArray(unsigned int count);
