//! input first.h
struct IFoo;
struct Root;
struct Left { Root* next; IFoo* value; };
struct Root { Left* next; };
//! input second.h
struct IFoo { virtual void Method() = 0; };
struct Root;
struct Right { Root* next; IFoo* value; };
struct Root { Right* next; };
