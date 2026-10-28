struct OtherBroken { unsigned& other; };
struct Broken { unsigned& another; };
struct Second { OtherBroken* value; Broken* other; };
struct First { Broken* value; OtherBroken* other; };
struct Both { Second* second; First* first; };
