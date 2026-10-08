extern "C" int Find(const wchar_t* value);
inline int Find(wchar_t* value) { return Find(static_cast<const wchar_t*>(value)); }
inline int Helper() { return 0; }
extern "C" int Missing(int value);
