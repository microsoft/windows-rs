//! namespace OpaqueClass
//! library api.dll

class Implementation { public: virtual ~Implementation() {} int value; };
             extern "C" void UseImplementation(Implementation* value);
