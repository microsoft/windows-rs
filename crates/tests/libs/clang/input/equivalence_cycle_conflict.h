//! input first.h
struct IFoo;
struct Right;
struct Left { Right* next; IFoo* value; };
struct Right { Left* next; IFoo* value; };
//! input second.h
struct IFoo { virtual void Method() = 0; };
struct Right;
struct Left { Left* next; IFoo* value; };
struct Right { Left* next; IFoo* value; };
