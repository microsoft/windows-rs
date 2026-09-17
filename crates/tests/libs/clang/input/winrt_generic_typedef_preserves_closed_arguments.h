//! namespace WinrtGeneric
//! library api.dll
//! reference-default

struct HSTRING__;
             typedef HSTRING__* HSTRING;
             struct IInspectable;
             template<typename K, typename V> struct IMapView {};
             typedef IMapView<HSTRING, IInspectable*> MAP;
             extern "C" void GetMap(MAP* value);
