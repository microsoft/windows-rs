// Tests free functions whose return type is a pointer.  fn.h only tests
// void and scalar return types; this exercises the non-void pointer branch
// in Fn::write / write_type for return types.

void* GetBuffer(int size);
int* GetArray(unsigned int count);
